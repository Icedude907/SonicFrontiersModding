use clap::{Parser, Subcommand, Args};

#[derive(Parser, Debug)]
#[command(author, about, subcommand_negates_reqs = true)]
pub struct Arguments{
    #[arg(long/*, required = true*/)]
    /// Prints the current configuration of the tool before executing tasks.
    pub check_config: bool,

    // #[arg(required_unless_present("show_config"))]
    #[command(subcommand)]
    pub mode: Option<ProgramMode>,
}

#[derive(Subcommand, Debug, PartialEq, Eq)]
#[command(rename_all = "kebab_case")]
pub enum ProgramMode{
    /// Unpack all pacs recursively in the specified folder(s) to './build/unpac'
    Unpack(CommandUnpack),
    /// Extract the 'text' folder from the game (will not override your changes).
    ExtractText,
    /// Compile './assets/text' and then repack everything in './build/repac'.
    Compile,
    /// Export the currently compiled mod as a zip file
    Export,
}

#[derive(Args, Debug, PartialEq, Eq)]
pub struct CommandUnpack{
    /// Where to look. If relative paths, checks in <frontiers>/
    pub paths: Vec<std::path::PathBuf>,

    // no_recurse: bool,
}