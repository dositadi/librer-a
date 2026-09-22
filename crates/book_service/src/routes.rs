use std::{ str::FromStr, time::Duration };

use axum::{
    Router,
    extract::{ DefaultBodyLimit, Request },
    http::{ HeaderName, HeaderValue, Method, StatusCode },
    response::{ IntoResponse, Response },
    routing::get,
};
use tower::{ ServiceBuilder };
use tower_http::{
    cors::{ CorsLayer },
    request_id::{ MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer },
    trace::TraceLayer,
};
use tracing::{ Span, info, info_span };

use crate::{ AppState, app::book };

const REQUEST_ID_HEADER: &str = "x-request-id";

pub fn init(state: AppState) -> Router {
    //Router::new().route("/livez", get(livez))
    let x_request_header = HeaderName::from_static(REQUEST_ID_HEADER);

    let middleware = ServiceBuilder::new()
        .layer(SetRequestIdLayer::new(x_request_header.clone(), MakeRequestUuid))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|request: &Request| {
                    let request_id = request
                        .headers()
                        .get(REQUEST_ID_HEADER)
                        .and_then(|value| value.to_str().ok())
                        .unwrap_or("Unknown");

                    info_span!(
                        "http-request",
                        method = ?request.method(),
                        uri_path = request.uri().path(),
                        request_id,
                        status_code = tracing::field::Empty,
                        latency = tracing::field::Empty,
                    )
                })
                .on_response(|response: &Response, latency: Duration, span: &Span| {
                    span.record("status_code", tracing::field::display(response.status()));
                    span.record("latency", latency.as_millis());

                    span.in_scope(|| {
                        info!("request completed");
                    });
                })
        )
        .layer(PropagateRequestIdLayer::new(x_request_header))
        .layer(cors_layer(&state));

    Router::new()
        .route("/livez", get(livez))
        .nest("/v1/books", book::router())
        .layer(DefaultBodyLimit::max(state.server_conf.default_body_limits))
        .layer(middleware)
        .with_state(state)
}

fn cors_layer(state: &AppState) -> CorsLayer {
    let methods: Vec<Method> = state.server_conf.allowed_methods
        .split(",")
        .map(|method| Method::from_str(method).expect("invalid method"))
        .collect();

    let origin: Vec<HeaderValue> = state.server_conf.allowed_origins
        .split(",")
        .map(|origin| HeaderValue::from_str(origin).expect("invalid origin"))
        .collect();

    let headers: Vec<HeaderName> = state.server_conf.allowed_headers
        .split(",")
        .map(|header| HeaderName::from_str(header).expect("invalid header"))
        .collect();

    let cors = CorsLayer::new();

    cors.allow_headers(headers).allow_methods(methods).allow_origin(origin)
}

async fn livez() -> impl IntoResponse {
    StatusCode::OK
}
