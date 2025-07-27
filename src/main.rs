mod cli;
mod utils;

use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
struct ColdbrewToml {
    package: Package,
    dependencies: Option<HashMap<String, String>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Package {
    name: String,
    version: String,
    description: String,
    authors: Vec<String>,
}

fn read_coldbrew_toml(path: &str) -> Result<ColdbrewToml, Box<dyn std::error::Error>> {
    let toml_str = fs::read_to_string(path)?;
    let config: ColdbrewToml = toml::from_str(&toml_str)?;
    Ok(config)
}

fn write_coldbrew_toml(path: &str, config: &ColdbrewToml) -> Result<(), Box<dyn std::error::Error>> {
    let toml_str = toml::to_string_pretty(config)?;
    fs::write(path, toml_str)?;
    Ok(())
}

fn get_project_file_path() -> std::io::Result<PathBuf> {
    let mut path = env::current_dir()?;
    path.push("coldbrew.toml");
    Ok(path)
}

fn get_project_name_from_dir() -> Option<String> {
    let current_dir = env::current_dir().ok()?;
    let dir_name = current_dir.file_name()?.to_str()?;
    Some(dir_name.to_string())
}

fn main() -> std::io::Result<()> {
    let matches = cli::parse();

    match matches.subcommand() {
        Some(("new", sub_m)) => {
            let name = sub_m.get_one::<String>("name").unwrap();
            utils::create_lotus_project(name)?;

            println!("    {} binary package `{}`", "Creating".bold().green(), name);
            println!("        {} coldbrew.toml", "Note".bold().blue());
        }

        Some(("build", _)) => {
            let path = get_project_file_path()?;
            println!("    {}  {} ...", "Building".bold().green(), path.display());

            match read_coldbrew_toml(path.to_str().unwrap()) {
                Ok(config) => {
                    println!("Project: {}", config.package.name.bold());
                    if let Some(deps) = config.dependencies {
                        for (name, version) in deps {
                            println!("Dependency: {} v{}", name.cyan(), version);
                        }
                    } else {
                        println!("No dependencies.");
                    }
                }
                Err(e) => {
                    eprintln!("{}: {}", "    Error reading coldbrew.toml".bold().red(), e);
                }
            }
        }

        Some(("run", _)) => {
            println!("{}", "Running Lotus project...".bold().green());
            // You could spawn the binary here.
        }

        Some(("add", sub_m)) => {
            let pkg = sub_m.get_one::<String>("package").unwrap();
            println!("Adding package: {}", pkg);

            let path = get_project_file_path()?;

            let mut config = match read_coldbrew_toml(path.to_str().unwrap()) {
                Ok(cfg) => cfg,
                Err(e) => {
                    eprintln!("{}: {}", "Failed to read coldbrew.toml".red(), e);
                    return Ok(());
                }
            };

            let deps = config.dependencies.get_or_insert_with(HashMap::new);
            deps.insert(pkg.to_string(), "0.1.0".to_string());

            if let Err(e) = write_coldbrew_toml(path.to_str().unwrap(), &config) {
                eprintln!("{}: {}", "Failed to update TOML".red(), e);
            } else {
                println!("Added dependency: {} v{}", pkg, "0.1.0");
            }
        }

        Some(("install", _)) => {
            println!("{}", "Installing dependencies...".bold().green());
            // Here you’d fetch the actual packages and store them locally
        }

        Some(("publish", _)) => {
            println!("{}", "Publishing package...".bold().green());
            // Handle registry upload
        }

        _ => unreachable!(),
    }

    Ok(())
}
