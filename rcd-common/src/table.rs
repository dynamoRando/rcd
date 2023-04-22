use guid_create::GUID;
use stdext::function_name;
use tracing::trace;
use rcd_enum::column_type::ColumnType;
use substring::Substring;

#[derive(Debug, Clone)]
pub struct Data {
    pub data_string: String,
    pub data_byte: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct Value {
    pub data: Option<Data>,
    pub col: Column,
}

impl Value {
    pub fn is_null(&self) -> bool {
        self.data.is_none()
    }
}

#[derive(Debug)]
pub struct Row {
    pub vals: Vec<Value>,
}

impl Default for Row {
    fn default() -> Self {
        Self::new()
    }
}

impl Row {
    pub fn new() -> Self {
        Self { vals: Vec::new() }
    }

    pub fn add_value(&mut self, value: Value) {
        self.vals.push(value);
    }
}

#[derive(Clone, Debug)]
pub struct Column {
    pub name: String,
    pub is_nullable: bool,
    pub idx: usize,
    pub data_type: String,
    pub is_primary_key: bool,
}

impl Column {
    pub fn data_type_to_enum_u32(&self) -> u32 {
        let ct = ColumnType::try_parse(&self.data_type).unwrap();
        ColumnType::to_u32(ct)
    }

    pub fn data_type_len(&self) -> u32 {
        let str_data_type: String = self.data_type.clone();

        trace!("[{}]: {str_data_type:?}", function_name!());

        let idx_first_paren = str_data_type.find('(');

        if idx_first_paren.is_none() {
            0
        } else {
            let idx_first = idx_first_paren.unwrap() + 1;
            let idx_last = str_data_type.find(')').unwrap();
            let str_length = str_data_type.substring(idx_first, idx_last).trim();

            trace!("[{}]: {str_length:?}", function_name!());

            let length: u32 = str_length.parse().unwrap();
            length
        }
    }
}

#[derive(Debug)]
pub struct Table {
    pub num_cols: usize,
    pub name: String,
    pub cols: Vec<Column>,
    pub rows: Vec<Row>,
}

impl Default for Table {
    fn default() -> Self {
        Self::new()
    }
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

    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    pub fn set_num_cols(&mut self, total_cols: usize) {
        self.num_cols = total_cols;
    }

    pub fn num_cols(&self) -> usize {
        self.num_cols
    }

    pub fn add_column(&mut self, column: Column) {
        self.cols.push(column);
    }

    pub fn add_row(&mut self, row: Row) {
        self.rows.push(row);
    }

    pub fn get_column_by_index(&self, idx: usize) -> Option<Column> {
        for col in &self.cols {
            if col.idx == idx {
                return Some(col.clone());
            }
        }
        None
    }

    pub fn debug(&self) {
        for row in &self.rows {
            for val in &row.vals {
                trace!(
                    "Col: {} Value {} ",
                    val.col.name,
                    &val.data.as_ref().unwrap().data_string
                );
            }
        }
    }

    pub fn to_cdata_rows(&self) -> Vec<rcdproto::rcdp::Row> {
        let mut result: Vec<rcdproto::rcdp::Row> = Vec::new();

        for (idx, t_row) in self.rows.iter().enumerate() {
            let mut c_values: Vec<rcdproto::rcdp::RowValue> = Vec::new();

            for t_val in &t_row.vals {
                let t_col_item = &t_val.col;

                let c_col_schema_item = rcdproto::rcdp::ColumnSchema {
                    column_name: t_col_item.name.to_string(),
                    column_type: t_col_item.data_type_to_enum_u32(),
                    column_length: t_col_item.data_type_len(),
                    column_id: GUID::rand().to_string(),
                    is_nullable: t_col_item.is_nullable,
                    ordinal: t_col_item.idx as u32,
                    table_id: GUID::rand().to_string(),
                    is_primary_key: t_col_item.is_primary_key,
                };

                let mut c_bin_data = &t_val.data.as_ref().unwrap().data_byte;
                let c_str_data = &t_val.data.as_ref().unwrap().data_string;
                let c_str_bin_data = c_str_data.as_bytes().to_vec();

                if c_bin_data.is_empty() {
                    c_bin_data = &c_str_bin_data;
                }

                let c_bd = c_bin_data.to_vec();

                let c_val: rcdproto::rcdp::RowValue = rcdproto::rcdp::RowValue {
                    column: Some(c_col_schema_item),
                    is_null_value: c_bd.is_empty(),
                    value: c_bd,
                    string_value: c_str_data.clone(),
                };

                c_values.push(c_val);
            }

            let c_remote_data: rcdproto::rcdp::RowRemoteMetadata =
                rcdproto::rcdp::RowRemoteMetadata {
                    is_hash_out_of_sync_with_host: false,
                    is_local_deleted: false,
                    is_remote_deleted: false,
                    is_remote_out_of_sync_with_host: false,
                };

            let c_row = rcdproto::rcdp::Row {
                row_id: idx as u32,
                table_name: self.name.clone(),
                database_name: String::from(""),
                values: c_values,
                is_remoteable: false,
                remote_metadata: Some(c_remote_data),
                hash: Vec::new(),
            };

            result.push(c_row);
        }

        result
    }
}
