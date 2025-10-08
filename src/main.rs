use serde::Deserialize;
use std::io::{self, Read};

#[derive(Deserialize)]
struct NpmPackageInfo {
    #[serde(rename = "dist-tags")]
    dist_tags: DistTags,
}

#[derive(Deserialize)]
struct DistTags {
    latest: String,
}

fn get_latest_version(package_name: &str) -> Result<String, Box<dyn std::error::Error>> {
    let url = format!("https://registry.npmjs.org/{}", package_name);

    let response = ureq::get(&url).set("Accept", "application/json").call()?;

    let package_info: NpmPackageInfo = response.into_json()?;
    Ok(package_info.dist_tags.latest)
}

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read from stdin");

    let input = input.trim();

    // Assume input is npm:package_name
    if let Some(package_name) = input.strip_prefix("npm:") {
        match get_latest_version(package_name) {
            Ok(version) => {
                println!("npm:{}@{}", package_name, version);
            }
            Err(e) => {
                eprintln!("Error: Failed to get version for {}: {}", package_name, e);
                std::process::exit(1);
            }
        }
    } else {
        eprintln!("Error: Input must start with 'npm:'");
        std::process::exit(1);
    }
}
