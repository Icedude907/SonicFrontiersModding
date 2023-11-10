use std::{path::Path, fs::File, io::{Write, Read}};

use walkdir::WalkDir;
use zip::{write::FileOptions, CompressionMethod, result::ZipResult};

use crate::util::if2;


/// Names in recurse_files are added to the root of the zip, and subfolders are recursively placed in their parents.
pub fn compress(dst: &Path, recurse_files: &[&Path]) -> ZipResult<()>{
    let file = File::create(dst)?;
    let mut zip = zip::ZipWriter::new(file);
    let options = FileOptions::default()
        .compression_method(CompressionMethod::Zstd)
        .compression_level(Some(15));

    let mut buffer: Vec<u8> = Vec::new();
    
    for root in recurse_files{
        let meta = std::fs::metadata(root).unwrap();
        let is_root_file = meta.is_file();
        for entry in WalkDir::new(root).into_iter() {
            let entry = entry.unwrap();
            let srcpath = entry.path();
            let zippath = srcpath.strip_prefix(root.parent().unwrap()).unwrap();

            let ftype = entry.file_type();

            // Explicitly write directories
            if ftype.is_dir() || ftype.is_symlink(){ // idk about symlinks if it'll work
                zip.add_directory(zippath.to_str().unwrap(), options)?;
            }else if ftype.is_file(){
                zip.start_file(zippath.to_str().unwrap(), options)?;
                let mut infile = File::open(srcpath)?;
                infile.read_to_end(&mut buffer)?;
                zip.write(&buffer)?;
                buffer.clear();
            }
        }
    }
    zip.finish()?;

    return Ok(());
}