#[macro_export]
macro_rules! create_cli_argument {
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
