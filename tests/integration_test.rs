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

#[test]
fn integration_2() {
    let all = Arg::without_value("all", false);
    let msg = Arg::with_value("msg", Type::ReadAsString, false);
    let cli: CliArguments = create_cli_arguments!(&all, &msg);
    let args = vec!["git".to_string(),
                    "commit".to_string(),
                    "--all".to_string(),
                    "--msg".to_string(),
                    "\"message\"".to_string()
    ];

    let status = cli.parse_subset(args.into_iter().skip(2));
    if let Err(_) = status {
        // ...
    }
}
