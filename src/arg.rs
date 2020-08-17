use std::any::Any;
use std::cell::{Cell, RefCell};
use std::fmt;

#[derive(Debug)]
pub enum Type {
    ReadAsU8,
    ReadAsU16,
    ReadAsU32,
    ReadAsU64,
    ReadAsU128,
    ReadAsUsize,
    ReadAsI8,
    ReadAsI16,
    ReadAsI32,
    ReadAsI64,
    ReadAsI128,
    ReadAsIsize,
    ReadAsF32,
    ReadAsF64,
    ReadAsBool,
    ReadAsChar,
    ReadAsString,
}

#[derive(Debug)]
pub(crate) enum PrivateType {
    ReadAsU8(u8),
    ReadAsU16(u16),
    ReadAsU32(u32),
    ReadAsU64(u64),
    ReadAsU128(u128),
    ReadAsUsize(usize),
    ReadAsI8(i8),
    ReadAsI16(i16),
    ReadAsI32(i32),
    ReadAsI64(i64),
    ReadAsI128(i128),
    ReadAsIsize(isize),
    ReadAsF32(f32),
    ReadAsF64(f64),
    ReadAsBool(bool),
    ReadAsChar(char),
    ReadAsString(String),
}

pub struct Arg {
    name: String,
    pub(crate) type_read: Option<PrivateType>,
    pub(crate) required: bool,
    pub(crate) has_value: bool,
    pub(crate) value: RefCell<Option<Box<dyn Any>>>,
    pub(crate) found: Cell<bool>,
}

impl fmt::Debug for Arg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let borrowed_value = self.value.borrow();
        let value: String;
        if borrowed_value.is_some() {
            value = format!("{:?}", self.format_value());
        } else {
            value = String::from("None");
        }
        write!(f,
                "Arg (name: {:?}, type_read: {:?}, required: {:?}, has_value: {:?}, value: {:?}, found: {:?})",
                self.name,
                self.type_read,
                self.required,
                self.has_value,
                value,
                self.found
        )
    }
}

impl fmt::Display for Arg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let borrowed_value = self.value.borrow();
        let value: String;
        if borrowed_value.is_some() {
            value = format!("{}", self.format_value());
        } else {
            value = String::from("None");
        }
        if self.has_value {
            write!(f, "--{}={}", self.name, value)
        } else {
            write!(f, "--{}", self.name)
        }
    }
}

impl Arg {
    pub fn with_value(name: &str, type_id: Type, required: bool) -> Arg {
        let arg_type = match type_id {
            Type::ReadAsU8 => PrivateType::ReadAsU8(0),
            Type::ReadAsU16 => PrivateType::ReadAsU16(0),
            Type::ReadAsU32 => PrivateType::ReadAsU32(0),
            Type::ReadAsU64 => PrivateType::ReadAsU64(0),
            Type::ReadAsU128 => PrivateType::ReadAsU128(0),
            Type::ReadAsUsize => PrivateType::ReadAsUsize(0),
            Type::ReadAsI8 => PrivateType::ReadAsI8(0),
            Type::ReadAsI16 => PrivateType::ReadAsI16(0),
            Type::ReadAsI32 => PrivateType::ReadAsI32(0),
            Type::ReadAsI64 => PrivateType::ReadAsI64(0),
            Type::ReadAsI128 => PrivateType::ReadAsI128(0),
            Type::ReadAsIsize => PrivateType::ReadAsIsize(0),
            Type::ReadAsF32 => PrivateType::ReadAsF32(0.0),
            Type::ReadAsF64 => PrivateType::ReadAsF64(0.0),
            Type::ReadAsBool => PrivateType::ReadAsBool(false),
            Type::ReadAsChar => PrivateType::ReadAsChar('0'),
            Type::ReadAsString => PrivateType::ReadAsString(String::new()),
        };

        Arg {
            name: name.to_string(),
            type_read: Some(arg_type),
            required,
            has_value: true,
            value: RefCell::new(None),
            found: Cell::new(false),
        }
    }

    pub fn without_value(name: &str, required: bool) -> Arg {
        Arg {
            name: name.to_string(),
            type_read: None,
            required,
            has_value: false,
            value: RefCell::new(None),
            found: Cell::new(false),
        }
    }

    pub fn get_name(&self) -> String {
        String::from(&self.name)
    }

    pub(crate) fn format_value(&self) -> String {
        if self.has_value {
            let borrowed_value = self.value.borrow();

            if borrowed_value.is_some() {
                let value = borrowed_value.as_ref().expect(&*format!(
                    "Error unwrapping value for argument {}",
                    self.name
                ));
                match self.type_read {
                    Some(PrivateType::ReadAsU8(_)) => match value.downcast_ref::<u8>() {
                        Some(v) => format!("{:?}", v),
                        None => String::from("None"),
                    },
                    Some(PrivateType::ReadAsU16(_)) => match value.downcast_ref::<u16>() {
                        Some(v) => format!("{:?}", v),
                        None => String::from("None"),
                    },
                    Some(PrivateType::ReadAsU32(_)) => match value.downcast_ref::<u32>() {
                        Some(v) => format!("{:?}", v),
                        None => String::from("None"),
                    },
                    Some(PrivateType::ReadAsU64(_)) => match value.downcast_ref::<u64>() {
                        Some(v) => format!("{:?}", v),
                        None => String::from("None"),
                    },
                    Some(PrivateType::ReadAsU128(_)) => match value.downcast_ref::<u128>() {
                        Some(v) => format!("{:?}", v),
                        None => String::from("None"),
                    },
                    Some(PrivateType::ReadAsUsize(_)) => match value.downcast_ref::<usize>() {
                        Some(v) => format!("{:?}", v),
                        None => String::from("None"),
                    },
                    Some(PrivateType::ReadAsI8(_)) => match value.downcast_ref::<i8>() {
                        Some(v) => format!("{:?}", v),
                        None => String::from("None"),
                    },
                    Some(PrivateType::ReadAsI16(_)) => match value.downcast_ref::<i16>() {
                        Some(v) => format!("{:?}", v),
                        None => String::from("None"),
                    },
                    Some(PrivateType::ReadAsI32(_)) => match value.downcast_ref::<i32>() {
                        Some(v) => format!("{:?}", v),
                        None => String::from("None"),
                    },
                    Some(PrivateType::ReadAsI64(_)) => match value.downcast_ref::<i64>() {
                        Some(v) => format!("{:?}", v),
                        None => String::from("None"),
                    },
                    Some(PrivateType::ReadAsI128(_)) => match value.downcast_ref::<i128>() {
                        Some(v) => format!("{:?}", v),
                        None => String::from("None"),
                    },
                    Some(PrivateType::ReadAsIsize(_)) => match value.downcast_ref::<isize>() {
                        Some(v) => format!("{:?}", v),
                        None => String::from("None"),
                    },
                    Some(PrivateType::ReadAsF32(_)) => match value.downcast_ref::<f32>() {
                        Some(v) => format!("{:?}", v),
                        None => String::from("None"),
                    },
                    Some(PrivateType::ReadAsF64(_)) => match value.downcast_ref::<f64>() {
                        Some(v) => format!("{:?}", v),
                        None => String::from("None"),
                    },
                    Some(PrivateType::ReadAsBool(_)) => match value.downcast_ref::<bool>() {
                        Some(v) => format!("{:?}", v),
                        None => String::from("None"),
                    },
                    Some(PrivateType::ReadAsChar(_)) => match value.downcast_ref::<char>() {
                        Some(v) => format!("{:?}", v),
                        None => String::from("None"),
                    },
                    Some(PrivateType::ReadAsString(_)) => match value.downcast_ref::<String>() {
                        Some(v) => format!("{:?}", v),
                        None => String::from("None"),
                    },
                    None => String::new(),
                }
            } else {
                String::new()
            }
        } else {
            String::new()
        }
    }
}
