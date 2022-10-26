use argparse;

pub struct Options {
    pub level_name: String,
    pub show_description: bool,
    pub show_level_list: bool,
}

pub fn parse_args() -> Options {
    // Convert command line args into an Options struct

    // Init default options
    let mut options = Options {
        level_name: "Default".to_string(),
        show_description: false,
        show_level_list: false,
    };

    // Set up argparser and use it
    {
        let mut parser = argparse::ArgumentParser::new();
        parser.refer(&mut options.level_name).add_argument(
            "scenario",
            argparse::Store,
            "Scenario name. Use -l option to see available scenarios",
        );
        parser.refer(&mut options.show_level_list).add_option(
            &["-l", "--list"],
            argparse::StoreTrue,
            "List available scenarios",
        );
        parser.refer(&mut options.show_description).add_option(
            &["-d", "--description"],
            argparse::StoreTrue,
            "Show the description of a level",
        );
        parser.parse_args_or_exit();
    }
    options
}
