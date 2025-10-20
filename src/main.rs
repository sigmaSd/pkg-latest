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

fn get_npm_latest_version(
    package_name: &str,
) -> Result<(String, String), Box<dyn std::error::Error>> {
    // Extract the base package name (without subpath) for the registry lookup
    let base_package = if package_name.starts_with('@') {
        // For scoped packages like @scope/package/subpath, we need @scope/package
        // This is always the first two slash-separated parts
        let parts: Vec<&str> = package_name.splitn(3, '/').collect();
        if parts.len() >= 2 {
            format!("{}/{}", parts[0], parts[1])
        } else {
            package_name.to_string()
        }
    } else if let Some(slash_pos) = package_name.find('/') {
        // For non-scoped packages with subpath, take everything before first slash
        package_name[..slash_pos].to_string()
    } else {
        // No subpath, use as-is
        package_name.to_string()
    };

    let url = format!("https://registry.npmjs.org/{}", base_package);
    let mut response = ureq::get(&url)
        .header("Accept", "application/json")
        .call()?;
    let package_info: NpmPackageInfo = response.body_mut().read_json()?;
    Ok((base_package, package_info.dist_tags.latest))
}

fn get_jsr_latest_version(
    package_name: &str,
) -> Result<(String, String), Box<dyn std::error::Error>> {
    // Extract the base package name (without subpath) for the registry lookup
    // JSR packages always start with @ and have format @scope/package/subpath
    // We need to extract @scope/package (first two slash-separated parts)
    let base_package = {
        let parts: Vec<&str> = package_name.splitn(3, '/').collect();
        if parts.len() >= 2 {
            format!("{}/{}", parts[0], parts[1])
        } else {
            package_name.to_string()
        }
    };

    let url = format!("https://jsr.io/{}/meta.json", base_package);
    let mut response = ureq::get(&url)
        .header("Accept", "application/json")
        .call()?;
    let package_info: JsrPackageInfo = response.body_mut().read_json()?;
    Ok((base_package, package_info.latest))
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
            Ok((base_package, version)) => {
                println!("npm:{}@{}", base_package, version);
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
            Ok((base_package, version)) => {
                println!("jsr:{}@{}", base_package, version);
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
