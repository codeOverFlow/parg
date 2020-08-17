extern crate parg;
use parg::arg::{Arg, Type};
use parg::cli_arguments::CliArguments;
use parg::create_cli_arguments;

#[test]
fn integration() {
    let arg1 = Arg::with_value("arg1", Type::ReadAsI32, false);
    let cli: CliArguments = create_cli_arguments!(arg1);

    let status = cli.parse();
    if let Err(_) = status {
        // ...
    }
}
