use crate::tools;
use console::style;
use serde_json::Value;
use std::collections::HashMap;
use std::io::{BufReader, BufWriter, Result};
use std::{fs::File, path::Path};

macro_rules! dir_name {
  ($current_path:expr, $path:expr) => {
    match $current_path {
      true => ".".to_string(),
      false => $path.to_string(),
    }
  };
}

pub fn create(
  name: String,
  author: Option<String>,
  description: Option<String>,
  license: Option<String>,
  version: Option<String>,
  main: Option<String>,
  current_path: bool,
  fold_name: Option<String>,
) -> std::io::Result<()> {
  let file = File::create(format!(
    "{}/package-info.json",
    dir_name!(current_path, fold_name.unwrap_or(name.to_string()))
  ))?;
  let writer = BufWriter::new(file);
  let mut hash_map: HashMap<&str, Value> = HashMap::new();
  hash_map.insert(
    "author",
    Value::String(author.unwrap_or("".trim().to_string()).to_string()),
  );
  hash_map.insert(
    "description",
    Value::String(description.unwrap_or("".trim().to_string()).to_string()),
  );
  hash_map.insert(
    "licence",
    Value::String(license.unwrap_or("MIT".trim().to_string()).to_string()),
  );
  hash_map.insert(
    "version",
    Value::String(version.unwrap_or("0.0.1".trim().to_string()).to_string()),
  );
  hash_map.insert(
    "main",
    Value::String(main.unwrap_or("src/mod.ts".trim().to_string()).to_string()),
  );
  hash_map.insert("name", Value::String(name.to_string()));
  hash_map.insert("deps", Value::Object(serde_json::Map::new()));

  serde_json::to_writer_pretty(writer, &hash_map)?;
  Ok(())
}

pub fn read<P: AsRef<Path>>(path: P) -> Result<Value> {
  let file = File::open(path)?;
  let reader = BufReader::new(file);
  let v: Value = serde_json::from_reader(reader)?;
  Ok(v)
}

pub fn write(path: String) -> Result<()> {
  let file = File::create(format!("{}/.modules.json", path))?;
  let writer = BufWriter::new(file);
  let mut map = HashMap::new();
  map.insert("imports", load_modules(path)?);
  println!(
    "{} Writing dependencies to modules.json...",
    style("[]").bold().green()
  );
  serde_json::to_writer(writer, &map)?;
  Ok(())
}

fn load_modules(path: String) -> Result<HashMap<String, String>> {
  println!("{} Loading modules...", style("[]").bold().green());
  let mut map: HashMap<String, String> = HashMap::new();

  let r = read(format!("{}/package-info.json", path))?;
  let deps: &serde_json::Map<String, Value> = r["deps"].as_object().unwrap();

  for (key, value) in deps {
    map.insert(
      format!("{}/", key.to_string()),
      format!(
        "https://deno.land/x/{}{}/",
        key.to_string(),
        value.to_string().replace("\"", "")
      ),
    );
  }

  Ok(map)
}

pub fn get_args() -> std::io::Result<Vec<String>> {
  let mut args: Vec<String> = Vec::new();
  let r = read("package-info.json")?;
  for arg in r.get("args") {
    args.push(serde_json::to_string(arg)?);
  }
  Ok(args)
}

pub fn get_scripts() -> std::io::Result<HashMap<String, String>> {
  let mut hashmap: HashMap<String, String> = HashMap::new();
  let r: Value = read("package-info.json")?;
  let _scripts: &Value = &r["scripts"];

  match _scripts.is_object() {
    true => {
      let scripts = _scripts.as_object().unwrap();
      for (key, value) in scripts {
        hashmap.insert(key.to_string(), value.to_string());
      }
    }
    false => println!(
      "{} Please create the scripts value",
      style("[X]").red().bold()
    ),
  }

  Ok(hashmap)
}

pub fn add_dep(dep: &str) -> std::result::Result<(), Box<dyn std::error::Error>> {
  let mut r = read("package-info.json")?;
  println!("{} Searching for {}...", style("[]").green().bold(), dep);
  let latest = tools::version::get_latest_version(dep)?;
  println!(
    "{} Adding {}@{} to package-info...",
    style("[+]").green().bold(),
    dep,
    latest
  );
  r["deps"]
    .as_object_mut()
    .unwrap()
    .insert(dep.to_string(), Value::String(format!("@{}", latest)));
  let file = File::create("package-info.json")?;
  let mut writer = BufWriter::new(file);
  let data = serde_json::to_string_pretty(&r)?;
  use std::io::Write;
  writer.write_all(data.as_bytes())?;
  Ok(())
}
