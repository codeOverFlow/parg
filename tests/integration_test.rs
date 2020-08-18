extern crate parg;
use parg::create_cli_arguments;
use parg::CliArguments;
use parg::{Arg, Type};

#[test]
fn integration() {
    let arg1 = Arg::with_value("arg1", Type::ReadAsI32, false);
    let cli: CliArguments = create_cli_arguments!(&arg1);

    let status = cli.parse();
    if let Err(_) = status {
        // ...
    }
}
