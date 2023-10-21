#![allow(unused_parens, non_upper_case_globals)] // Style
#![allow(unused_variables, unused_import_braces, unused_imports, dead_code)] // Laziness

use std::{path::{PathBuf, Path}, process::{Command, Stdio}, io::Stdout};

use walkdir::WalkDir;

use crate::pac::extract_pac;
mod pac;


const unpacker      : &'static str = r#"a:\Games\Not Emu\Sonic\Frontiers\Modding Tools\HedgeLib-dev-21_10_2023\bin\HedgeArcPack.exe"#;
const text_extractor: &'static str = r#"a:\Games\Not Emu\Sonic\Frontiers\Modding Tools\PuyoTextEditor-2.0.6\PuyoTextEditor.exe"#;
const frontiers_data: &'static str = r#"A:\Games\Not Emu\Steam\steamapps\common\SonicFrontiers\image\x64\raw\"#;

// Out mirrors frontiers' directory structure.
const out_assets    : &'static str = "assets";
const out_pac       : &'static str = "build";
const path_text     : &'static str = "text";

fn main() {
    // Currently, extracts all files in the text folder.
    let proj_root = find_root().expect("Could not find the project's root folder (containing mod.ini)");
    println!("Project root: {}", proj_root.display());

    let in_text_pac = PathBuf::from(frontiers_data).join(path_text);
    let out_text_pac = PathBuf::from(&proj_root).join(out_pac).join(path_text);
    println!("Extracting all text pacs into: {}\n", out_text_pac.display());
    extract_pac(&in_text_pac, &out_text_pac);
    let in_texts = out_text_pac;
    let out_texts = PathBuf::from(&proj_root).join(out_assets).join(path_text);
    println!("Extracting all text files into: {}\n", out_texts.display());
    extract_text(&in_texts, &out_texts);
}


/// Moves build/text/*.pac/*.cnvrs-text
/// Into assets/text/*.pac/*.xml
fn extract_text(input: &Path, output: &Path){
    std::fs::create_dir_all(output).unwrap();

    // let entries = std::fs::read_dir(input).unwrap();
    for entry in WalkDir::new(input).into_iter().skip(1){
        let entry = entry.unwrap();
        let abspath = entry.path(); // Source
        let relpath = abspath.strip_prefix(input).unwrap();
        let meta = entry.metadata().unwrap();

        let mut destination = PathBuf::from(output).join(&relpath);

        if destination.extension().is_some_and(|x| x != "cnvrs-text"){
            println!("SKIP (Not Text): {}", destination.display());
        }

        let dst_meta = std::fs::metadata(&destination);
        if meta.is_dir() {
            // TODO: Filter out file exists errors
            let _ = std::fs::create_dir(destination);
        }else if meta.is_file(){
            let txt_extract = PathBuf::from(text_extractor);
            if dst_meta.is_ok_and(|x| x.is_dir()) {
                println!("SKIP (Exists): {}", relpath.display());
                continue;
            }
            // There ought to be a better way to do this - this is just rediculous.
            destination.set_extension({
                let mut extension = destination.extension().map(|x| x.to_str().unwrap()).unwrap_or_default().to_string();
                extension.push_str(".xml");
                extension
            });

            println!("TEXT: {} > {}", relpath.display(), destination.display());

            // TODO: Multithread / async this.
            let child = Command::new(txt_extract).arg(abspath).args(["-o", destination.to_str().unwrap()])
                .spawn().unwrap();
            let output = child.wait_with_output().expect("Failed to wait on kids");
        }
    }
}

/// Root directory of the project (contains mod.ini)
fn find_root() -> Result<PathBuf, ()>{
    let mut current = std::env::current_dir().expect("Don't know current directory. Cannot find project root.");
    println!("{}", current.display());
    while current.pop() {
        current.push("mod.ini");
        if std::fs::metadata(current.as_path()).map(|x| x.is_file()).unwrap_or(false) {
            current.pop();
            return Ok(current);
        }
        current.pop();
    }
    return Err(());
}