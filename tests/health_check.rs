#[cfg(test)]
mod tests {

    use actix_web::{middleware::Logger, test, web, App};
    use once_cell::sync::Lazy;
    use sqlx::{Connection, Executor, PgConnection, PgPool};
    use uuid::Uuid;
    use z2p::{configuration::get_configuration, logging::*, startup::routing};

    // Ensure that the `tracing` stack is only initialised once using `once_cell`
    static TRACING: Lazy<()> = Lazy::new(|| {
        let default_filter_level = "info".to_string();
        let subscriber_name = "test".to_string();

        if std::env::var("TEST_LOG").is_ok() {
            let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::stdout);
            init_subscriber(subscriber);
        } else {
            let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::sink);
            init_subscriber(subscriber);
        }
    });

    /// Macro rule for app initialization to reduce boilerplate
    macro_rules! app_init {
        () => {{
            Lazy::force(&TRACING);

            let pool = init_db().await;

            let app = test::init_service(
                App::new()
                    .wrap(Logger::default())
                    .configure(routing)
                    .app_data(web::Data::new(pool.clone())),
            )
            .await;

            (app, pool)
        }};
    }

    /// Database initialization function for tests
    /// Creates database with random name to prevent collisions
    async fn init_db() -> PgPool {
        let mut configuration = get_configuration().expect("Failed to read configuration.");
        // Generate random name for db
        configuration.database.database_name = Uuid::new_v4().to_string();

        // Create database
        let mut connection = PgConnection::connect_with(
            &configuration
                .database
                .without_db()
        )
        .await
        .expect("Failed to connect to Postgres");

        connection
            .execute(
                format!(
                    r#"CREATE DATABASE "{}";"#,
                    configuration.database.database_name
                )
                .as_str(),
            )
            .await
            .expect("Failed to create database.");

        // Migrate database
        let connection_pool =
            PgPool::connect_with(configuration.database.with_db())
                .await
                .expect("Failed to connect to Postgres.");

        sqlx::migrate!("./migrations")
            .run(&connection_pool)
            .await
            .expect("Failed to migrate the database");

        connection_pool
    }

    #[actix_web::test]
    async fn health_check_works() {
        let (app, _) = app_init!();

        let req = test::TestRequest::get().uri("/health_check").to_request();

        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success());

        let result = test::read_body(resp).await;
        assert!(result.is_empty());
    }

    #[actix_web::test]
    async fn subscriber_ok_valid_data() {
        let (app, pool) = app_init!();

        let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
        let req = test::TestRequest::post()
            .uri("/subscribe")
            .insert_header(("Content-Type", "application/x-www-form-urlencoded"))
            .set_payload(body)
            .to_request();

        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status().as_u16(), 200);

        let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
            .fetch_one(&pool)
            .await
            .expect("Failed to fetch saved subscription.");
        assert_eq!(saved.email, "ursula_le_guin@gmail.com");
        assert_eq!(saved.name, "le guin");
    }

    #[actix_web::test]
    async fn subscriber_invlaid_data_code() {
        let pool = init_db().await;
        let app = test::init_service(
            App::new()
                .configure(routing)
                .app_data(web::Data::new(pool.clone())),
        )
        .await;

        let bodys = vec![
            ("name=le%20guin", "Empty mail"),
            ("email=ursula_le_guin%40gmail.com", "Empty name"),
            ("", "Empty Req"),
        ];

        for (body, error) in bodys {
            let req = test::TestRequest::post()
                .uri("/subscribe")
                .insert_header(("Content-Type", "application/x-www-form-urlencoded"))
                .set_payload(body)
                .to_request();

            let resp = test::call_service(&app, req).await;

            assert_eq!(
                resp.status().as_u16(),
                400,
                "Return code must be 400: {error}"
            );
        }
    }
}
