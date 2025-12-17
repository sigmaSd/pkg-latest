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

fn extract_npm_parts(package_name: &str) -> (&str, &str) {
    if package_name.starts_with('@') {
        // For scoped packages like @scope/package/subpath
        let parts: Vec<&str> = package_name.splitn(3, '/').collect();
        if parts.len() >= 3 {
            let base_len = parts[0].len() + 1 + parts[1].len();
            (&package_name[..base_len], &package_name[base_len..])
        } else {
            (package_name, "")
        }
    } else if let Some(slash_pos) = package_name.find('/') {
        // For non-scoped packages with subpath
        (&package_name[..slash_pos], &package_name[slash_pos..])
    } else {
        // No subpath
        (package_name, "")
    }
}

fn extract_jsr_parts(package_name: &str) -> (&str, &str) {
    // JSR packages always start with @
    let parts: Vec<&str> = package_name.splitn(3, '/').collect();
    if parts.len() >= 3 {
        let base_len = parts[0].len() + 1 + parts[1].len();
        (&package_name[..base_len], &package_name[base_len..])
    } else {
        (package_name, "")
    }
}

fn get_npm_latest_version(package_name: &str) -> Result<String, Box<dyn std::error::Error>> {
    let (base_package, subpath) = extract_npm_parts(package_name);

    let url = format!("https://registry.npmjs.org/{}", base_package);
    let mut response = ureq::get(&url)
        .header("Accept", "application/json")
        .call()?;
    let package_info: NpmPackageInfo = response.body_mut().read_json()?;

    Ok(format!(
        "npm:{}@{}{}",
        base_package, package_info.dist_tags.latest, subpath
    ))
}

fn get_jsr_latest_version(package_name: &str) -> Result<String, Box<dyn std::error::Error>> {
    let (base_package, subpath) = extract_jsr_parts(package_name);

    let url = format!("https://jsr.io/{}/meta.json", base_package);
    let mut response = ureq::get(&url)
        .header("Accept", "application/json")
        .call()?;
    let package_info: JsrPackageInfo = response.body_mut().read_json()?;

    Ok(format!(
        "jsr:{}@{}{}",
        base_package, package_info.latest, subpath
    ))
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
            Ok(result) => {
                println!("{}", result);
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
            Ok(result) => {
                println!("{}", result);
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
        match get_npm_latest_version(input) {
            Ok(result) => {
                println!("{}", result);
            }
            Err(e) => {
                eprintln!("Error: Failed to get npm version for {}: {}", input, e);
                std::process::exit(1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_npm_scoped_with_subpath() {
        assert_eq!(
            extract_npm_parts("@google/gemini-cli/gemini"),
            ("@google/gemini-cli", "/gemini")
        );
    }

    #[test]
    fn test_npm_scoped_with_multiple_subpaths() {
        assert_eq!(extract_npm_parts("@az/e/f/c"), ("@az/e", "/f/c"));
    }

    #[test]
    fn test_npm_scoped_without_subpath() {
        assert_eq!(
            extract_npm_parts("@google/gemini-cli"),
            ("@google/gemini-cli", "")
        );
    }

    #[test]
    fn test_npm_non_scoped_with_subpath() {
        assert_eq!(extract_npm_parts("lodash/fp/map"), ("lodash", "/fp/map"));
    }

    #[test]
    fn test_npm_non_scoped_without_subpath() {
        assert_eq!(extract_npm_parts("lodash"), ("lodash", ""));
    }

    #[test]
    fn test_jsr_with_subpath() {
        assert_eq!(
            extract_jsr_parts("@sigma/bisect/mod"),
            ("@sigma/bisect", "/mod")
        );
    }

    #[test]
    fn test_jsr_with_multiple_subpaths() {
        assert_eq!(
            extract_jsr_parts("@scope/pkg/a/b/c"),
            ("@scope/pkg", "/a/b/c")
        );
    }

    #[test]
    fn test_jsr_without_subpath() {
        assert_eq!(extract_jsr_parts("@sigma/bisect"), ("@sigma/bisect", ""));
    }
}
