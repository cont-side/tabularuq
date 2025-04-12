use chrono::DateTime;

#[derive(Debug)]
pub enum DataType {
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    F32(f32),
    F64(f64),
    Bool(bool),
    String(String),
    Bytes(Vec<u8>),
    DateTime(DateTime<chrono::FixedOffset>),
}

impl DataType {
    pub fn value<T>(&self) -> Option<T>
    where
        T: FromDataType,
    {
        T::from_data_type(self)
    }
}

pub trait FromDataType {
    fn from_data_type(data_type: &DataType) -> Option<Self>
    where
        Self: Sized;
}

impl FromDataType for i8 {
    fn from_data_type(data_type: &DataType) -> Option<Self> {
        if let DataType::I8(value) = data_type { Some(*value) } else { None }
    }
}

impl FromDataType for i16 {
    fn from_data_type(data_type: &DataType) -> Option<Self> {
        if let DataType::I16(value) = data_type { Some(*value) } else { None }
    }
}

impl FromDataType for i32 {
    fn from_data_type(data_type: &DataType) -> Option<Self> {
        if let DataType::I32(value) = data_type { Some(*value) } else { None }
    }
}

impl FromDataType for i64 {
    fn from_data_type(data_type: &DataType) -> Option<Self> {
        if let DataType::I64(value) = data_type { Some(*value) } else { None }
    }
}

impl FromDataType for i128 {
    fn from_data_type(data_type: &DataType) -> Option<Self> {
        if let DataType::I128(value) = data_type { Some(*value) } else { None }
    }
}

impl FromDataType for u8 {
    fn from_data_type(data_type: &DataType) -> Option<Self> {
        if let DataType::U8(value) = data_type { Some(*value) } else { None }
    }
}

impl FromDataType for u16 {
    fn from_data_type(data_type: &DataType) -> Option<Self> {
        if let DataType::U16(value) = data_type { Some(*value) } else { None }
    }
}

impl FromDataType for u32 {
    fn from_data_type(data_type: &DataType) -> Option<Self> {
        if let DataType::U32(value) = data_type { Some(*value) } else { None }
    }
}

impl FromDataType for u64 {
    fn from_data_type(data_type: &DataType) -> Option<Self> {
        if let DataType::U64(value) = data_type { Some(*value) } else { None }
    }
}

impl FromDataType for u128 {
    fn from_data_type(data_type: &DataType) -> Option<Self> {
        if let DataType::U128(value) = data_type { Some(*value) } else { None }
    }
}

impl FromDataType for f32 {
    fn from_data_type(data_type: &DataType) -> Option<Self> {
        if let DataType::F32(value) = data_type { Some(*value) } else { None }
    }
}

impl FromDataType for f64 {
    fn from_data_type(data_type: &DataType) -> Option<Self> {
        if let DataType::F64(value) = data_type { Some(*value) } else { None }
    }
}

impl FromDataType for bool {
    fn from_data_type(data_type: &DataType) -> Option<Self> {
        if let DataType::Bool(value) = data_type { Some(*value) } else { None }
    }
}

impl FromDataType for String {
    fn from_data_type(data_type: &DataType) -> Option<Self> {
        if let DataType::String(value) = data_type { Some(value.clone()) } else { None }
    }
}

impl FromDataType for Vec<u8> {
    fn from_data_type(data_type: &DataType) -> Option<Self> {
        if let DataType::Bytes(value) = data_type { Some(value.clone()) } else { None }
    }
}

impl FromDataType for DateTime<chrono::FixedOffset> {
    fn from_data_type(data_type: &DataType) -> Option<Self> {
        if let DataType::DateTime(value) = data_type { Some(value.clone()) } else { None }
    }
}
