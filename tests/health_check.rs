#[cfg(test)]
mod tests {

    use actix_web::{test, App};
    use sqlx::{Connection, PgConnection};
    use z2p::{configuration::get_configuration, startup::routing};

    #[actix_web::test]
    async fn health_check_works() {
        let app = test::init_service(App::new().configure(routing)).await;

        let req = test::TestRequest::get().uri("/health_check").to_request();

        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success());

        let result = test::read_body(resp).await;
        assert!(result.is_empty());
    }

    #[actix_web::test]
    async fn subscriber_ok_valid_data() {
        let app = test::init_service(App::new().configure(routing)).await;

        let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
        let req = test::TestRequest::post()
            .uri("/subscribe")
            .insert_header(("Content-Type", "application/x-www-form-urlencoded"))
            .set_payload(body)
            .to_request();

        let configuration = get_configuration().expect("Failed to read configuration");
        let connection_string = configuration.database.connection_string();

        let mut connection = PgConnection::connect(&connection_string)
            .await
            .expect("Failed to connect to Postgres.");

        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status().as_u16(), 200);

        let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
            .fetch_one(&mut connection)
            .await
            .expect("Failed to fetch saved subscription.");
        assert_eq!(saved.email, "ursula_le_guin@gmail.com");
        assert_eq!(saved.name, "le guin");
    }

    #[actix_web::test]
    async fn subscriber_invlaid_data_code() {
        let app = test::init_service(App::new().configure(routing)).await;

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
