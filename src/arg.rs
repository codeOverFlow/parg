use std::any::Any;
use std::cell::{Cell, RefCell};
use std::fmt;

/// This enum indicates the expected type of the argument value.
#[derive(Debug)]
pub enum Type {
    /// Value is expected to be `u8`.
    ReadAsU8,
    /// Value is expected to be `u16`.
    ReadAsU16,
    /// Value is expected to be `u32`.
    ReadAsU32,
    /// Value is expected to be `u64`.
    ReadAsU64,
    /// Value is expected to be `u128`.
    ReadAsU128,
    /// Value is expected to be `usize`.
    ReadAsUsize,
    /// Value is expected to be `i8`.
    ReadAsI8,
    /// Value is expected to be `i16`.
    ReadAsI16,
    /// Value is expected to be `i32`.
    ReadAsI32,
    /// Value is expected to be `i64`.
    ReadAsI64,
    /// Value is expected to be `i128`.
    ReadAsI128,
    /// Value is expected to be `isize`.
    ReadAsIsize,
    /// Value is expected to be `f32`.
    ReadAsF32,
    /// Value is expected to be `f64`.
    ReadAsF64,
    /// Value is expected to be `bool`.
    ReadAsBool,
    /// Value is expected to be `char`.
    ReadAsChar,
    /// Value is expected to be `String`.
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

/// This structure represents an Argument for the command line
/// in the form "--arg_name value".
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
    /// Construct an `Arg` expecting a value.
    ///
    /// # Arguments
    /// * `name` - The name of the argument.
    /// * `reading_type` - The expected `Type` of the argument.
    /// * `required` - Check if whether or not the argument is required.
    ///
    /// # Example
    /// ```
    /// # use parg::arg::{Arg, Type};
    /// // match the optional i32 argument --foo <value>
    /// let arg = Arg::with_value("foo", Type::ReadAsI32, false);
    /// ```
    pub fn with_value(name: &str, reading_type: Type, required: bool) -> Arg {
        let arg_type = match reading_type {
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

    /// Construct an `Arg` expecting no value.
    ///
    /// # Arguments
    /// * `name` - The name of the argument.
    /// * `required` - Check if whether or not the argument is required.
    ///
    /// # Example
    ///
    /// ```
    /// # use parg::arg::{Arg, Type};
    /// // match the optional argument --foo
    /// let arg = Arg::without_value("foo", false);
    /// ```
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

    /// Get the name of the `Arg`.
    ///
    /// # Example
    /// ```
    /// # use parg::arg::{Arg, Type};
    /// // match the optional i32 argument --foo <value>
    /// let arg = Arg::with_value("foo", Type::ReadAsI32, false);
    /// assert_eq!(arg.get_name(), String::from("foo"));
    /// ```
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
