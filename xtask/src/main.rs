mod cmds;
use clap::{crate_authors, crate_description, crate_name, crate_version, AppSettings, Clap};

use color_eyre::eyre::{eyre, Result};
#[derive(Clap)]
#[clap(
    name = crate_name!(),
    version = crate_version!(),
    author = crate_authors!(),
    about = crate_description!(),
    setting = AppSettings::ColoredHelp,
    setting = AppSettings::DeriveDisplayOrder,
    setting = AppSettings::SubcommandRequiredElseHelp,
)]
struct Opts {
    #[clap(subcommand)]
    subcommand: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
    Sync(Sync),
}

#[derive(Clap)]
#[clap(setting = AppSettings::ColoredHelp)]
struct Sync {}

fn main() -> Result<()> {
    color_eyre::install()?;
    let opts = Opts::parse();
    match opts.subcommand {
        SubCommand::Sync(_) => {}
    };
    Ok(())
}
