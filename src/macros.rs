/// Create a `CliArguments` with all `Arg` given to the macro.
///
/// # Examples
///
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
///
/// This gives a shortcut to.
///
/// ```
/// # #[macro_use] extern crate parg;
/// # use parg::arg::{Arg, Type};
/// # use parg::cli_arguments::CliArguments;
/// # use std::collections::BTreeMap;
/// # fn main() {
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
/// # }
/// ```
#[macro_export]
macro_rules! create_cli_arguments {
    ($($args:expr),+) => {
        {
            use std::collections::BTreeMap;
            let mut tree: BTreeMap<String, Arg> = BTreeMap::new();
            for arg in vec![$($args), *] {
                tree.insert(arg.get_name(), arg);
            }
            CliArguments::new(tree)
        }
    };
}
