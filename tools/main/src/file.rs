use std::path::Path;

use walkdir::WalkDir;


/// Intended to copy files from the assets folder that dont require additional processing
/// to the repac folder for being mixed in with the other pac files.
/// WARNING: Files of the same name in repac will be overridden!
pub fn copy_directory_recursive_extension_blacklist(input: &Path, output: &Path, exclude: &[&str]){
    // TODO: Don't copy redundancies (same timestamp)
    // TODO: Hardlink if possible (no links on exfat is a crime)
    std::fs::create_dir_all(output).unwrap();

    for entry in WalkDir::new(input).into_iter().skip(1){
        let entry = entry.unwrap();
        let abspath = entry.path(); // Source
        let relpath = abspath.strip_prefix(input).unwrap();
        let destination = output.join(relpath);
        if entry.file_type().is_dir(){
            let _ = std::fs::create_dir_all(destination);
            continue;
        }
        let dontcopy = entry.file_name().to_str().unwrap().split_once('.').map(|(_, x)| exclude.iter().any(|ext| x.contains(ext))).unwrap_or(false);
        if entry.file_type().is_file() && !dontcopy{
            println!("    Copying {}", relpath.display());
            std::fs::copy(&abspath, &destination).unwrap();
        }
    }
}

/// Returns true of source is *newer* than dest.
/// If no date data is accessible, defaults to true (you should do work)
pub fn compare_date_paths(source: &Path, dest: &Path) -> std::io::Result<bool>{
    // TODO: In the future may use a cache for destinations rather than file system information to reduce seeks.
    let srcmod = source.metadata()?.modified();
    let dstmod = dest.metadata()?.modified(); // TODO: If this is being checked against multiple times there's an inefficiency here.
    // let dt: DateTime<Utc> = st.clone().into();
    if let (Ok(srcmod), Ok(dstmod)) = (srcmod, dstmod){
        let newer = srcmod > dstmod;
        return Ok(newer);
    }

    return Ok(true);
}