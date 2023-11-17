use std::fs::File;
use std::io::Read;
use serde::Deserialize;
use anyhow::Error;

impl Blocks{

    pub async fn get_all_blocks(api_url: &str) -> Result<Vec<Blcoks>, anyhow::Error>{
        // To-Do
    }
    pub async fn get_blocks_by_batch(api_url: &str) -> Result<Vec<Blcoks>, anyhow::Error>{
        // To-Do
    }
    pub async fn get_blocks_by_range(api_url: &str) -> Result<Vec<Blcoks>, anyhow::Error>{
        // To-Do
    }
}