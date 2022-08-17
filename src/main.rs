use page_scraper::scrape_cookie;
use secrets::get_secrets;

use crate::vin::VinRepository;

mod database;
mod page_scraper;
mod searching;
mod secrets;
mod vin;

#[tokio::main]
async fn main() -> Result<(), ()> {
    let secrets = get_secrets();
    let cookie = scrape_cookie(&secrets.cookie_source_url, &secrets.cookie_name)
        .await
        .unwrap();
    let repo = VinRepository::new(&secrets.api_root, &cookie, "pl");

    let mut db = database::VinDatabase::new(DATABASE_URL).await;

    Ok(())
}

const DATABASE_URL: &str = "sqlite:./vinbase.sqlite3?mode=rwc";
