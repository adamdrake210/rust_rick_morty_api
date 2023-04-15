use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::io::{BufWriter, Write};

#[derive(Debug, Deserialize)]
struct CharacterInfo {
  count: u32,
  pages: u32,
  next: String,
  prev: Option<String>
}

#[derive(Debug, Deserialize)]
struct CharacterResults {
  id: u32,
  name: String,
  status: String,
  species: String,
  gender: String,
  image: String,
}

#[derive(Debug, Deserialize)]
struct ApiData {
    // Define the fields of the struct that correspond to the JSON data returned by the API
    info: CharacterInfo,
    results: Vec<CharacterResults>
}




fn main() -> Result<(), Box<dyn Error>> {
    // Initialize the HTTP client
    let client = reqwest::blocking::Client::new();

    // Send an HTTP request to the API and deserialize the response into a Vec<ApiData>
    let response = client.get("https://rickandmortyapi.com/api/character").send()?;
    let api_data: ApiData = response.json()?;
    // Open a new file for writing the CSV data
    let file = File::create("data.csv")?;
    let mut writer = csv::Writer::from_writer(BufWriter::new(file));

    // Write the CSV header row
    writer.write_record(&["Name", "Status", "Species", "gender", "Image"])?;

    // Write each row of data to the CSV file
    for data in api_data.results {
        println!("Character Species: {}", data.species);
        writer.serialize((data.name, data.status, data.species, data.gender, data.image))?;
    }

    Ok(())
}