// main.rs

use resdb_rust_sdk::ResDB;

mod models;
use models::Transaction;
use models::Block;
// use models::CryptoKeypair;

use std::collections::HashMap;

#[tokio::main]
async fn test_transaction_api() {
    let res_db = ResDB::new();

    // Create an instance of Transaction
    let my_struct = res_db.create_object::<Transaction>();

    // Call the asynchronous function to get all transactions
    match res_db.get_all_transactions::<Transaction>("https://crow.resilientdb.com/v1/transactions").await {
        Ok(transactions) => {
            if let Some(first_transaction) = transactions.first() {
                // Access fields of the first transaction (replace with the desired field)
                println!("{:?}", first_transaction);
            } else {
                println!("No transactions available");
            }
        }
        Err(error) => {
            // Handle the error
            eprintln!("Error: {}", error);
        }
    }

    // Call the asynchronous function to get a transaction by ID
    match res_db.get_transaction_by_id::<Transaction>(
        "https://crow.resilientdb.com/v1/transactions",
        "3af5ac7a231b6c219bc61867fa25e654d956b61f3990cb5e747d9a1b4baf568e",
    )
    .await
    {
        Ok(transaction) => {
            // Access fields of the transaction (replace with the desired field)
            println!("{:?}", transaction);
        }
        Err(error) => {
            // Handle the error
            eprintln!("Error: {}", error);
        }
    }

    // Call the asynchronous function to get transactions by key range
    match res_db
        .get_transaction_by_key_range::<Transaction>("https://crow.resilientdb.com/v1/transactions", "10", "60")
        .await
    {
        Ok(transactions) => {
            // Access fields of the transactions (replace with the desired field)
            for transaction in transactions {
                println!("{:?}", transaction);
            }
        }
        Err(error) => {
            // Handle the error
            eprintln!("Error: {}", error);
        }
    }
}

#[tokio::main]
async fn test_transaction_api_map() {
    let data_map = HashMap::new();
    let res_db = ResDB::new();
    
    match res_db.get_all_transactions_map(
        "https://crow.resilientdb.com/v1/transactions",
        data_map,
    )
    .await
    {
        Ok(map) => {
            // Access fields of the transaction (replace with the desired field)
            println!("{:?}", map[10]);
        }
        Err(error) => {
            // Handle the error
            eprintln!("Error: {}", error);
        }
    }

    let data_map = HashMap::new();
    match res_db.get_transaction_by_id_map(
        "https://crow.resilientdb.com/v1/transactions",
        "3af5ac7a231b6c219bc61867fa25e654d956b61f3990cb5e747d9a1b4baf568e",
        data_map,
    )
    .await
    {
        Ok(map) => {
            // Access fields of the transaction (replace with the desired field)
            println!("{:?}", map);
        }
        Err(error) => {
            // Handle the error
            eprintln!("Error: {}", error);
        }
    }

    let data_map = HashMap::new();
    match res_db.get_transaction_by_key_range_map(
        "https://crow.resilientdb.com/v1/transactions",
        "3af5ac7a231b6c219bc61867fa25e654d956b61f3990cb5e747d9a1b4baf568e",
        "2e11fb1f6d235a49f4f2a85c6c57f2ddc9a650fb0f0a6e88ea469584294c03cb",
        data_map,
    )
    .await
    {
        Ok(map) => {
            // Access fields of the transaction (replace with the desired field)
            println!("{:?}", map);
        }
        Err(error) => {
            // Handle the error
            eprintln!("Error: {}", error);
        }
    }
}


#[tokio::main]
async fn test_blocks_api() {
    let res_db = ResDB::new();

    // Create an instance of Block
    let my_struct = res_db.create_object::<Block>();

    // Call the asynchronous function to get all transactions
    match res_db.get_all_blocks::<Block>("https://crow.resilientdb.com/v1/blocks").await {
        Ok(blocks) => {
            if let Some(first_block) = blocks.first() {
                // Access fields of the first transaction (replace with the desired field)
                println!("{:?}", first_block);
            } else {
                println!("No transactions available");
            }
        }
        Err(error) => {
            // Handle the error
            eprintln!("Error: {}", error);
        }
    }

    // Call the asynchronous function to get a transaction by ID
    match res_db.get_blocks_grouped::<Block>("https://crow.resilientdb.com/v1/blocks", &5)
    .await
    {
        Ok(blocks) => {
            if let Some(first_block) = blocks.first() {
                // Access fields of the first transaction (replace with the desired field)
                println!("{:?}", first_block);
            } else {
                println!("No transactions available");
            }
        }
        Err(error) => {
            // Handle the error
            eprintln!("Error: {}", error);
        }
    }
}

#[tokio::main]
async fn test_blocks_api_map() {
    let res_db = ResDB::new();
    let data_map = HashMap::new();

    // Call the asynchronous function to get blocks
    match res_db
        .get_all_blocks_map("https://crow.resilientdb.com/v1/blocks", data_map)
        .await
    {
        Ok(blocks) => {
            if let Some(first_block) = blocks.first() {
                // Access fields of the first transaction (replace with the desired field)
                println!("{:?}", first_block);
            } else {
                println!("No transactions available");
            }
        }
        Err(error) => {
            // Handle the error
            eprintln!("Error: {}", error);
        }
    }

    let data_map = HashMap::new();
    match res_db
        .get_blocks_by_range_map("https://crow.resilientdb.com/v1/blocks", &1, &10, data_map)
        .await
    {
        Ok(blocks) => {
            if let Some(first_block) = blocks.first() {
                // Access fields of the first transaction (replace with the desired field)
                println!("{:?}", first_block);
            } else {
                println!("No transactions available");
            }
        }
        Err(error) => {
            // Handle the error
            eprintln!("Error: {}", error);
        }
    }

    let data_map = HashMap::new();
    match res_db
        .get_blocks_grouped_map("https://crow.resilientdb.com/v1/blocks", &10, data_map)
        .await
    {
        Ok(blocks) => {
            if let Some(first_block) = blocks.first() {
                // Access fields of the first transaction (replace with the desired field)
                println!("{:?}", first_block);
            } else {
                println!("No transactions available");
            }
        }
        Err(error) => {
            // Handle the error
            eprintln!("Error: {}", error);
        }
    }
}







// fn main(){
//     // test_transaction_api();
//     // test_transaction_api_map();
//     // test_blocks_api();
//     // test_blocks_api_map()
  
//     // Generate key pair
//     let keypair = generate_keypair(None);
//     println!("Private Key: {:?}", keypair.private_key);
//     println!("Public Key: {:?}", keypair.public_key);

//     // Hash data
//     let data_to_hash = "Some data to hash";
//     let hashed_data = hash_data(data_to_hash);
//     println!("Hashed data: {}", hashed_data);
// }

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder()
        .build()?;

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse()?);

    let data = r#"
{
    "query": "mutation { postTransaction(data: {\noperation: \"CREATE\"\namount: 69\nsignerPublicKey: \"8fPAqJvAFAkqGs8GdmDDrkHyR7hHsscVjes39TVVfN54\"\nsignerPrivateKey: \"5R4ER6smR6c6fsWt3unPqP6Rhjepbn82Us7hoSj5ZYCc\"\nrecipientPublicKey: \"ECJksQuF9UWi3DPCYvQqJPjF6BqSbXrnDiXUjdiVvkyH\"\nasset: \"\"\"{\n            \"data\": { \"time\": 444\n            },\n          }\"\"\"\n      }) {\n  id\n  }\n}\n"
}
"#;
    let json: serde_json::Value = serde_json::from_str(&data)?;

    let request = client.request(reqwest::Method::POST, "https://cloud.resilientdb.com/graphql")
        .headers(headers)
        .json(&json);

    let response = request.send().await?;
    let body = response.text().await?;

    println!("{}", body);

    Ok(())
}