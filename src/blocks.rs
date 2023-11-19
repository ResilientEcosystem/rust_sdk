// // write rust code to save the Json response from the blocks endpoint into rust objects
// use std::fs::File;
// use std::io::Read;
// use serde::Deserialize;
// use anyhow::Error;

// #[derive(Debug, Deserialize)]

// pub struct Blocks{

// }

// pub struct Transaction {
//     pub cmd: String,
//     pub key: String,
// }

// pub struct Blocks {
//     pub id: i32,
//     pub number: String,
//     pub transactions: Vec<Transaction>,
//     pub size:i32,
//     pub createdAt: String,
// }

// impl Blocks {
//     pub fn new() -> Self {

//         Blocks {
//             id: i32:: new(),
//             number: String::new(),
//             transactions: Vec::new(),
//             size: i32::new(),
//             createdAt: String::new(),
//         }
    
//     }
// }
 
// //  implementation of GEt v1/blocks
// pub async fn get_all_blocks(api_url: &str) -> Result<Vec<Blocks>, anyhow::Error> {
//     let response_text = reqwest::get(api_url).await?;

//     let mut file = File::open("Users/mac/Desktop/ecs189f/resdb_rust_sdk/json_data/blocks.json").expect("Unable to open the file");
//     // Read the file content into a string
//     let mut content = String::new();
//         file.read_to_string(&mut content).expect("Unable to read the file");

//         let json_array: Vec<serde_json::Value> = serde_json::from_str(&content)?;
//         let mut blocks = Vec::new();
    
//     for json_obj in json_array{
//         let block: Blocks = serde_json::from_value(json_obj)?;
//         blocks.push(block);
//     }
    
//     // let response: Result<Transaction, serde_json::Error> = serde_json::from_str(&content);
//     let response: Result<Vec<Blocks>, serde_json::Error> = Ok(blocks);

//     // Handle the deserialization result
//     match response {
//         Ok(parsed_response) => Ok(parsed_response),
//         Err(err) => Err(anyhow::Error::from(err)),
//     }
// }


