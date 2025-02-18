# 🚀 Tourist Beacon Scraper

This Rust project downloads, processes, and converts **Tourist Beacon** data from a Google Spreadsheet into JSON format. It handles CSV and XLSX files, resolving image references and formatting the output for easy usage.  

## 📌 Features  
✅ **Downloads CSV and XLSX files** from a remote Google Spreadsheet.  
✅ **Converts CSV to JSON**, preserving relevant fields and formatting data.  
✅ **Processes image references**, matching them to their respective beacon entries.  
✅ **Asynchronous file handling** for optimized performance.  
✅ **Efficient data parsing** using `csv` and `serde_json`.  

## 📂 Project Structure  

```
📦 tourist-beacon-processor
 ┣ 📂 tmp/                  # Temporary storage for downloaded files
 ┣ 📜 Cargo.toml            # Rust dependencies and package metadata
 ┣ 📜 main.rs               # Main application logic
 ┗ 📜 README.md             # This documentation
```

## ⚡ Getting Started  

### 1️⃣ Prerequisites  
Ensure you have **Rust** and **Cargo** installed:  
```sh
rustc --version  # Ensure Rust is installed
cargo --version  # Ensure Cargo is installed
```

### 2️⃣ Install Dependencies  
Run the following command to install missing dependencies:  
```sh
cargo build
```

### 3️⃣ Run the Processor  
Run the program to fetch and process the data:  
```sh
cargo run
```

This will:  
✔ Create a `tmp/` directory.  
✔ Download **CSV** and **XLSX** files from Google Sheets.  
✔ Convert **CSV** data into **JSON**.  
✔ Store the processed data in `tmp/tourist-beacon.json`.  

## 🛠 Configuration  
This project processes data from **Google Sheets**, using predefined URLs. Modify the URLs inside `main.rs` if needed:  

```rust
const REMOTE_TOURIST_BEACON_CSV: &str = "https://docs.google.com/spreadsheets/d/...";
const REMOTE_TOURIST_BEACON_IMGS_CSV: &str = "https://docs.google.com/spreadsheets/d/...";
```

## 📜 Output Format  
The generated JSON file (`tmp/tourist-beacon.json`) has the following structure:

```json
[
  {
    "uuid": "123456",
    "number": "001",
    "site_name": "Galactic Beacon Alpha",
    "system": "Sol",
    "distance": "0 LY",
    "beacon_type": "Historical",
    "series": "Alpha Series",
    "set": "Primary",
    "images": [
      {
        "id": "img001",
        "name": "Beacon Alpha Image",
        "src": "https://example.com/image1.jpg"
      }
    ],
    "captured_at": "2025-02-18T12:00:00Z"
  }
]
```

## 🏗 Dependencies  
- **[Tokio](https://crates.io/crates/tokio)** – Asynchronous runtime for Rust.  
- **[Reqwest](https://crates.io/crates/reqwest)** – HTTP client for downloading data.  
- **[CSV](https://crates.io/crates/csv)** – CSV parsing library.  
- **[Serde](https://crates.io/crates/serde)** – JSON serialization/deserialization.  

Install dependencies manually if needed:  
```sh
cargo add tokio reqwest csv serde serde_json tokio-util futures-util
```

## 🚀 Future Enhancements  
🔹 Support for XLSX parsing.  
🔹 Better error handling and logging.  
🔹 CLI arguments for customizing URLs and output directories.  

## 📜 License  
This project is licensed under the **MIT License**.  

---

This **README** provides an overview, setup instructions, and usage details. Let me know if you need any changes! 🚀
