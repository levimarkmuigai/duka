use actix_session::SessionMiddleware;
use actix_web::{App, cookie::Key, test, web};
use backend::api::{route, session::PgSessionStore};
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

#[sqlx::test]
async fn login_user_returns_200_for_valid_json(pool: PgPool) {
    let store = PgSessionStore::new(pool.clone());
    let key = Key::generate();
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool))
            .wrap(SessionMiddleware::new(store, key))
            .configure(route::routes),
    )
    .await;

    let user = serde_json::json!({
        "email":  "login@test.com",
        "password": "securepassword"
    });

    let req1 = test::TestRequest::post()
        .uri("/api/register_user")
        .set_json(&user)
        .to_request();

    let setup_res = test::call_service(&app, req1).await;
    assert!(setup_res.status().is_success(), "Failed to register user");

    let auth_data = serde_json::json!({
        "email": "login@test.com",
        "password": "securepassword"
    });

    let req = test::TestRequest::post()
        .uri("/api/login")
        .set_json(&auth_data)
        .to_request();

    let response = test::call_service(&app, req).await;

    assert_eq!(
        response.status().as_u16(),
        200,
        "Login failed and API did not return a 200 OK"
    );

    let set_cookie_header = response.headers().get("set-cookie");
    assert!(
        set_cookie_header.is_some(),
        "The API retur 200 OK, but no session cookie is set in the headers!"
    );
}

#[sqlx::test]
async fn login_returns_401_for_invalid_password(pool: PgPool) {
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(route::routes),
    )
    .await;

    let user = serde_json::json!({
        "email": "user@gmail.com",
        "password": "securepassword"
    });

    let request = test::TestRequest::post()
        .uri("/api/register_user")
        .set_json(&user)
        .to_request();

    let setup_res = test::call_service(&app, request).await;
    assert!(setup_res.status().is_success(), "Failed to register user");
    let auth_payload = serde_json::json!({
        "email": "user@gmail.com",
        "password": "invalidpassword"
    });

    let req = test::TestRequest::post()
        .uri("/api/login")
        .set_json(&auth_payload)
        .to_request();

    let response = test::call_service(&app, req).await;

    assert_eq!(
        response.status().as_u16(),
        401,
        "The API did not return a 401 Unauthorized when an invalid password was given."
    );
}

#[sqlx::test]
async fn login_returns_401_for_invalid_email(pool: PgPool) {
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(route::routes),
    )
    .await;

    let user = serde_json::json!({
        "email": "user@gmail.com",
        "password": "securepassword"
    });

    let request = test::TestRequest::post()
        .uri("/api/register_user")
        .set_json(&user)
        .to_request();

    let setup_res = test::call_service(&app, request).await;
    assert!(setup_res.status().is_success(), "Failed to register user");
    let auth_payload = serde_json::json!({
        "email": "invaild@email.com",
        "password": "securepassword"
    });

    let req = test::TestRequest::post()
        .uri("/api/login")
        .set_json(&auth_payload)
        .to_request();

    let response = test::call_service(&app, req).await;

    assert_eq!(
        response.status().as_u16(),
        401,
        "The API did not return a 401 Unauthorized when an invalid email was given."
    );
}

#[sqlx::test]
async fn login_returns_400_for_invalid_json(pool: PgPool) {
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(route::routes),
    )
    .await;

    let user = serde_json::json!({
        "email": "user@gmail.com",
        "password": "securepassword"
    });

    let request = test::TestRequest::post()
        .uri("/api/register_user")
        .set_json(&user)
        .to_request();

    let setup_res = test::call_service(&app, request).await;
    assert!(setup_res.status().is_success(), "Failed to register user");

    let auth_payload = serde_json::json!({
        "email": "invaild@email.com"
    });

    let req = test::TestRequest::post()
        .uri("/api/login")
        .set_json(&auth_payload)
        .to_request();

    let response = test::call_service(&app, req).await;

    assert_eq!(
        response.status().as_u16(),
        400,
        "The API did not return a 400 Bad Reqeuest when an email was missing."
    );
}
