use actix_web::{App, test, web};
use backend::api::route;
use sqlx::PgPool;

#[sqlx::test]
async fn register_user_returns_200_for_valid_json(pool: PgPool) {
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(route::routes),
    )
    .await;

    let body = serde_json::json!({
        "email": "new@example.com",
        "password": "securepassword"
    });

    let req = test::TestRequest::post()
        .uri("/api/register_user")
        .set_json(&body)
        .to_request();

    let response = test::call_service(&app, req).await;

    assert!(
        response.status().is_success(),
        "Api failed to return a 200 ok status: Status was: {}",
        response.status()
    );

    let response_body: serde_json::Value = test::read_body_json(response).await;

    assert_eq!(response_body["status"], "success");
    assert_eq!(response_body["message"], "Registration successful");

    let saved_user = sqlx::query!(
        "SELECT email FROM sellers WHERE email = $1",
        "new@example.com"
    )
    .fetch_optional(&pool)
    .await
    .expect("Failed to query database");

    assert!(
        saved_user.is_some(),
        "THe API returned a 200 Ok but the merchant was not saved to the database"
    );
}

#[sqlx::test]
async fn register_user_returns_400_for_invalid_json(pool: PgPool) {
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool))
            .configure(route::routes),
    )
    .await;

    let invalid_body = serde_json::json!({
        "password": "securepassword"
    });

    let req = test::TestRequest::post()
        .uri("/api/register_user")
        .set_json(&invalid_body)
        .to_request();

    let response = test::call_service(&app, req).await;

    assert_eq!(
        response.status().as_u16(),
        400,
        "The API did not fail with a 400 Bad Request when payload was missing and email"
    );
}
