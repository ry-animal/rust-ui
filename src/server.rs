use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use leptos::*;
use leptos_axum::{generate_route_list, LeptosRoutes};
use rust_ui::App;
use tower::ServiceBuilder;
use tower_http::services::ServeDir;

#[derive(Clone)]
pub struct AppState {}

async fn server_fn_handler(
    State(_state): State<AppState>,
    req: axum::extract::Request<axum::body::Body>,
) -> impl IntoResponse {
    leptos_axum::handle_server_fns(req).await
}

async fn leptos_routes_handler(
    State(_state): State<AppState>,
    req: axum::extract::Request<axum::body::Body>,
) -> Response {
    let handler = leptos_axum::render_route(
        || view! { <App/> },
        || "".to_string(),
    );
    handler(req).await.into_response()
}

async fn file_and_error_handler() -> Result<Response, (StatusCode, String)> {
    Ok(Html("404 - Page not found".to_string()).into_response())
}

#[tokio::main]
async fn main() {
    let conf = get_configuration(None).await.unwrap();
    let addr = conf.leptos_options.site_addr;
    let routes = generate_route_list(App);

    let app_state = AppState {};

    let app = Router::new()
        .leptos_routes(&routes, {
            let app_state = app_state.clone();
            move || leptos_routes_handler(State(app_state.clone()), _)
        })
        .route("/api/*fn_name", get(server_fn_handler).post(server_fn_handler))
        .layer(ServiceBuilder::new())
        .nest_service("/pkg", ServeDir::new("pkg"))
        .fallback(file_and_error_handler)
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!("listening on http://{}", &addr);
    axum::serve(listener, app).await.unwrap();
}