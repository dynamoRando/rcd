use crate::{ rcd_enum::ColumnType};
use guid_create::GUID;
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
        return self.data.is_none();
    }
}

#[derive(Debug)]
pub struct Row {
    pub vals: Vec<Value>,
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
        return ColumnType::to_u32(ct);
    }

    pub fn data_type_len(&self) -> u32 {
        let str_data_type: String = self.data_type.clone();
        let idx_first_paren = str_data_type.find("(");

        if idx_first_paren.is_none() {
            return 0;
        } else {
            let idx_first = idx_first_paren.unwrap();
            let idx_last = str_data_type.find(")").unwrap();
            let str_length = str_data_type.substring(idx_first, idx_last);
            let length: u32 = str_length.parse().unwrap();
            return length;
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
        return self.num_cols;
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
        return None;
    }

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

    pub fn to_cdata_rows(&self) -> Vec<crate::cdata::Row> {
        let mut result: Vec<crate::cdata::Row> = Vec::new();
        let mut idx = 0;

        for t_row in &self.rows {
            let mut c_values: Vec<crate::cdata::RowValue> = Vec::new();

            for t_val in &t_row.vals {
                let t_col_item = &t_val.col;

                let c_col_schema_item = crate::cdata::ColumnSchema {
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

                if c_bin_data.len() == 0 {
                    c_bin_data = &c_str_bin_data;
                }

                let c_bd = c_bin_data.to_vec();

                let c_val: crate::cdata::RowValue = crate::cdata::RowValue {
                    column: Some(c_col_schema_item),
                    is_null_value: if c_bd.len() > 0 { false } else { true },
                    value: c_bd,
                    string_value: c_str_data.clone(),
                };

                c_values.push(c_val);
            }

            let c_remote_data: crate::cdata::RowRemoteMetadata = crate::cdata::RowRemoteMetadata {
                is_hash_out_of_sync_with_host: false,
                is_local_deleted: false,
                is_remote_deleted: false,
                is_remote_out_of_sync_with_host: false,
            };

            let c_row = crate::cdata::Row {
                row_id: idx,
                table_name: self.name.clone(),
                database_name: String::from(""),
                values: c_values,
                is_remoteable: false,
                remote_metadata: Some(c_remote_data),
                hash: Vec::new(),
            };

            result.push(c_row);

            idx = idx + 1;
        }

        return result;
    }
}
