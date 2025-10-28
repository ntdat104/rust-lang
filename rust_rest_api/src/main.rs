mod models;
mod routes;

use axum::{ Router, http::Method, serve };
use std::{ net::SocketAddr, sync::{ Arc, Mutex }, time::Duration };
use tower_http::{
    auth::AddAuthorizationLayer, classify::{ServerErrorsAsFailures, SharedClassifier}, compression::CompressionLayer, cors::{ Any, CorsLayer }, limit::RequestBodyLimitLayer, timeout::TimeoutLayer, trace::{DefaultMakeSpan, DefaultOnFailure, DefaultOnRequest, DefaultOnResponse, TraceLayer}
};
use tracing_subscriber::{ layer::SubscriberExt, util::SubscriberInitExt };
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use tokio::net::TcpListener;

use models::User;
use routes::{ app_router, ApiDoc };

#[tokio::main]
async fn main() {
    // Logging
    tracing_subscriber
        ::registry()
        .with(tracing_subscriber::EnvFilter::new("axum=info,tower_http=info"))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let db: Arc<Mutex<Vec<User>>> = Arc::new(Mutex::new(Vec::new()));

    // ✅ CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers(Any);

    // ✅ Compression
    let compression = CompressionLayer::new();

    // ✅ Giới hạn body (ví dụ: tối đa 1MB)
    let body_limit = RequestBodyLimitLayer::new(1_000_000);

    // ✅ Timeout (ví dụ: 10 giây)
    let timeout = TimeoutLayer::new(Duration::from_secs(10));

    // ✅ Xác thực (Bearer token hoặc Basic Auth)
    // Ví dụ: chỉ chấp nhận Authorization: Bearer mysecrettoken
    let auth = AddAuthorizationLayer::bearer("mysecrettoken");

    // ✅ TraceLayer có classifier tùy chỉnh (chuẩn 0.6)
    let trace = TraceLayer::new(
        SharedClassifier::new(ServerErrorsAsFailures::new())
    )
    .make_span_with(DefaultMakeSpan::default())
    .on_request(DefaultOnRequest::default())
    .on_response(DefaultOnResponse::default())
    .on_failure(DefaultOnFailure::default());

    // ✅ Router chính + Swagger UI
    let app = Router::new()
        .merge(app_router(db.clone()))
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .layer(cors)
        .layer(compression)
        .layer(body_limit)
        .layer(timeout)
        .layer(auth)
        .layer(trace);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("🚀 Server running at http://{}", addr);
    tracing::info!("📘 Swagger docs at http://{}/swagger-ui", addr);

    println!("🚀 Server running at http://{}", addr);
    println!("📘 Swagger docs at http://{}/swagger-ui", addr);

    // Run the server
    let listener = TcpListener::bind(addr).await.unwrap();
    serve(listener, app.into_make_service()).await.unwrap();
}
