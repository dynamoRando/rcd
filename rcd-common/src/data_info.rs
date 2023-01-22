pub struct DataInfo {
    pub db_name: String,
    pub table_name: String,
    pub row_id: u32,
    pub hash: Option<u64>,
    pub is_deleted: bool,
}
