extern crate parg;

use parg::Arg;
use parg::Type;

fn main() {
    // create the required argument --threshold <u8 value>
    #[allow(unused_variables)]
    let threshold = Arg::with_value("threshold", Type::ReadAsU8, true);

    // create the optional argument --threshold <u8 value> with default value 42
    #[allow(unused_variables)]
    let threshold = Arg::with_default_value("threshold", Type::ReadAsU8, Box::new(42), false);

    // create the optional argument --threshold with no value
    let threshold = Arg::without_value("threshold", false);

    // add a description for the argument to show in usage
    threshold.set_description("a little description for the argument");
}
