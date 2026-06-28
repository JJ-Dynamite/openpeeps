use axum::{
    routing::get,
    Router, Json, response::IntoResponse,
};
use tower_http::cors::{CorsLayer, Any};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct HealthResponse { status: String, service: String, version: String }
#[derive(Serialize)]
struct RootResponse { service: String, version: String, description: String, endpoints: Vec<String> }
#[derive(Serialize)]
struct Illustration { id: String, name: String, category: String, svg_url: String, options: Vec<String> }
#[derive(Serialize)]
struct IllustrationListResponse { illustrations: Vec<Illustration>, total: usize }

async fn health() -> impl IntoResponse {
    Json(HealthResponse { status: "healthy".into(), service: "openpeeps".into(), version: "0.1.0".into() })
}

async fn root() -> impl IntoResponse {
    Json(RootResponse {
        service: "openpeeps".into(), version: "0.1.0".into(),
        description: "Free hand-drawn illustrations".into(),
        endpoints: vec!["GET /health".into(), "GET /illustrations".into(), "GET /illustrations/:id".into()],
    })
}

async fn list_illustrations() -> impl IntoResponse {
    let illustrations = vec![
        Illustration { id: "1".into(), name: "Person Working".into(), category: "business".into(), svg_url: "/illust/1.svg".into(), options: vec!["skin_tone".into(), "hair_color".into(), "clothing".into()] },
        Illustration { id: "2".into(), name: "Person Thinking".into(), category: "people".into(), svg_url: "/illust/2.svg".into(), options: vec!["skin_tone".into(), "hair_style".into()] },
    ];
    Json(IllustrationListResponse { illustrations, total: illustrations.len() })
}

async fn get_illustration(axum::extract::Path(id): axum::extract::Path<String>) -> impl IntoResponse {
    Json(Illustration {
        id: id.clone(), name: "Illustration".into(), category: "people".into(),
        svg_url: format!("/illust/{}.svg", id),
        options: vec!["skin_tone".into(), "hair_color".into()],
    })
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let cors = CorsLayer::new().allow_origin(Any).allow_methods(Any).allow_headers(Any);
    let app = Router::new()
        .route("/", get(root)).route("/health", get(health))
        .route("/illustrations", get(list_illustrations)).route("/illustrations/:id", get(get_illustration))
        .layer(cors);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    tracing::info!("openpeeps backend running on port 3001");
    axum::serve(listener, app).await.unwrap();
}
