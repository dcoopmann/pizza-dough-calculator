mod common;

#[actix_web::test]
async fn health_check_works() {
    let address = common::spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health-check", address))
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
