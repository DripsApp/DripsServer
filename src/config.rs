pub fn get_kv_path() -> &'static str {
    let path = dotenv!("KV_PATH");
    path
}
pub fn get_sqlite_path() -> &'static str {
    let path = dotenv!("SQLITE_URL");
    path
}
pub fn get_sqlite_table_path() -> &'static str {
    let path = dotenv!("SQLITE_TABLE_PATH");
    path
}
