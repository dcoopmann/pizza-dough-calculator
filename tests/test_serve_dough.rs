mod common;

use pizza_dough_calculator::pizza_dough::PizzaDough;

#[actix_web::test]
async fn pizza_dough_is_served() {
    let address = common::spawn_app().await;
    let client = reqwest::Client::new();
    let body = PizzaDough::new(2.0, "L".to_string(), "F".to_string());

    let response = client
        .post(&format!("{}/serve-dough", address))
        .json(&body)
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());

    let received_dough = response.json::<PizzaDough>().await.unwrap();
    assert_eq!(
        received_dough, body,
        "Ensure received json matches locally generated pizza dough struct"
    );
}
