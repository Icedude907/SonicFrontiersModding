
use std::{path::{PathBuf, Path}, process::Command};

use walkdir::WalkDir;

use crate::{config::CFG, file, util::if2};

const text_ext      : &'static str = "xml";

/// Compiles assets/text/*.pac/*.cnvrs-text.xml
/// Into build/repac/text/*.pac/*.cnvrs-text
/// Only runs the command on changed files (using filesystem date modified)
pub fn compile_text(input: &Path, output: &Path){
    std::fs::create_dir_all(output).unwrap();

    for entry in WalkDir::new(input).into_iter().skip(1){
        let entry = entry.unwrap();
        let abspath = entry.path(); // Source
        let relpath = abspath.strip_prefix(input).unwrap();
        let srcmeta = entry.metadata().unwrap();

        let destination = PathBuf::from(output).join(&relpath);
        if entry.file_type().is_dir(){
            let _ = std::fs::create_dir_all(destination);
            continue;
        }
        if srcmeta.is_file() && abspath.extension().is_some_and(|x| x.to_str().unwrap().contains("xml") ) {
            let destination = destination.with_extension("");
            let shouldwork = file::compare_date_paths(abspath, &destination).unwrap();

            if(!shouldwork){
                println!("SKIP (No change): {}", relpath.display());
                continue;
            }

            println!("COMPILE: {} > {}",
                relpath.display(),
                destination.strip_prefix(output.parent().and_then(|x|x.parent()).unwrap_or(output)).unwrap().display()
            );

            // TODO: Multithread / async this.
            let txt_extract = PathBuf::from(&CFG.tool_text);
            let child = Command::new(txt_extract)
                .arg(abspath)
                .args(["--format", "cnvrs-text"])
                .args(["-o", destination.to_str().unwrap()])
                .spawn().unwrap();
            let _output = child.wait_with_output().expect("Failed to wait on kids");
        }else{ /* skip */ }

    }
}

/// Extracts build/extract/text/*.pac/*.cnvrs-text
/// Into assets/text/*.pac/*.cnvrs-text.xml
pub fn extract_text(input: &Path, output: &Path){
    std::fs::create_dir_all(output).unwrap();

    for entry in WalkDir::new(input).into_iter().skip(1){
        let entry = entry.unwrap();
        let abspath = entry.path(); // Source
        let relpath = abspath.strip_prefix(input).unwrap();
        let meta = entry.metadata().unwrap();

        let mut destination = PathBuf::from(output).join(&relpath);

        if destination.extension().is_some_and(|x| x != "cnvrs-text"){
            println!("SKIP (Not Text): {}", destination.display());
            continue;
        }
        let _dst_meta = std::fs::metadata(&destination);

        if meta.is_dir() {
            // TODO: Filter out file exists errors
            let _ = std::fs::create_dir(destination);
        }else if meta.is_file(){
            // There ought to be a better way to do this - this is just rediculous.
            destination.set_extension({
                let mut extension = destination.extension().map(|x| x.to_str().unwrap()).unwrap_or_default().to_string();
                extension.push_str(".xml");
                extension
            });
            let dst_meta = std::fs::metadata(&destination);

            if dst_meta.is_ok() {
                println!("SKIP (Exists): {}", relpath.display());
                continue;
            }

            println!("TEXT: {} > {}", relpath.display(), destination.display());

            // TODO: Multithread / async this.
            let txt_extract = PathBuf::from(&CFG.tool_text);
            let child = Command::new(txt_extract).arg(abspath).args(["-o", destination.to_str().unwrap()])
                .spawn().unwrap();
            let _output = child.wait_with_output().expect("Failed to wait on kids");
        }
    }
}