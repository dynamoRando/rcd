#[allow(dead_code)]
#[derive(Debug)]
pub struct Data {
    pub data_string: String,
    pub data_byte: Vec<u8>,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Value {
    pub data: Option<Data>,
    pub col: Column,
}

impl Value {
    #[allow(dead_code)]
    pub fn is_null(&self) -> bool {
        return self.data.is_none();
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Row {
    pub vals: Vec<Value>,
}

impl Row {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self { vals: Vec::new() }
    }

    #[allow(dead_code)]
    pub fn add_value(&mut self, value: Value) {
        self.vals.push(value);
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct Column {
    pub name: String,
    pub is_nullable: bool,
    pub idx: usize,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Table {
    pub num_cols: usize,
    pub name: String,
    pub cols: Vec<Column>,
    pub rows: Vec<Row>,
}

impl Table {
    pub fn new() -> Self {
        Self {
            num_cols: 0,
            name: String::from(""),
            cols: Vec::new(),
            rows: Vec::new(),
        }
    }

    #[allow(dead_code)]
    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    #[allow(dead_code)]
    pub fn set_num_cols(&mut self, total_cols: usize) {
        self.num_cols = total_cols;
    }

    #[allow(dead_code)]
    pub fn num_cols(&self) -> usize {
        return self.num_cols;
    }

    #[allow(dead_code)]
    pub fn add_column(&mut self, column: Column) {
        self.cols.push(column);
    }

    #[allow(dead_code)]
    pub fn add_row(&mut self, row: Row) {
        self.rows.push(row);
    }

    #[allow(dead_code)]
    pub fn get_column_by_index(&self, idx: usize) -> Option<Column> {
        for col in &self.cols {
            if col.idx == idx {
                return Some(col.clone());
            }
        }
        return None;
    }

    #[allow(dead_code)]
    pub fn debug(&self) {
        for row in &self.rows {
            for val in &row.vals {
                print!(
                    "Col: {} Value {} ",
                    val.col.name,
                    &val.data.as_ref().unwrap().data_string
                );
            }
            println!();
        }
    }

    #[allow(dead_code, unused_variables)]
    pub fn to_cdata_rows(&self) -> Vec<crate::cdata::Row> {
        let result: Vec<crate::cdata::Row> = Vec::new();
        let idx = 0;

        for t_row in &self.rows {
            let c_values: Vec<crate::cdata::RowValue> = Vec::new();

            for t_val in &t_row.vals {
                
            }

            let c_remote_data: crate::cdata::RowRemoteMetadata = crate::cdata::RowRemoteMetadata {
                is_hash_out_of_sync_with_host: false,
                is_local_deleted: false,
                is_remote_deleted: false,
                is_remote_out_of_sync_with_host: false,
            };

            let c_row = crate::cdata::Row {
                row_id: idx,
                table_id: 0,
                database_id: String::from(""),
                values: c_values,
                is_remoteable: false,
                remote_metadata: Some(c_remote_data),
            };
        }

        unimplemented!("to_cdata_rows not implemented");
    }
}
