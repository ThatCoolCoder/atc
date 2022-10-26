use argparse;

pub struct Options {
    pub level_name: String,
    pub show_description: bool,
}

pub fn parse_args(level_names: &Vec<&str>) -> Options {
    // Convert command line args into an Options struct

    // Init default options
    let mut options = Options {
        level_name: "Default".to_string(),
        show_description: false,
    };

    let level_names_string = level_names.join(", ");
    let level_names_help = format!("Scenario name. Available options: {level_names_string}");

    // Set up argparser and use it
    {
        let mut parser = argparse::ArgumentParser::new();
        parser.refer(&mut options.level_name).add_argument(
            "scenario",
            argparse::Store,
            &level_names_help,
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
