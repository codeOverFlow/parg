extern crate parg;

use parg::create_cli_arguments;
use parg::Arg;
use parg::CliArguments;
use parg::Type;

fn main() {
    // create the required argument --threshold <u8 value>
    let threshold = Arg::with_value("threshold", Type::ReadAsU8, true);
    threshold.set_description("a little description for the argument");

    let path = Arg::with_value("path", Type::ReadAsString, true);

    // create the cli with arguments
    let cli: CliArguments = create_cli_arguments!(&path, &threshold);
    // set the name and description to print in usage
    cli.set_info("my_command", "The description");
}
