use std::{path::{Path, PathBuf}, process::Command};

use crate::unpacker;



pub fn extract_pac(input: &Path, output: &Path){
    std::fs::create_dir_all(output).unwrap();

    let entries = std::fs::read_dir(input).unwrap();
    for entry in entries{
        let entry = entry.unwrap();
        let relpath = PathBuf::from(entry.file_name());
        let source = entry.path(); // abspath
        let meta = entry.metadata().unwrap();
        if meta.is_file() && source.extension().is_some_and(|x| x == "pac") {
            let pac_extract = PathBuf::from(unpacker);
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

            let child = Command::new(pac_extract).arg(source).arg(destination)
                .spawn().unwrap();
            let output = child.wait_with_output().expect("Failed to wait on kids");
        }else{
            println!("SKIP: {}", relpath.display())
        }
    }
}