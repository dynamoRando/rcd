use substring::Substring;


#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ColumnType {
    Unknown = 0,
    Int = 1,
    Bit = 2,
    Char = 3,
    DateTime = 4,
    Decimal = 5,
    Varchar = 6,
    Binary = 7,
    Varbinary = 8,
    Text = 9,
}

impl ColumnType {
    pub fn data_type_as_string_sqlite(self: &Self) -> String {
        match self {
            ColumnType::Unknown => panic!(),
            ColumnType::Int => String::from("INT"),
            ColumnType::Bit => String::from("TINYINT"),
            ColumnType::Char => String::from("CHAR"),
            ColumnType::DateTime => String::from("DATETIME"),
            ColumnType::Decimal => String::from("DECIMAL"),
            ColumnType::Varchar => String::from("VARCHAR"),
            ColumnType::Binary => String::from("BLOB"),
            ColumnType::Varbinary => String::from("BLOB"),
            ColumnType::Text => String::from("TEXT"),
        }
    }

    pub fn data_type_to_enum_u32(desc: String) -> u32 {
        println!("{:?}", desc);
        let ct = ColumnType::try_parse(&desc).unwrap();
        return ColumnType::to_u32(ct);
    }

    pub fn data_type_len(desc: String) -> u32 {
        let idx_first_paren = desc.find("(");

        if idx_first_paren.is_none() {
            return 0;
        } else {
            let idx_first = idx_first_paren.unwrap();
            let idx_last = desc.find(")").unwrap();
            let str_length = desc.substring(idx_first + 1, idx_last);
            println!("{}", str_length);
            let length: u32 = str_length.parse().unwrap();
            return length;
        }
    }

    pub fn try_parse(desc: &str) -> Option<ColumnType> {
        let string_data_type = desc.to_lowercase();

        if string_data_type.len() == 0 {
            return Some(ColumnType::Unknown);
        }

        if string_data_type.contains("int") {
            return Some(ColumnType::Int);
        }

        if string_data_type.contains("bit") {
            return Some(ColumnType::Bit);
        }

        if string_data_type.contains("varchar") {
            return Some(ColumnType::Varchar);
        }

        if string_data_type.contains("char") {
            return Some(ColumnType::Char);
        }

        if string_data_type.contains("datetime") {
            return Some(ColumnType::DateTime);
        }

        if string_data_type.contains("decimal") {
            return Some(ColumnType::Decimal);
        }

        if string_data_type.contains("varbinary") {
            return Some(ColumnType::Varbinary);
        }

        if string_data_type.contains("blob") {
            return Some(ColumnType::Varbinary);
        }

        if string_data_type.contains("binary") {
            return Some(ColumnType::Binary);
        }

        if string_data_type.contains("text") {
            return Some(ColumnType::Text);
        }

        return None;
    }

    pub fn from_u32(value: u32) -> ColumnType {
        match value {
            0 => ColumnType::Unknown,
            1 => ColumnType::Int,
            2 => ColumnType::Bit,
            3 => ColumnType::Char,
            4 => ColumnType::DateTime,
            5 => ColumnType::Decimal,
            6 => ColumnType::Varchar,
            7 => ColumnType::Binary,
            8 => ColumnType::Varbinary,
            9 => ColumnType::Text,
            _ => panic!("Unknown value: {}", value),
        }
    }

    pub fn to_u32(col_type: ColumnType) -> u32 {
        match col_type {
            ColumnType::Unknown => 0,
            ColumnType::Int => 1,
            ColumnType::Bit => 2,
            ColumnType::Char => 3,
            ColumnType::DateTime => 4,
            ColumnType::Decimal => 5,
            ColumnType::Varchar => 6,
            ColumnType::Binary => 7,
            ColumnType::Varbinary => 8,
            ColumnType::Text => 9,
        }
    }
}