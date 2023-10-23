use std::{path::{Path, PathBuf}, process::Command};

use walkdir::WalkDir;

use crate::CFG;

/// Extracts ".pac" files contianing "_en"
pub fn un_pac(input: &Path, output: &Path){
    std::fs::create_dir_all(output).unwrap();

    let entries = std::fs::read_dir(input).unwrap();
    for entry in entries{
        let entry = entry.unwrap();
        let relpath = PathBuf::from(entry.file_name());
        let source = entry.path(); // abspath
        let meta = entry.metadata().unwrap();
        if meta.is_file() && source.extension().is_some_and(|x| x == "pac") {
            let destination = PathBuf::from(output).join(&relpath);

            if !source.components().last().unwrap().as_os_str().to_str().unwrap().contains("_en") {
                println!("SKIP (Not English): {}", relpath.display());
                continue;
            }

            if std::fs::metadata(&destination).is_ok_and(|x| x.is_dir()){
                println!("SKIP (Exists): {}", relpath.display());
                continue;
            }

            println!("UNPACK: {} > {}", relpath.display(), destination.display());

            let child = Command::new(&CFG.tool_unpacker).arg(source).arg(destination)
                .spawn().unwrap();
            let _output = child.wait_with_output().expect("Failed to wait on kids");
        }else{
            println!("SKIP: {}", relpath.display())
        }
    }
}

/// All folders ending in .pac will be compressed using HedgePack
pub fn re_pac(input: &Path, output: &Path){
    // As an aside - HedgeModManager needs some work on documentation and better asset injection.
    // This repacking process is kind of slow & doesn't allow for per file overrides.
    std::fs::create_dir_all(output).unwrap();

    for entry in WalkDir::new(input).into_iter().skip(1){
        let entry = entry.unwrap();
        let abspath = entry.path(); // Source
        let relpath = abspath.strip_prefix(input).unwrap();

        if entry.file_type().is_dir() && abspath.extension().is_some_and(|x| x == "pac"){
            let destination = PathBuf::from(output).join(&relpath);
            let _ = std::fs::create_dir_all(destination.parent().unwrap());
            println!("Packing folder: {}\n  > {}", abspath.display(), destination.display());

            // TODO: Multithread / async this.
            let child = Command::new(&CFG.tool_unpacker)
                .arg(abspath.to_str().unwrap())
                .arg(destination.to_str().unwrap())
                .arg("-P")
                .arg("-T=frontiers")
                .spawn().unwrap();
            let _output = child.wait_with_output().expect("Failed to wait on kids");
        } else {/* Skip */}
    }
}