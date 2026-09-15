use book_service::routes;
use tokio::net::TcpListener;
use tracing::info;
use tracing_subscriber::{ EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt };

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| {
        "info, tower_http=error, axum=error, toasty=error, tokio_postgres=error".into()
    });

    tracing_subscriber
        ::registry()
        .with(filter)
        .with(fmt::layer().json().with_current_span(false).with_target(false))
        .init();

    let addr = String::from("0.0.0.0:3000");
    info!(addr=%addr, "Starting server");

    let listener = TcpListener::bind(addr).await?;

    let app = routes::init();

    axum::serve(listener, app).await?;

    Ok(())
}
