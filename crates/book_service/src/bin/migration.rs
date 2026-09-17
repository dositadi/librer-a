use std::{ thread::sleep, time::Duration };

use book_service::{ DBConf, models::Book };
use toasty::{ Db };
use toasty_cli::{ Config, ToastyCli };

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let db_url = DBConf::init().into_db_url();
    let config = Config::load()?;
    let db = connect_to_db(&db_url).await?;

    let cli = ToastyCli::with_config(db, config);

    cli.parse_and_run().await?;

    Ok(())
}

async fn connect_to_db(db_url: &str) -> anyhow::Result<Db, std::io::Error> {
    for _ in 0..5 {
        match toasty::Db::builder().models(toasty::models!(Book)).connect(db_url).await {
            Ok(db) => {
                return Ok(db);
            }
            Err(_) => {
                sleep(Duration::from_secs(1));
                continue;
            }
        }
    }

    let connection_err = std::io::Error::new(
        std::io::ErrorKind::ConnectionAborted,
        "connection failed"
    );

    Err(connection_err)
}
