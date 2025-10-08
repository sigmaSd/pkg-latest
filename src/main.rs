use serde::Deserialize;
use std::env;

#[derive(Deserialize)]
struct NpmPackageInfo {
    #[serde(rename = "dist-tags")]
    dist_tags: DistTags,
}

#[derive(Deserialize)]
struct DistTags {
    latest: String,
}

#[derive(Deserialize)]
struct JsrPackageInfo {
    latest: String,
}

fn get_npm_latest_version(package_name: &str) -> Result<String, Box<dyn std::error::Error>> {
    let url = format!("https://registry.npmjs.org/{}", package_name);
    let mut response = ureq::get(&url)
        .header("Accept", "application/json")
        .call()?;
    let package_info: NpmPackageInfo = response.body_mut().read_json()?;
    Ok(package_info.dist_tags.latest)
}

fn get_jsr_latest_version(package_name: &str) -> Result<String, Box<dyn std::error::Error>> {
    let url = format!("https://jsr.io/{}/meta.json", package_name);
    let mut response = ureq::get(&url)
        .header("Accept", "application/json")
        .call()?;
    let package_info: JsrPackageInfo = response.body_mut().read_json()?;
    Ok(package_info.latest)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <npm:package|jsr:package>", args[0]);
        std::process::exit(1);
    }

    let input = &args[1];

    if let Some(package_name) = input.strip_prefix("npm:") {
        match get_npm_latest_version(package_name) {
            Ok(version) => {
                println!("npm:{}@{}", package_name, version);
            }
            Err(e) => {
                eprintln!(
                    "Error: Failed to get npm version for {}: {}",
                    package_name, e
                );
                std::process::exit(1);
            }
        }
    } else if let Some(package_name) = input.strip_prefix("jsr:") {
        match get_jsr_latest_version(package_name) {
            Ok(version) => {
                println!("jsr:{}@{}", package_name, version);
            }
            Err(e) => {
                eprintln!(
                    "Error: Failed to get jsr version for {}: {}",
                    package_name, e
                );
                std::process::exit(1);
            }
        }
    } else {
        eprintln!("Error: Input must start with 'npm:' or 'jsr:'");
        std::process::exit(1);
    }
}
