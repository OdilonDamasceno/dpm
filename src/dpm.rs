use crate::tools;
use std::env;
use std::io::{stdin, stdout, Result, Write};

macro_rules! json_option {
  ($option:expr) => {
    if $option.is_empty() {
      Option::default()
    } else {
      Option::from($option)
    }
  };
}

macro_rules! current_path {
  ($is_current_path:expr, $path_name:expr) => {
    match $is_current_path {
      false => $path_name,
      true => ".".to_string(),
    }
  };
}

pub fn add(args: &mut [String]) -> Result<()> {
  match args {
    [] => {
      let res = tools::json::write(current_path!(true, "".to_string()));
      if res.is_err() {
        println!("Failed to write dependencies");
      }
    }
    _ => {
      for dep in args {
        let res = tools::json::add_dep(dep);
        if res.is_err() {
          println!("Failed to add {}", dep);
        }
        tools::json::write(current_path!(true, "".to_string()))?;
      }
    }
  };
  Ok(())
}

pub fn init(fold_name: Option<String>, current_path: bool) -> Result<()> {
  let current_dir = env::current_dir()?;
  let def_name: String = fold_name.clone().unwrap_or(
    current_dir
      .iter()
      .last()
      .unwrap()
      .to_str()
      .unwrap()
      .to_string(),
  );

  let name = read_line(format!("Package name: ({}) ", def_name));
  let description = read_line("Description: ".to_string());
  let author = read_line("Author: ".to_string());
  let license = read_line("License: (MIT) ".to_string());
  let version = read_line("Version: (0.0.1) ".to_string());
  let main = read_line("Main file: (src/mod.ts) ".to_string());
  tools::json::create(
    if name.is_empty() {
      def_name.to_string()
    } else {
      name
    },
    json_option!(author),
    json_option!(description),
    json_option!(license),
    json_option!(version),
    json_option!(main),
    current_path,
    Option::from(def_name),
  )?;
  Ok(())
}

pub fn create(project: &str) -> std::io::Result<()> {
  println!(
    "{} Creating a new project...",
    console::style("[ ]").green().bold()
  );
  let res = std::fs::create_dir(project);
  match res.is_err() {
    true => println!(
      "{} Failed to create this project!\nError: {}",
      console::style("[X]").red().bold(),
      res.unwrap_err()
    ),
    false => {
      init(Option::from(project.to_string()), false)?;
      tools::json::write(current_path!(false, project.to_string()))?;
      println!(
        r#"
All done!
    Now run:

    $ cd {}
    $ dpm run"#,
        project
      );
    }
  }
  Ok(())
}

pub fn run(script: Option<&String>) -> Result<()> {
  match script.is_none() {
    true => {
      let args: Vec<String> = tools::json::get_args()?;
      let main: String = tools::json::read("package-info.json")?["main"]
        .to_string()
        .replace("\"", "");
      let mut shell = std::process::Command::new("deno");
      shell.arg("run");
      shell.args(args);
      shell.arg("--import-map=.modules.json");
      shell.arg(main);
      shell.spawn().expect("Failed to run");
    }
    false => {
      let scripts = tools::json::get_scripts()?;
      match scripts.contains_key(script.unwrap()) {
        true => {
          let command: String = scripts.get(script.unwrap()).unwrap().replace("\"", "");
          let mut command_split: Vec<&str> = command.split(" ").collect();
          let mut shell = std::process::Command::new(command_split[0]);
          command_split.remove(0);
          shell.args(command_split.into_iter());
          shell.spawn().expect("Failed to exec this script");
        }
        _ => println!("{} Script not found", console::style("[X]").red().bold()),
      }
    }
  }

  Ok(())
}

fn read_line(message: String) -> String {
  print!("{}", message);
  stdout().flush().unwrap();
  let mut name = String::new();
  stdin().read_line(&mut name).expect("Failed to read");
  name.trim().to_string()
}
