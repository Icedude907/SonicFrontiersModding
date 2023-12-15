#![allow(unused_parens, non_upper_case_globals)] // Style
#![allow(unused_variables, unused_import_braces, unused_imports, dead_code, irrefutable_let_patterns, unused_braces, unreachable_patterns, unused_macros)] // Laziness

use clap::{Parser, CommandFactory};

mod pac;
mod args;
mod config;
mod util;
mod text;
mod file;
mod zip;

use crate::{config::{PAT, CFG}, args::ProgramMode};

// TODO Repacking requires all files to be present (even if unmodified)
// The mod loader could fix this but it doesnt
// Need a tool to extract associated folders in assets t

fn main() {
    // Argument parsing
    let args = args::Arguments::parse();

    // Clap limitation? No subcommand required-unless-defined.
    if args.check_config == false && args.mode.is_none() {
        let _ = args::Arguments::command().print_long_help();
        std::process::exit(2);
    }

    // Config settling
    config::update_config_all();
    if args.check_config {
        // println!("{:#?}", &**CFG); // Unwraps the SuperLazy
        config::print_config_valid();
        if args.mode.is_some() {
            util::wait_for_press_enter();
        }else{
            std::process::exit(0);
        }
    }

    // Subcommand execution

    let proj_assets = CFG.proj_root.join(&PAT.assets);
    let proj_unpac  = CFG.proj_root.join(&PAT.unpack);
    let proj_repac  = CFG.proj_root.join(&PAT.repack);
    let proj_build  = CFG.proj_root.join(&PAT.build);
    let proj_raw    = CFG.proj_root.join(&PAT.raw);

    let frontiers_text = CFG.frontiers_data.join(&PAT.text);
    let proj_text  = proj_assets.join(&PAT.text);
    let unpac_text = proj_unpac.join(&PAT.text);
    let repac_text = proj_repac.join(&PAT.text);
    match args.mode.unwrap(){
        ProgramMode::ExtractText => {
            println!("JOB START 1/2: Unpacking <frontiers>/{} into build/unpac/{}:", &PAT.text.display(), &PAT.text.display());
            pac::un_pac(&frontiers_text, &unpac_text, true);
            println!("JOB START 2/2: Extracting build/unpac/{} into assets/text:", &PAT.text.display());
            text::extract_text(&unpac_text, &proj_text);
        }
        ProgramMode::Compile => {
            println!("JOB START 1/3: Compiling assets/text into build/repac/{}", &PAT.text.display());
            text::compile_text(&proj_text, &repac_text);

            println!("JOB START 2/3: Copying other assets into build/repac/");
            // TODO: Remove files in destination that no longer exist in source.
            file::copy_directory_recursive_extension_blacklist(&proj_assets, &proj_repac, &[
                "ignore", // for a user
                "xml", "cnvrs-text", // text
                "needle", // archives that need another layer of packing
                "log", // Extra stuff
            ]);

            println!("JOB START 3/3: Repacking build/repac/* into raw/*");
            pac::re_pac(&proj_repac, &proj_raw);
        }
        ProgramMode::Unpack(dat) => {
            let unpac_dir = proj_unpac.join("UnpackCommand");
            println!("JOB START:");
            for p in dat.paths {
                let unpac_dir = unpac_dir.join(p.components().last().unwrap());
                println!("Unpacking {} into {}:", p.display(), unpac_dir.display());
                pac::un_pac(&p, &unpac_dir, false);
            }
        }
        ProgramMode::Export => {
            let now = chrono::Local::now().format("%F");
            let export_file = proj_build.join(format!("modexport_{}.zip", now));

            let directories = [
                "mod.ini",
                "ModCodes.hmm",
                "ModReadme.txt",
                "raw/"
            ].map(|rel| CFG.proj_root.join(rel));
            let directories: Vec<_> = directories.iter().map(|f| f.as_path()).collect();

            println!("JOB START: Exporting the following files/folders: ");
            for d in directories.iter(){
                println!("  {}", d.display());
            }
            zip::compress(&export_file, &directories).unwrap();
        }
        _ => {unimplemented!()}
    }
}