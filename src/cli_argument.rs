use crate::arg::{Arg, PrivateType};
use std::any::{Any, TypeId};
use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug)]
pub struct CliArguments {
    named_args: BTreeMap<String, Arg>,
}

impl fmt::Display for CliArguments {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (_, arg) in self.named_args.iter() {
            writeln!(f, "{}", arg)?;
        }
        write!(f, "")
    }
}

impl CliArguments {
    pub fn new(named_args: BTreeMap<String, Arg>) -> CliArguments {
        CliArguments { named_args }
    }

    fn reset_args(&self) {
        for (_, arg) in self.named_args.iter() {
            arg.value.replace(None);
            arg.found.set(false);
        }
    }

    fn read_value(&self, arg: String, arg_name: String) -> Result<(), String> {
        if let Some(argument) = self.named_args.get(&arg_name) {
            if argument.has_value {
                let value: Box<dyn Any> = match &argument.type_read {
                    Some(PrivateType::ReadAsU8(_)) => {
                        let tmp = arg.parse::<u8>().map_err(|e| {
                            format!("Argument value {} for {} must be u8: {}", arg, arg_name, e)
                        })?;
                        Box::new(tmp)
                    }
                    Some(PrivateType::ReadAsU16(_)) => {
                        let tmp = arg.parse::<u16>().map_err(|e| {
                            format!("Argument value {} for {} must be u16: {}", arg, arg_name, e)
                        })?;
                        Box::new(tmp)
                    }
                    Some(PrivateType::ReadAsU32(_)) => {
                        let tmp = arg.parse::<u32>().map_err(|e| {
                            format!("Argument value {} for {} must be u32: {}", arg, arg_name, e)
                        })?;
                        Box::new(tmp)
                    }
                    Some(PrivateType::ReadAsU64(_)) => {
                        let tmp = arg.parse::<u64>().map_err(|e| {
                            format!("Argument value {} for {} must be u64: {}", arg, arg_name, e)
                        })?;
                        Box::new(tmp)
                    }
                    Some(PrivateType::ReadAsU128(_)) => {
                        let tmp = arg.parse::<u128>().map_err(|e| {
                            format!(
                                "Argument value {} for {} must be u128: {}",
                                arg, arg_name, e
                            )
                        })?;
                        Box::new(tmp)
                    }
                    Some(PrivateType::ReadAsUsize(_)) => {
                        let tmp = arg.parse::<usize>().map_err(|e| {
                            format!(
                                "Argument value {} for {} must be usize: {}",
                                arg, arg_name, e
                            )
                        })?;
                        Box::new(tmp)
                    }
                    Some(PrivateType::ReadAsI8(_)) => {
                        let tmp = arg.parse::<i8>().map_err(|e| {
                            format!("Argument value {} for {} must be i8: {}", arg, arg_name, e)
                        })?;
                        Box::new(tmp)
                    }
                    Some(PrivateType::ReadAsI16(_)) => {
                        let tmp = arg.parse::<i16>().map_err(|e| {
                            format!("Argument value {} for {} must be i16: {}", arg, arg_name, e)
                        })?;
                        Box::new(tmp)
                    }
                    Some(PrivateType::ReadAsI32(_)) => {
                        let tmp = arg.parse::<i32>().map_err(|e| {
                            format!("Argument value {} for {} must be i32: {}", arg, arg_name, e)
                        })?;
                        Box::new(tmp)
                    }
                    Some(PrivateType::ReadAsI64(_)) => {
                        let tmp = arg.parse::<i64>().map_err(|e| {
                            format!("Argument value {} for {} must be i64: {}", arg, arg_name, e)
                        })?;
                        Box::new(tmp)
                    }
                    Some(PrivateType::ReadAsI128(_)) => {
                        let tmp = arg.parse::<i128>().map_err(|e| {
                            format!(
                                "Argument value {} for {} must be i128: {}",
                                arg, arg_name, e
                            )
                        })?;
                        Box::new(tmp)
                    }
                    Some(PrivateType::ReadAsIsize(_)) => {
                        let tmp = arg.parse::<isize>().map_err(|e| {
                            format!(
                                "Argument value {} for {} must be isize: {}",
                                arg, arg_name, e
                            )
                        })?;
                        Box::new(tmp)
                    }
                    Some(PrivateType::ReadAsF32(_)) => {
                        let tmp = arg.parse::<f32>().map_err(|e| {
                            format!("Argument value {} for {} must be f32: {}", arg, arg_name, e)
                        })?;
                        Box::new(tmp)
                    }
                    Some(PrivateType::ReadAsF64(_)) => {
                        let tmp = arg.parse::<f64>().map_err(|e| {
                            format!("Argument value {} for {} must be f64: {}", arg, arg_name, e)
                        })?;
                        Box::new(tmp)
                    }
                    Some(PrivateType::ReadAsBool(_)) => {
                        let tmp = arg.parse::<bool>().map_err(|e| {
                            format!(
                                "Argument value {} for {} must be bool: {}",
                                arg, arg_name, e
                            )
                        })?;
                        Box::new(tmp)
                    }
                    Some(PrivateType::ReadAsChar(_)) => {
                        let tmp = arg.parse::<char>().map_err(|e| {
                            format!(
                                "Argument value {} for {} must be char: {}",
                                arg, arg_name, e
                            )
                        })?;
                        Box::new(tmp)
                    }
                    Some(PrivateType::ReadAsString(_)) => Box::new(String::from(arg)),
                    None => return Err(format!("Argument {} must have a value", arg_name)),
                };
                argument.value.replace(Some(value));
            } else {
                argument.value.replace(Some(Box::new(true)));
            }
        }
        Ok(())
    }

    fn check_args(&self) -> Result<(), String> {
        for (name, arg) in self.named_args.iter() {
            if arg.required {
                if !arg.found.get() {
                    return Err(format!("Argument --{} is required !", name));
                }
            }

            if arg.found.get() && arg.has_value {
                if arg.value.borrow().is_none() {
                    return Err(format!("Argument --{} needs a value !", name));
                }
            }
        }
        Ok(())
    }

    fn check_type(&self, type_id: TypeId, type_read: &PrivateType) -> bool {
        match type_read {
            PrivateType::ReadAsU8(sample) => type_id == sample.type_id(),
            PrivateType::ReadAsU16(sample) => type_id == sample.type_id(),
            PrivateType::ReadAsU32(sample) => type_id == sample.type_id(),
            PrivateType::ReadAsU64(sample) => type_id == sample.type_id(),
            PrivateType::ReadAsU128(sample) => type_id == sample.type_id(),
            PrivateType::ReadAsUsize(sample) => type_id == sample.type_id(),
            PrivateType::ReadAsI8(sample) => type_id == sample.type_id(),
            PrivateType::ReadAsI16(sample) => type_id == sample.type_id(),
            PrivateType::ReadAsI32(sample) => type_id == sample.type_id(),
            PrivateType::ReadAsI64(sample) => type_id == sample.type_id(),
            PrivateType::ReadAsI128(sample) => type_id == sample.type_id(),
            PrivateType::ReadAsIsize(sample) => type_id == sample.type_id(),
            PrivateType::ReadAsF32(sample) => type_id == sample.type_id(),
            PrivateType::ReadAsF64(sample) => type_id == sample.type_id(),
            PrivateType::ReadAsBool(sample) => type_id == sample.type_id(),
            PrivateType::ReadAsChar(sample) => type_id == sample.type_id(),
            PrivateType::ReadAsString(sample) => type_id == sample.type_id(),
        }
    }

    pub fn get_value<T: 'static + Clone>(&self, arg_name: &str) -> Result<T, String> {
        if let Some(arg) = self.named_args.get(arg_name) {
            if arg.has_value {
                if let Some(type_read) = &arg.type_read {
                    let is_type_conform = self.check_type(TypeId::of::<T>(), type_read);
                    if !is_type_conform {
                        return Err(format!(
                            "The requested type for \"{}\" does not match the reading type !",
                            arg_name
                        ));
                    }
                }
                let borrowed_value = arg.value.borrow();
                let value = borrowed_value.as_ref().unwrap();
                match value.downcast_ref::<T>() {
                    Some(v) => Ok(v.clone()),
                    None => Err(format!("Error downcasting argument {}", arg_name)),
                }
            } else {
                Err(format!("Argument {} does not take a value !", arg_name))
            }
        } else {
            Err(format!("Argument {} does not exists !", arg_name))
        }
    }

    pub fn exists(&self, arg_name: &str) -> bool {
        if let Some(arg) = self.named_args.get(arg_name) {
            arg.found.get()
        } else {
            false
        }
    }

    pub fn parse(&self) -> Result<(), String> {
        self.reset_args();
        let mut last_arg_name = String::new();
        let mut read_value = false;
        for arg in std::env::args().skip(1) {
            if read_value {
                read_value = false;
                self.read_value(String::from(&arg), String::from(&last_arg_name))?;
            }

            if arg.starts_with("--") && arg.chars().count() >= 3 {
                last_arg_name = String::from(&arg[2..]);
                if let Some(argument) = self.named_args.get(&last_arg_name) {
                    read_value = true;
                    argument.found.set(true);
                }
            }
        }

        // make checks
        self.check_args()?;
        Ok(())
    }
}
