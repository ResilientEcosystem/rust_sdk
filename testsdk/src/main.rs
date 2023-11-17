// This is the main file for a project that uses the library.
use resdb_rust_sdk::ResDB;

fn main() {
    // Specify the URL of the JSON API endpoint
    let api_url = "https://crow.resilientdb.com/v1/transactions";
    let res_db = ResDB::new(database_url);
    res_db.get_all_transactions();

}
