#![allow(unused_parens, non_upper_case_globals)] // Style
#![allow(unused_variables, unused_import_braces, unused_imports, dead_code, irrefutable_let_patterns, unused_braces)] // Laziness

use clap::{Parser, CommandFactory};

mod pac;
mod args;
mod config;
mod util;
mod text;

use crate::{config::{PAT, CFG}, args::ProgramMode};

fn main() {
    let args = args::Arguments::parse();
    println!("{:#?}", args);

    // Semi-clap limitation, no subcommand required-unless-defined.
    if args.check_config == false && args.mode.is_none() {
        println!("No work to do. Printing help message:");
        let _ = args::Arguments::command().print_long_help();
        std::process::exit(2);
    }

    config::update_config_all();

    println!("{:#?}", &**CFG); // Unwraps the SuperLazy
    if args.check_config {
        if args.mode.is_some() {
            util::wait_for_press_enter();
        }else{
            std::process::exit(0);
        }
    }

    let proj_assets = CFG.proj_root.join(&PAT.assets);
    let proj_unpac  = CFG.proj_root.join(&PAT.unpack);
    let proj_repac  = CFG.proj_root.join(&PAT.repack);
    let proj_raw    = CFG.proj_root.join(&PAT.raw);

    let frontiers_text = CFG.frontiers_data.join(&PAT.text);
    let proj_text  = proj_assets.join(&PAT.text);
    let unpac_text = proj_unpac.join(&PAT.text);
    let repac_text = proj_repac.join(&PAT.text);
    match args.mode.unwrap(){
        ProgramMode::ExtractText => {
            println!("JOB START 1/2: Unpacking <frontiers>/{} into build/unpac/{}:", &PAT.text.display(), &PAT.text.display());
            pac::un_pac(&frontiers_text, &unpac_text);
            println!("JOB START 2/2: Extracting build/unpac/{} into assets/text:", &PAT.text.display());
            text::extract_text(&unpac_text, &proj_text);
        }
        ProgramMode::CompileText => {
            println!("JOB START 1/2: Compiling assets/text into build/repac/{}", &PAT.text.display());
            // TODO: Interleave other documents in the pacs automatically to keep a complete file structure.
            text::compile_text(&proj_text, &repac_text);
            // This is a much more general job
            println!("JOB START 2/2: Repacking build/repac/* into raw/*");
            pac::re_pac(&proj_repac, &proj_raw);
        }
        _ => {unimplemented!()}
    }
}