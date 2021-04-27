#![allow(dead_code)]
mod channel;
mod package;
mod parse;
mod project;

use channel::Channel;
use clap::{crate_authors, crate_description, crate_name, crate_version, AppSettings, Clap};
use color_eyre::Result;
use std::path::Path;

#[derive(Clap)]
#[clap(
    name = crate_name!(),
    version = crate_version!(),
    author = crate_authors!(),
    about = crate_description!(),
    setting = AppSettings::ColoredHelp,
    setting = AppSettings::DeriveDisplayOrder,
)]
pub(crate) struct Cli {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap)]
#[clap(
    name = crate_name!(),
    version = crate_version!(),
    author = crate_authors!(),
    about = crate_description!(),
    setting = AppSettings::ColoredHelp,
    setting = AppSettings::DeriveDisplayOrder,
)]
pub(crate) enum SubCommand {
    Search(Search),
    Project(Project),
}

/// Search individual packages
#[derive(Clap)]
pub(crate) struct Search {
    /// search term to filter packages with
    #[clap(required = true)]
    pub(crate) search_term: String,
    /// the release channel to find packages from
    #[clap(short='c', long="channel", possible_values=&["dev", "alpha", "beta", "rc", "stable"], default_value="alpha")]
    pub(crate) channel: Channel,
}

/// Search packages present in a gradle project
#[derive(Clap)]
pub(crate) struct Project {
    /// Project path
    #[clap(required = true)]
    pub(crate) project_path: String,
    /// the release channel to find packages from
    #[clap(short='c', long="channel", possible_values=&["dev", "alpha", "beta", "rc", "stable"], default_value="alpha")]
    pub(crate) channel: Channel,
}

fn search_package(search_term: String, channel: Channel) -> Result<()> {
    let packages = crate::parse::parse(&search_term, channel)?;

    if packages.is_empty() {
        println!("No results found!");
    } else {
        for package in packages.iter() {
            println!("{}", package);
        }
    };

    Ok(())
}

fn search_project(project_path: String, channel: Channel) -> Result<()> {
    /*
     * 1. Handle OS differences and reach project_path
     * 2. Extract dependencies using ./gradlew dependencies
     * 3. Parse dependencies and create a list
     * 4. Delete temp dependencies file
     * 5. Pass the list to search_package() along with the channel specified
     */
    let path = Path::new(&project_path);
    crate::project::parse(path, channel)?;

    Ok(())
}

fn main() -> Result<()> {
    pretty_env_logger::init();
    color_eyre::install()?;
    let cli = Cli::parse();

    match cli.subcmd {
        SubCommand::Search(search) => search_package(search.search_term, search.channel)?,
        SubCommand::Project(project) => search_project(project.project_path, project.channel)?,
    };

    Ok(())
}
