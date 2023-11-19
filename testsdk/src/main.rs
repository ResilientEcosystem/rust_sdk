// This is the main file for a project that uses the library.
// extern crate resdb_rust_sdk;
// use tokio::main;

// #[tokio::main]
// async fn main() {
//     // Specify the URL of the JSON API endpoint
//     let api_url = "https://crow.resilientdb.com/v1/transactions";

//     match resdb_rust_sdk::get_data_from_api(api_url).await {
//         Ok(response) => {
//             println!("Response: {:?}", response);
//             // Handle the response data as needed
//         }
//         Err(err) => {
//             eprintln!("Error: {:?}", err);
//             // Handle the error
//         }
//     }
// }
use std::fs;
use serde_json::Value; // Assuming you're using serde to handle JSON

fn main() {
    // Read the contents of the local JSON file
    let file_contents = match fs::read_to_string("./transactions.json") {
        Ok(contents) => contents,
        Err(err) => {
            eprintln!("Error reading file: {:?}", err);
            return;
        }
    };

    // Parse the JSON data from the file
    let parsed_data: Result<Value, _> = serde_json::from_str(&file_contents);

    match parsed_data {
        Ok(json_data) => {
            println!("Parsed data: {:?}", json_data);
            // Handle the parsed data as needed
        }
        Err(err) => {
            eprintln!("Error parsing JSON: {:?}", err);
            // Handle the error
        }
    }
}
