use book_service::{ DBConf, models::Book };
use toasty_cli::{ Config, ToastyCli };

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let db_url = DBConf::init().into_db_url();
    let config = Config::load()?;
    let db = toasty::Db::builder().models(toasty::models!(Book)).connect(&db_url).await?;

    let cli = ToastyCli::with_config(db, config);

    cli.parse_and_run().await?;

    Ok(())
}
