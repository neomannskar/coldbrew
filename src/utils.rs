use std::fs::{self, File};
use std::io::{Result, Write};
use std::path::PathBuf;

pub fn create_lotus_project(project_name: &str) -> Result<()> {
    let project_path = PathBuf::from(project_name);
    fs::create_dir(&project_path)?;

    let mut toml_path = project_path.clone();
    toml_path.push("coldbrew.toml");

    let mut file = File::create(&toml_path)?;

    let toml_contents = format!(
        r#"[package]
name = "{}"
version = "0.1.0"
description = ""
authors = ["you@example.com"]

[dependencies]
"#,
        project_name
    );

    file.write_all(toml_contents.as_bytes())?;

    println!("Created project at {}", project_path.display());

    Ok(())
}
