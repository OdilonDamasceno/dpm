mod dpm;
mod tools;
use std::env;
use std::io::Result;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

macro_rules! show_help {
    () => {
        println!(
            r#"Deno package manager

USAGE:
    dpm <command>

COMMANDS:
    add              Write .modules.json from package-info.json
    add <package>    Add <package> to package-info.json and .modules.json
    init             Initialize deno project
    help             Show help
    version          Show version
    create <name>    Create a new project
    run              Run the default script
    run <script>     Run the <script>

dpm@{}"#,
            VERSION
        );
    };
}

fn main() -> Result<()> {
    let mut args: Vec<_> = env::args().collect();
    let mut options: &str = "";
    if args.len() >= 2 {
        options = &args[1];
    }

    match args.len() {
        1 => show_help!(),
        _ => match options {
            "init" => dpm::init(Option::default(), true)?,
            "help" => show_help!(),
            "add" => dpm::add(&mut args[2..])?,
            "version" => println!("dpm@{}", VERSION),
            "create" => dpm::create(&args[2])?,
            "run" => match args.len() > 2 {
                true => dpm::run(Option::from(&args[2]))?,
                false => dpm::run(Option::default())?,
            },
            _ => show_help!(),
        },
    };
    Ok(())
}
