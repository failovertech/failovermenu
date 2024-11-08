mod client;
mod credentials;
mod organizations;

use crate::client::OpenApiClient;
use crate::credentials::Credentials;
use crate::organizations::*;

use serde::{Deserialize, Serialize};
use anyhow::{Result, Context};
use tokio;
use std::env;

use log::{debug, error, info};

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::builder().filter_level(log::LevelFilter::Debug).init();
    
    log::debug!("STARTING");

    let credentials = Credentials::from_file("~/.failovermenu")?;

    println!("Initializing API client...");
    let client = OpenApiClient::new(credentials)
        .context("Failed to create API client")?;
    
    println!("Fetching organizations...");
    let organizations_response = fetch_organizations(&client).await;
    
    let orgs = get_organizations(organizations_response);
    let org = orgs.first().unwrap();

    print_organization(org);

    log::debug!("FINISHED");

    Ok(())
}