#![allow(dead_code)]
#![allow(unused_variables)]
use crate::channel::Channel;
use color_eyre::Result;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::process::Command;

fn read_file(path: &str) -> Result<()> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    print!("{}", contents);

    Ok(())
}

#[cfg(windows)]
fn generate_dependencies(path: &Path) -> Result<()> {
    let output_path = path.canonicalize()?.join("dependencies.txt");

    let command = Command::new("cmd")
        .current_dir(path)
        .arg("gradlew.bat -q androidDependencies > dependencies.txt")
        .output()
        .expect("failed to execute process");

    Ok(())
}

#[cfg(unix)]
fn generate_dependencies(path: &Path) -> Result<()> {
    let output_path = path.canonicalize()?.join("dependencies.txt");

    let command = Command::new("sh")
        .current_dir(path)
        .arg("-c")
        .arg("./gradlew -q androidDependencies > dependencies.txt")
        .output()
        .expect("failed to execute process");

    Ok(())
}

/// The entrypoint for project module which handles returns a list of packages.
pub(crate) fn parse(path: &Path, channel: Channel) -> Result<()> {
    // TODO: Testing required on windows
    generate_dependencies(path)?;
    Ok(())
}
