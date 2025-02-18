use csv::ReaderBuilder;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{create_dir_all, File};
use std::io::Write;
use tokio::fs;

const TEMP_DIR: &str = "./tmp";
const REMOTE_TOURIST_BEACON_CSV: &str = "https://docs.google.com/spreadsheets/d/1eu30UyjpQrWexAglwD1Ax_GaDz4d7l8KD76kSzX4DEk/gviz/tq?tqx=out:csv&sheet=Beacons&range=A3:ZZ";
const REMOTE_TOURIST_BEACON_IMGS_CSV: &str = "https://docs.google.com/spreadsheets/d/1eu30UyjpQrWexAglwD1Ax_GaDz4d7l8KD76kSzX4DEk/gviz/tq?tqx=out:csv&sheet=ImgLookup&range=A:Z";

const LOCAL_TOURIST_BEACON_CSV: &str = "./tmp/tourist-beacon.csv";
const LOCAL_TOURIST_BEACON_JSON: &str = "./tmp/tourist-beacon.json";
const LOCAL_TOURIST_BEACON_IMGS_CSV: &str = "./tmp/tourist-beacon-images.csv";

#[derive(Debug, Deserialize, Serialize)]
struct TouristBeacon {
    uuid: String,
    number: String,
    site_name: String,
    system: String,
    distance: String,
    beacon_type: String,
    series: String,
    set: String,
    images: Vec<String>,
    captured_at: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct ImageEntry {
    id: String,
    name: String,
    src: String,
}

async fn download_file(url: &str, local_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let response = client.get(url).send().await?;
    let bytes = response.bytes().await?;
    let mut file = File::create(local_path)?;
    file.write_all(&bytes)?;
    Ok(())
}

async fn load_images() -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    let mut rdr = ReaderBuilder::new()
        .has_headers(false)
        .from_path(LOCAL_TOURIST_BEACON_IMGS_CSV)?;
    let mut images = HashMap::new();

    for result in rdr.deserialize() {
        let record: ImageEntry = result?;
        images.insert(record.name.to_uppercase(), record.src);
    }
    Ok(images)
}

async fn generate_beacon_json() -> Result<(), Box<dyn std::error::Error>> {
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_path(LOCAL_TOURIST_BEACON_CSV)?;
    let images = load_images().await?;
    let now = chrono::Utc::now().to_rfc3339();
    let mut beacons = Vec::new();

    for result in rdr.deserialize() {
        let mut record: TouristBeacon = result?;
        record.images = vec![images
            .get(&record.site_name.to_uppercase())
            .cloned()
            .unwrap_or_default()];
        record.captured_at = now.clone();
        beacons.push(record);
    }

    let json_output = serde_json::to_string_pretty(&beacons)?;
    fs::write(LOCAL_TOURIST_BEACON_JSON, json_output).await?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    create_dir_all(TEMP_DIR)?;

    download_file(REMOTE_TOURIST_BEACON_CSV, LOCAL_TOURIST_BEACON_CSV).await?;
    download_file(
        REMOTE_TOURIST_BEACON_IMGS_CSV,
        LOCAL_TOURIST_BEACON_IMGS_CSV,
    )
    .await?;
    generate_beacon_json().await?;

    println!("Files downloaded and JSON generated successfully.");
    Ok(())
}
