use crate::arg::{Arg, PrivateType};
use std::any::{Any, TypeId};
use std::collections::BTreeMap;
use std::fmt;

/// Argument Engine looking for all `Arg`.
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
    ///  Construct a new CliArguments.
    ///
    /// # Arguments
    /// * `named_args` - A `BTreeMap<String, Arg` of the arguments
    ///
    /// # Example
    /// Can be use directly
    /// ```
    /// # use parg::arg::{Arg, Type};
    /// # use std::collections::BTreeMap;
    /// # use parg::cli_arguments::CliArguments;
    /// let a = Arg::with_value("config", Type::ReadAsString, true);
    /// let b = Arg::with_value("thread", Type::ReadAsU8, false);
    /// let c = Arg::without_value("verbose", false);
    /// let mut tree: BTreeMap<String, Arg> = BTreeMap::new();
    ///
    /// tree.insert(a.get_name(), a);
    /// tree.insert(b.get_name(), b);
    /// tree.insert(c.get_name(), c);
    ///
    /// // Create the cli
    /// let cli: CliArguments = CliArguments::new(tree);
    /// ```
    /// Or using `create_cli_argument!`
    /// ```
    /// # #[macro_use] extern crate parg;
    /// # use parg::arg::{Arg, Type};
    /// # use parg::cli_arguments::CliArguments;
    /// # fn main() {
    /// let a = Arg::with_value("config", Type::ReadAsString, true);
    /// let b = Arg::with_value("thread", Type::ReadAsU8, false);
    /// let c = Arg::without_value("verbose", false);
    ///
    /// // Create the cli
    /// let cli: CliArguments = create_cli_arguments!(a, b, c);
    /// # }
    /// ```
    pub fn new(named_args: BTreeMap<String, Arg>) -> CliArguments {
        CliArguments { named_args }
    }

    fn check_args(&self) -> Result<(), String> {
        for (name, arg) in self.named_args.iter() {
            if !arg.found.get() {
                if !arg.has_default_value() {
                    if arg.required {
                        return Err(format!("Argument --{} is required !", name));
                    }
                } else {
                    arg.accept_default_value()?;
                }
            }

            if arg.found.get() && arg.has_value {
                if arg.value.borrow().is_none() {
                    if !arg.has_default_value() {
                        return Err(format!("Argument --{} needs a value !", name));
                    } else {
                        arg.accept_default_value()?;
                    }
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

    ///  Check if an `Arg` exists.
    ///
    /// # Arguments
    /// * `arg_name` - The name of the `Arg` to check.
    ///
    /// # Returns
    /// Return `true` if the `Arg` exists, `false` otherwise.
    ///
    /// # Example
    /// ```
    /// # #[macro_use] extern crate parg;
    /// # use parg::arg::{Arg, Type};
    /// # use parg::cli_arguments::CliArguments;
    /// # fn main() {
    /// let a = Arg::with_value("config", Type::ReadAsString, true);
    /// let b = Arg::with_value("thread", Type::ReadAsU8, false);
    /// let c = Arg::without_value("verbose", false);
    ///
    /// // Create the cli
    /// let cli: CliArguments = create_cli_arguments!(a, b, c);
    ///
    /// // parse args and get return status
    /// let return_status = cli.parse();
    /// if let Err(msg) = return_status {
    ///     eprintln!("{}", msg);
    ///     return;
    /// }
    ///
    /// // get the value as a String
    /// let verbose = cli.exists("verbose");
    ///
    /// if verbose {
    ///     // do stuff
    /// }
    /// # }
    /// ```
    pub fn exists(&self, arg_name: &str) -> bool {
        if let Some(arg) = self.named_args.get(arg_name) {
            arg.found.get()
        } else {
            false
        }
    }

    ///  Get the value of the `arg_name` argument.
    ///
    /// # Arguments
    /// * `arg_name` - The name of the `Arg` to get the value of.
    ///
    /// # Returns
    /// Return a `Result<T, String>`, T being the requested type and String the error message.
    ///
    /// # Example
    /// ```
    /// # #[macro_use] extern crate parg;
    /// # use parg::arg::{Arg, Type};
    /// # use parg::cli_arguments::CliArguments;
    /// # fn main() {
    /// let a = Arg::with_value("config", Type::ReadAsString, true);
    /// let b = Arg::with_value("thread", Type::ReadAsU8, false);
    /// let c = Arg::without_value("verbose", false);
    ///
    /// // Create the cli
    /// let cli: CliArguments = create_cli_arguments!(a, b, c);
    ///
    /// // parse args and get return status
    /// let return_status = cli.parse();
    /// if let Err(msg) = return_status {
    ///     eprintln!("{}", msg);
    ///     return;
    /// }
    ///
    /// // get the value as a String
    /// let config: String = match cli.get_value("config") {
    ///     Ok(value) => value,
    ///     Err(msg) => {
    ///         eprintln!("{}", msg);
    ///         return;
    ///     }
    /// };
    /// // if command is ... --config "a config" ...
    /// // print: config = a config
    /// println!("config = {}", config);
    /// # }
    /// ```
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
                let value = match borrowed_value.as_ref() {
                    Some(v) => v,
                    None => match arg.default_value {
                        Some(ref default) => default,
                        None => {
                            return Err(format!(
                                "\"{}\" has no value nor default value !",
                                arg_name
                            ))
                        }
                    },
                };
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

    ///  Parse the command line arguments.
    ///
    /// # Returns
    /// Return a `Result<(), String>`, String being the error message if any.
    ///
    /// # Example
    /// ```
    /// # #[macro_use] extern crate parg;
    /// # use parg::arg::{Arg, Type};
    /// # use parg::cli_arguments::CliArguments;
    /// # fn main() {
    /// let a = Arg::with_value("config", Type::ReadAsString, true);
    /// let b = Arg::with_value("thread", Type::ReadAsU8, false);
    /// let c = Arg::without_value("verbose", false);
    ///
    /// // Create the cli
    /// let cli: CliArguments = create_cli_arguments!(a, b, c);
    ///
    /// // parse args and get return status
    /// let return_status = cli.parse();
    /// if let Err(msg) = return_status {
    ///     eprintln!("{}", msg);
    ///     return;
    /// }
    /// # }
    /// ```
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

    fn reset_args(&self) {
        for (_, arg) in self.named_args.iter() {
            arg.value.replace(None);
            arg.found.set(false);
        }
    }
}
