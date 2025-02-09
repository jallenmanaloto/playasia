use playasia::server::create_app;
use poem::{http::StatusCode, test::TestClient};
use serde_json::json;
use serial_test::serial;

const VALID_TOKEN: &str =
    "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJ1c2VyMTIzIiwiZXhwIjoxODkzNDU2MDAwfQ.CEvP_ma0kab9ugQ8oPCVn08U7G9YTu1sNxzGmvhs3lA";

#[tokio::test]
async fn test_health_check() {
    let app = create_app();
    let client = TestClient::new(app);

    let get_response = client.get("/health").send().await;
    get_response.assert_status(StatusCode::OK);
}

#[tokio::test]
#[serial]
async fn test_get_all_items() {
    std::fs::write("data.json", "[]").expect("Failed to reset data file.");

    let app = create_app();
    let client = TestClient::new(app);

    // Creating test data
    let payload1 = r#"{"name": "Test Item 1"}"#;
    let payload2 = r#"{"name": "Test Item 2"}"#;

    // Create the first item.
    let create_resp1 = client
        .post("/items")
        .body(payload1)
        .header("Authorization", VALID_TOKEN)
        .header("Content-Type", "application/json")
        .send()
        .await;
    create_resp1.assert_status(StatusCode::CREATED);

    // Create the second item.
    let create_resp2 = client
        .post("/items")
        .body(payload2)
        .header("Authorization", VALID_TOKEN)
        .header("Content-Type", "application/json")
        .send()
        .await;
    create_resp2.assert_status(StatusCode::CREATED);

    // GET request to retrieve all items
    let get_response = client.get("/items").send().await;
    get_response.assert_status(StatusCode::OK);

    let expected_data = json!([
        {"id": 1, "name": "Test Item 1"},
        {"id": 2, "name": "Test Item 2"},
    ]);
    get_response.assert_json(expected_data).await;
}

#[tokio::test]
#[serial]
async fn test_get_item() {
    std::fs::write("data.json", "[]").expect("Failed to reset data file.");

    let app = create_app();
    let client = TestClient::new(app);

    // Creating test data
    let payload1 = r#"{"name": "Test Item 1"}"#;
    let payload2 = r#"{"name": "Test Item 2"}"#;

    // Create the first item.
    let create_resp1 = client
        .post("/items")
        .body(payload1)
        .header("Authorization", VALID_TOKEN)
        .header("Content-Type", "application/json")
        .send()
        .await;
    create_resp1.assert_status(StatusCode::CREATED);

    // Create the second item.
    let create_resp2 = client
        .post("/items")
        .body(payload2)
        .header("Authorization", VALID_TOKEN)
        .header("Content-Type", "application/json")
        .send()
        .await;
    create_resp2.assert_status(StatusCode::CREATED);

    // GET request to retrieve all items
    let get_response = client.get("/items/2").send().await;
    get_response.assert_status(StatusCode::OK);

    let expected_data = json!({"id": 2, "name": "Test Item 2"});
    get_response.assert_json(expected_data).await;
}

#[tokio::test]
#[serial]
async fn test_create_item() {
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
    response
        .assert_json(json!({
            "id": 1,
            "name": "Test Item"
        }))
        .await;
}

#[tokio::test]
#[serial]
async fn test_edit_item() {
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

    // Update the test data's name
    let payload = r#"{"name": "Zero 2 Prod"}"#;
    let response = client
        .put("/items/1")
        .body(payload)
        .header("Authorization", VALID_TOKEN)
        .header("Content-Type", "application/json")
        .send()
        .await;
    response.assert_status(StatusCode::OK);
    response
        .assert_json(json!({
            "id": 1,
            "name": "Zero 2 Prod"
        }))
        .await;
}

#[tokio::test]
#[serial]
async fn test_delete_item() {
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

    // Delete the test data's name
    let payload = r#"{"name": "Zero 2 Prod"}"#;
    let response = client
        .delete("/items/1")
        .body(payload)
        .header("Authorization", VALID_TOKEN)
        .header("Content-Type", "application/json")
        .send()
        .await;
    response.assert_status(StatusCode::OK);
    response
        .assert_json(json!({
            "message": "Item deleted successfully"
        }))
        .await;
}
