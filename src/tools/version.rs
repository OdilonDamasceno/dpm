use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
struct Version {
  latest: String,
}

pub fn get_latest_version(module: &str) -> Result<String, Box<dyn std::error::Error>> {
  let deno_cdn = format!("http://cdn.deno.land/{}/meta/versions.json", module);
  let res = reqwest::blocking::get(deno_cdn)?;
  let body: Version = res.json()?;
  Ok(body.latest)
}
