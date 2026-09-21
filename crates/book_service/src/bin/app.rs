use book_service::{ AppConf, AppState, models::Book, routes };
use tokio::net::TcpListener;
use tracing::info;
use tracing_subscriber::{ EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt };

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| {
        "info,tower_http=error,axum=error,toasty=error,tokio_postgres=error".into()
    });

    tracing_subscriber
        ::registry()
        .with(filter)
        .with(fmt::layer().json().with_current_span(true).with_target(true))
        .init();

    let conf = AppConf::init();

    let db = toasty::Db
        ::builder()
        .models(toasty::models!(Book))
        .connect(&conf.db.into_db_url()).await
        .expect("Failed to connect to database");

    let addr = conf.server.to_addr();
    let server_conf = conf.server;

    let app_state = AppState { db, server_conf };

    let app = routes::init(app_state);
    info!(addr=%addr, "Starting server");

    let listener = TcpListener::bind(addr).await?;

    axum::serve(listener, app).await?;

    Ok(())
}
