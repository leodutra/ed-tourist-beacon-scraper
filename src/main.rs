use csv::{ReaderBuilder, StringRecord};
use futures_util::StreamExt;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{create_dir_all, File};
use std::io::Write;
use tokio::fs::File as AsyncFile;
use tokio::io::{AsyncWriteExt, BufWriter};
use tokio_util::io::StreamReader;

const TEMP_DIR: &str = "./tmp";
const REMOTE_TOURIST_BEACON_XLSX: &str = "https://docs.google.com/spreadsheets/d/1eu30UyjpQrWexAglwD1Ax_GaDz4d7l8KD76kSzX4DEk/export?format=xlsx";
const REMOTE_TOURIST_BEACON_CSV: &str = "https://docs.google.com/spreadsheets/d/1eu30UyjpQrWexAglwD1Ax_GaDz4d7l8KD76kSzX4DEk/gviz/tq?tqx=out:csv&sheet=Beacons&range=A3:ZZ";
const REMOTE_TOURIST_BEACON_IMGS_CSV: &str = "https://docs.google.com/spreadsheets/d/1eu30UyjpQrWexAglwD1Ax_GaDz4d7l8KD76kSzX4DEk/gviz/tq?tqx=out:csv&sheet=ImgLookup&range=A:Z";

const LOCAL_TOURIST_BEACON_XLSX: &str = "./tmp/tourist-beacon.xlsx";
const LOCAL_TOURIST_BEACON_CSV: &str = "./tmp/tourist-beacon.csv";
const LOCAL_TOURIST_BEACON_JSON: &str = "./tmp/tourist-beacon.json";
const LOCAL_TOURIST_BEACON_IMGS_CSV: &str = "./tmp/tourist-beacon-images.csv";
const LOCAL_TOURIST_BEACON_IMGS_JSON: &str = "./tmp/tourist-beacon-images.json";

#[derive(Debug, Serialize, Deserialize)]
struct Beacon {
    uuid: String,
    number: String,
    site_name: String,
    system: String,
    distance: String,
    beacon_type: String,
    series: String,
    set: String,
    images: Vec<Image>,
    captured_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Image {
    id: String,
    name: String,
    src: String,
}

async fn download_file(url: &str, local_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let response = client.get(url).send().await?;
    let stream = futures_util::stream::once(response.bytes())
        .map(|result| result.map_err(std::io::Error::other));

    let file = AsyncFile::create(local_path).await?;
    let mut writer = BufWriter::new(file);

    let mut reader = Box::pin(StreamReader::new(
        stream.map(|result| result.map_err(std::io::Error::other)),
    ));

    tokio::io::copy(&mut reader, &mut writer).await?;
    writer.flush().await?;

    println!("Downloaded: {}", local_path);
    Ok(())
}

async fn load_images() -> Result<HashMap<String, Image>, Box<dyn std::error::Error>> {
    let mut reader = ReaderBuilder::new()
        .has_headers(false)
        .from_path(LOCAL_TOURIST_BEACON_IMGS_CSV)?;

    let mut images = HashMap::new();

    for result in reader.records() {
        let record = result?;
        let image = Image {
            id: record[0].to_string(),
            name: record[1].to_uppercase(),
            src: record[2].to_string(),
        };
        images.insert(image.name.clone(), image);
    }

    let json = serde_json::to_string_pretty(&images.values().collect::<Vec<_>>())?;
    let mut file = File::create(LOCAL_TOURIST_BEACON_IMGS_JSON)?;
    file.write_all(json.as_bytes())?;

    println!("Images JSON saved.");
    Ok(images)
}

async fn generate_beacon_json(
    images: HashMap<String, Image>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .from_path(LOCAL_TOURIST_BEACON_CSV)?;
    let mut beacons = Vec::new();
    let now = chrono::Utc::now().to_rfc3339();

    for result in reader.records() {
        let record = result?;
        if !record[0].is_empty() {
            let beacon = Beacon {
                uuid: record[0].to_string(),
                number: record[1].to_string(),
                site_name: record[2].to_string(),
                system: record[3].to_string(),
                distance: record[4].to_string(),
                beacon_type: record[5].to_string(),
                series: record[6].to_string(),
                set: record[7].to_string(),
                images: resolve_images(&record, &images),
                captured_at: now.clone(),
            };
            beacons.push(beacon);
        }
    }

    let json = serde_json::to_string_pretty(&beacons)?;
    let mut file = File::create(LOCAL_TOURIST_BEACON_JSON)?;
    file.write_all(json.as_bytes())?;

    println!("Beacons JSON saved.");
    Ok(())
}

fn resolve_images(record: &StringRecord, images: &HashMap<String, Image>) -> Vec<Image> {
    let image_fields = vec![8, 9, 10, 11, 12]; // Colunas das imagens
    let mut image_list = Vec::new();

    for &index in &image_fields {
        if let Some(name) = record.get(index) {
            if let Some(image) = images.get(&name.to_uppercase()) {
                image_list.push(image.clone());
            }
        }
    }
    image_list
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    create_dir_all(TEMP_DIR)?;

    let downloads = vec![
        download_file(REMOTE_TOURIST_BEACON_XLSX, LOCAL_TOURIST_BEACON_XLSX),
        download_file(REMOTE_TOURIST_BEACON_CSV, LOCAL_TOURIST_BEACON_CSV),
        download_file(
            REMOTE_TOURIST_BEACON_IMGS_CSV,
            LOCAL_TOURIST_BEACON_IMGS_CSV,
        ),
    ];

    futures_util::future::join_all(downloads).await;

    let images = load_images().await?;
    generate_beacon_json(images).await?;

    Ok(())
}
