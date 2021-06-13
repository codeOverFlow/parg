use std::any::{Any, TypeId};
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::fmt;
use core::iter::Iterator;

use crate::arg::{Arg, PrivateType};

/// Argument Engine looking for all `Arg`.
pub struct CliArguments<'a> {
    app_name: RefCell<String>,
    description: RefCell<String>,
    named_args: BTreeMap<String, &'a Arg>,
}

impl fmt::Display for CliArguments<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (_, arg) in self.named_args.iter() {
            writeln!(f, "{}", arg)?;
        }
        write!(f, "")
    }
}

impl<'a> CliArguments<'a> {
    ///  Construct a new CliArguments.
    ///
    /// # Arguments
    /// * `named_args` - A `BTreeMap<String, &Arg>` of the arguments
    ///
    /// # Example
    /// Can be use directly
    /// ```
    /// # use parg::{Arg, Type};
    /// # use std::collections::BTreeMap;
    /// # use parg::CliArguments;
    /// let a = Arg::with_value("config", Type::ReadAsString, true);
    /// let b = Arg::with_value("thread", Type::ReadAsU8, false);
    /// let c = Arg::without_value("verbose", false);
    /// let mut tree: BTreeMap<String, &Arg> = BTreeMap::new();
    ///
    /// tree.insert(a.get_name(), &a);
    /// tree.insert(b.get_name(), &b);
    /// tree.insert(c.get_name(), &c);
    ///
    /// // Create the cli
    /// let cli: CliArguments = CliArguments::new(tree);
    /// ```
    /// Or using `create_cli_argument!`
    /// ```
    /// # #[macro_use] extern crate parg;
    /// # use parg::{Arg, Type};
    /// # use parg::CliArguments;
    /// # fn main() {
    /// let a = Arg::with_value("config", Type::ReadAsString, true);
    /// let b = Arg::with_value("thread", Type::ReadAsU8, false);
    /// let c = Arg::without_value("verbose", false);
    ///
    /// // Create the cli
    /// let cli: CliArguments = create_cli_arguments!(&a, &b, &c);
    /// # }
    /// ```
    pub fn new(named_args: BTreeMap<String, &'a Arg>) -> CliArguments {
        CliArguments {
            app_name: RefCell::new(String::new()),
            description: RefCell::new(String::new()),
            named_args,
        }
    }

    fn check_args(&self) -> Result<(), String> {
        for (name, arg) in self.named_args.iter() {
            if !arg.found.get() {
                if !arg.has_default_value() {
                    if arg.required {
                        return Err(format!(
                            "Argument --{} is required !\n{}",
                            name,
                            self.generate_usage()
                        ));
                    }
                } else {
                    arg.accept_default_value()?;
                }
            }

            if arg.found.get() && arg.has_value {
                if arg.value.borrow().is_none() {
                    if !arg.has_default_value() {
                        return Err(format!(
                            "Argument --{} needs a value !\n{}",
                            name,
                            self.generate_usage()
                        ));
                    } else {
                        arg.accept_default_value()?;
                    }
                }
            }
        }
        Ok(())
    }

    fn check_type(&self, type_id: TypeId, type_read: &PrivateType) -> bool {
        let type_read_type_id = match type_read {
            PrivateType::ReadAsU8(sample) => sample.type_id(),
            PrivateType::ReadAsU16(sample) => sample.type_id(),
            PrivateType::ReadAsU32(sample) => sample.type_id(),
            PrivateType::ReadAsU64(sample) => sample.type_id(),
            PrivateType::ReadAsU128(sample) => sample.type_id(),
            PrivateType::ReadAsUsize(sample) => sample.type_id(),
            PrivateType::ReadAsI8(sample) => sample.type_id(),
            PrivateType::ReadAsI16(sample) => sample.type_id(),
            PrivateType::ReadAsI32(sample) => sample.type_id(),
            PrivateType::ReadAsI64(sample) => sample.type_id(),
            PrivateType::ReadAsI128(sample) => sample.type_id(),
            PrivateType::ReadAsIsize(sample) => sample.type_id(),
            PrivateType::ReadAsF32(sample) => sample.type_id(),
            PrivateType::ReadAsF64(sample) => sample.type_id(),
            PrivateType::ReadAsBool(sample) => sample.type_id(),
            PrivateType::ReadAsChar(sample) => sample.type_id(),
            PrivateType::ReadAsString(sample) => sample.type_id(),
        };
        type_id == type_read_type_id
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
    /// # use parg::{Arg, Type};
    /// # use parg::CliArguments;
    /// # fn main() {
    /// let a = Arg::with_value("config", Type::ReadAsString, true);
    /// let b = Arg::with_value("thread", Type::ReadAsU8, false);
    /// let c = Arg::without_value("verbose", false);
    ///
    /// // Create the cli
    /// let cli: CliArguments = create_cli_arguments!(&a, &b, &c);
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

    /// Generate a text to explain usage
    pub fn generate_usage(&self) -> String {
        let mut params = String::new();
        let mut params_descr = format!("{:23}Print this help\n", "--help");
        for (name, arg) in self.named_args.iter() {
            params = format!("{} --{} <value>", params, name);
            params_descr = format!(
                "{}--{} {:10}    {} (default: {})\n",
                params_descr,
                name,
                "<value>",
                arg.description.borrow(),
                arg.format_default_value()
            );
        }
        format!(
            "{}\nUsage:\n{}{}\n\nArguments:\n{}",
            self.description.borrow(),
            self.app_name.borrow(),
            params,
            params_descr
        )
    }

    ///  Get the value of the `arg_name` argument.
    ///
    /// # Arguments
    /// * `arg_name` - The name of the `Arg` to get the value of.
    ///
    /// # Returns
    /// Return a `value: T`, T being the requested type.
    ///
    /// # Example
    /// ```
    /// # #[macro_use] extern crate parg;
    /// # use parg::{Arg, Type};
    /// # use parg::CliArguments;
    /// # fn main() {
    /// let a = Arg::with_value("config", Type::ReadAsString, true);
    /// let b = Arg::with_value("thread", Type::ReadAsU8, false);
    /// let c = Arg::without_value("verbose", false);
    ///
    /// // Create the cli
    /// let cli: CliArguments = create_cli_arguments!(&a, &b, &c);
    ///
    /// // parse args and get return status
    /// let return_status = cli.parse();
    /// if let Err(msg) = return_status {
    ///     eprintln!("{}", msg);
    ///     return;
    /// }
    ///
    /// // get the value as a String
    /// let config: String = cli.get_value("config");
    /// // if command is ... --config "a config" ...
    /// // print: config = a config
    /// println!("config = {}", config);
    /// # }
    /// ```
    pub fn get_value<T: 'static + Clone>(&self, arg_name: &str) -> T {
        if let Some(arg) = self.named_args.get(arg_name) {
            if arg.has_value {
                // check that types match
                if let Some(type_read) = &arg.type_read {
                    let is_type_conform = self.check_type(TypeId::of::<T>(), type_read);
                    let is_option_type_conform =
                        self.check_type(TypeId::of::<Option<T>>(), type_read);
                    if !is_type_conform && !is_option_type_conform {
                        panic!(
                            "The requested type for \"{}\" does not match the reading type !",
                            arg_name
                        );
                    }
                }

                // access to the value
                let borrowed_value = arg.value.borrow();
                let value = match borrowed_value.as_ref() {
                    Some(v) => v,
                    None => match arg.default_value {
                        Some(ref default) => default,
                        None => panic!("\"{}\" has no value nor default value !", arg_name),
                    },
                };

                // cast then return Some(value) or panic
                match value.downcast_ref::<T>() {
                    Some(v) => v.clone(),
                    None => panic!("Error downcasting argument {}", arg_name),
                }
            } else {
                panic!("Argument {} does not take a value !", arg_name);
            }
        } else {
            panic!("Argument \"{}\" does not exists !", arg_name)
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
    /// # use parg::{Arg, Type};
    /// # use parg::CliArguments;
    /// # fn main() {
    /// let a = Arg::with_value("config", Type::ReadAsString, true);
    /// let b = Arg::with_value("thread", Type::ReadAsU8, false);
    /// let c = Arg::without_value("verbose", false);
    ///
    /// // Create the cli
    /// let cli: CliArguments = create_cli_arguments!(&a, &b, &c);
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
        self.internal_parse(std::env::args().skip(1))
    }

    pub fn parse_subset<T>(&self, args: T) -> Result<(), String>
        where
            T: Iterator<Item = String>
    {
        self.internal_parse(args)
    }

    fn internal_parse<T>(&self, args: T) -> Result<(), String>
        where
            T: Iterator<Item = String>
    {
        self.reset_args();
        let mut last_arg_name = String::new();
        let mut read_value = false;
        for arg in args {
            if read_value {
                read_value = false;
                self.read_value(String::from(&arg), String::from(&last_arg_name))?;
            }

            if arg.starts_with("--") && arg.chars().count() >= 3 {
                if arg.eq_ignore_ascii_case("--help") {
                    println!("{}", self.generate_usage());
                    return Err(String::new());
                }
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

    ///  Sets the cli name and description.
    ///
    /// # Arguments
    /// * `app_name` - The name of the cli app.
    /// * `description` - The description of the cli app.
    ///
    /// # Example
    ///
    /// ```
    /// # #[macro_use] extern crate parg;
    /// # use parg::{Arg, Type};
    /// # use parg::CliArguments;
    /// # fn main() {
    /// let a = Arg::with_value("config", Type::ReadAsString, true);
    /// let b = Arg::with_value("thread", Type::ReadAsU8, false);
    /// let c = Arg::without_value("verbose", false);
    ///
    /// // Create the cli
    /// let cli: CliArguments = create_cli_arguments!(&a, &b, &c);
    /// cli.set_info("my_app", "The app description");
    /// # }
    /// ```
    pub fn set_info(&self, app_name: &str, description: &str) {
        self.app_name.replace(String::from(app_name));
        self.description.replace(String::from(description));
    }
}
