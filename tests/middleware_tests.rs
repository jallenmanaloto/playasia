#[cfg(test)]
mod token_tests {
    const VALID_TOKEN: &str =
    "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJ1c2VyMTIzIiwiZXhwIjoxODkzNDU2MDAwfQ.CEvP_ma0kab9ugQ8oPCVn08U7G9YTu1sNxzGmvhs3lA";

    const INVALID_TOKEN: &str =
    "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJ1c2VyMTIzIiwiZXhwIjoxODkzNDU2MDAwfQ.CEvP_ma0kab9ugQ8oPCVn08U7G9YTu1sNxzGmvhs3lB";

    mod with_valid_token {
        use crate::token_tests::VALID_TOKEN;
        use playasia::server::create_app;
        use poem::{http::StatusCode, test::TestClient};
        use serial_test::serial;

        #[tokio::test]
        #[serial]
        async fn test_get_without_token() {
            let app = create_app();
            let client = TestClient::new(app);

            let get_response = client
                .get("/items")
                .header("Authorization", VALID_TOKEN)
                .header("Content-Type", "application/json")
                .send()
                .await;
            get_response.assert_status(StatusCode::OK);
        }

        #[tokio::test]
        #[serial]
        async fn test_post_with_valid_token() {
            std::fs::write("data.json", "[]").expect("Failed to reset data file.");

            let app = create_app();
            let client = TestClient::new(app);

            let payload = r#"{"name": "Test Item"}"#;
            let response = client
                .post("/items")
                .body(payload)
                .header("Authorization", VALID_TOKEN)
                .header("Content-Type", "application/json")
                .send()
                .await;
            response.assert_status(StatusCode::CREATED);
        }

        #[tokio::test]
        #[serial]
        async fn test_put_with_valid_token() {
            std::fs::write("data.json", "[]").expect("Failed to reset data file.");

            let app = create_app();
            let client = TestClient::new(app);

            // Create a test data
            let payload = r#"{"name": "Test Item"}"#;
            let response = client
                .post("/items")
                .body(payload)
                .header("Authorization", VALID_TOKEN)
                .header("Content-Type", "application/json")
                .send()
                .await;
            response.assert_status(StatusCode::CREATED);

            // Update test data
            let payload = r#"{"name": "Zero 2 Prod"}"#;
            let response = client
                .put("/items/1")
                .body(payload)
                .header("Authorization", VALID_TOKEN)
                .header("Content-Type", "application/json")
                .send()
                .await;
            response.assert_status(StatusCode::OK);
        }

        #[tokio::test]
        #[serial]
        async fn test_delete_with_valid_token() {
            std::fs::write("data.json", "[]").expect("Failed to reset data file.");

            let app = create_app();
            let client = TestClient::new(app);

            // Create a test data
            let payload = r#"{"name": "Test Item"}"#;
            let response = client
                .post("/items")
                .body(payload)
                .header("Authorization", VALID_TOKEN)
                .header("Content-Type", "application/json")
                .send()
                .await;
            response.assert_status(StatusCode::CREATED);

            // Delete test data
            let payload = r#"{"name": "Zero 2 Prod"}"#;
            let response = client
                .delete("/items/1")
                .body(payload)
                .header("Authorization", VALID_TOKEN)
                .header("Content-Type", "application/json")
                .send()
                .await;
            response.assert_status(StatusCode::OK);
        }
    }

    mod without_token {
        use crate::token_tests::VALID_TOKEN;
        use playasia::server::create_app;
        use poem::{http::StatusCode, test::TestClient};
        use serial_test::serial;

        #[tokio::test]
        #[serial]
        async fn test_get_without_token() {
            let app = create_app();
            let client = TestClient::new(app);

            let get_response = client.get("/items").send().await;
            get_response.assert_status(StatusCode::OK);
        }

        #[tokio::test]
        #[serial]
        async fn test_post_without_token() {
            std::fs::write("data.json", "[]").expect("Failed to reset data file.");

            let app = create_app();
            let client = TestClient::new(app);

            let payload = r#"{"name": "Test Item"}"#;
            let response = client
                .post("/items")
                .body(payload)
                .header("Content-Type", "application/json")
                .send()
                .await;
            response.assert_status(StatusCode::UNAUTHORIZED);
        }

        #[tokio::test]
        #[serial]
        async fn test_put_without_token() {
            std::fs::write("data.json", "[]").expect("Failed to reset data file.");

            let app = create_app();
            let client = TestClient::new(app);

            // Create a test data
            let payload = r#"{"name": "Test Item"}"#;
            let response = client
                .post("/items")
                .body(payload)
                .header("Authorization", VALID_TOKEN)
                .header("Content-Type", "application/json")
                .send()
                .await;
            response.assert_status(StatusCode::CREATED);

            // Update test data
            let payload = r#"{"name": "Zero 2 Prod"}"#;
            let response = client
                .put("/items/1")
                .body(payload)
                .header("Content-Type", "application/json")
                .send()
                .await;
            response.assert_status(StatusCode::UNAUTHORIZED);
        }

        #[tokio::test]
        #[serial]
        async fn test_delete_without_token() {
            std::fs::write("data.json", "[]").expect("Failed to reset data file.");

            let app = create_app();
            let client = TestClient::new(app);

            // Create a test data
            let payload = r#"{"name": "Test Item"}"#;
            let response = client
                .post("/items")
                .body(payload)
                .header("Authorization", VALID_TOKEN)
                .header("Content-Type", "application/json")
                .send()
                .await;
            response.assert_status(StatusCode::CREATED);

            // Delete test data
            let payload = r#"{"name": "Zero 2 Prod"}"#;
            let response = client
                .delete("/items/1")
                .body(payload)
                .header("Content-Type", "application/json")
                .send()
                .await;
            response.assert_status(StatusCode::UNAUTHORIZED);
        }
    }

    mod with_invalid_token {
        use crate::token_tests::{INVALID_TOKEN, VALID_TOKEN};
        use playasia::server::create_app;
        use poem::{http::StatusCode, test::TestClient};
        use serial_test::serial;

        #[tokio::test]
        #[serial]
        async fn test_post_with_invalid_token() {
            std::fs::write("data.json", "[]").expect("Failed to reset data file.");

            let app = create_app();
            let client = TestClient::new(app);

            let payload = r#"{"name": "Test Item"}"#;
            let response = client
                .post("/items")
                .body(payload)
                .header("Authorization", INVALID_TOKEN)
                .header("Content-Type", "application/json")
                .send()
                .await;
            response.assert_status(StatusCode::BAD_REQUEST);
        }

        #[tokio::test]
        #[serial]
        async fn test_put_with_invalid_token() {
            std::fs::write("data.json", "[]").expect("Failed to reset data file.");

            let app = create_app();
            let client = TestClient::new(app);

            // Create a test data
            let payload = r#"{"name": "Test Item"}"#;
            let response = client
                .post("/items")
                .body(payload)
                .header("Authorization", VALID_TOKEN)
                .header("Content-Type", "application/json")
                .send()
                .await;
            response.assert_status(StatusCode::CREATED);

            // Update test data
            let payload = r#"{"name": "Zero 2 Prod"}"#;
            let response = client
                .put("/items/1")
                .body(payload)
                .header("Authorization", INVALID_TOKEN)
                .header("Content-Type", "application/json")
                .send()
                .await;
            response.assert_status(StatusCode::BAD_REQUEST);
        }

        #[tokio::test]
        #[serial]
        async fn test_delete_with_invalid_token() {
            std::fs::write("data.json", "[]").expect("Failed to reset data file.");

            let app = create_app();
            let client = TestClient::new(app);

            // Create a test data
            let payload = r#"{"name": "Test Item"}"#;
            let response = client
                .post("/items")
                .body(payload)
                .header("Authorization", VALID_TOKEN)
                .header("Content-Type", "application/json")
                .send()
                .await;
            response.assert_status(StatusCode::CREATED);

            // Delete test data
            let payload = r#"{"name": "Zero 2 Prod"}"#;
            let response = client
                .delete("/items/1")
                .body(payload)
                .header("Authorization", INVALID_TOKEN)
                .header("Content-Type", "application/json")
                .send()
                .await;
            response.assert_status(StatusCode::BAD_REQUEST);
        }
    }
}

#[cfg(test)]
mod middleware_method {
    use playasia::middleware::JwtMiddleware;
    use serial_test::serial;

    #[tokio::test]
    #[serial]
    async fn test_new_secret() {
        let mw = JwtMiddleware::new("secret-key");
        assert_eq!(mw.secret, "secret-key".to_string());
    }
}
