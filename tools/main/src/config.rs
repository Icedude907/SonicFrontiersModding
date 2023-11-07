use std::path::{PathBuf, Path};

use once_cell::sync::Lazy;

use crate::util::SuperLazy;

// ----------------------------------------
// If you are utilising this project, this is the stuff you need to edit.
pub static CFG: SuperLazy<Config> = SuperLazy::new(||{
    Config {
        proj_root: PathBuf::new(),
        // Change these
        frontiers_data: PathBuf::from(r#"A:\Games\Not Emu\Steam\steamapps\common\SonicFrontiers\image\x64\raw\"#),
        tool_unpacker: r#"a:\Games\Not Emu\Sonic\Frontiers\Modding Tools\HedgeLib-dev-21_10_2023\bin\HedgeArcPack.exe"#.into(),
        tool_text: r#"a:\Games\Not Emu\Sonic\Frontiers\Modding Tools\PuyoTextEditor-2.0.6\PuyoTextEditor.exe"#.into(),
    }
});
// Everything below this is internals you shouldn't need to touch unless making wider changes
// ----------------------------------------

/// Tries to fill in missing parts of the CFG using current information
pub fn update_config_all(){
    if(CFG.proj_root.as_os_str().is_empty()){
        let proj_root = find_root().expect("Could not find the project's root folder (containing mod.ini)");
        unsafe{ CFG.elevate_mut().proj_root = proj_root; }
    }
}

pub fn print_config_valid(){
    let get_validity = |x:bool|{
        if x {"Seems Valid"} else {"Cannot Confirm Validity. Continue at your own risk!"}
    };

    println!("Config: {{
    proj_root: {}
        {}
    frontiers_data: {}
        {}
    tool_unpacker: {}
        {}
    tool_text: {}
        {}
}}",
        CFG.proj_root.display(), "(Derived automatically)",
        CFG.frontiers_data.display(), get_validity(is_frontiers_raw(&CFG.frontiers_data)),
        CFG.tool_unpacker, get_validity(is_tool_unpacker(&CFG.tool_unpacker)),
        CFG.tool_text, get_validity(is_tool_text(&CFG.tool_text)),
    )
}

#[derive(Debug)]
pub struct Config{
    /// Absolute - searched for
    pub proj_root: PathBuf,
    /// Absolute - provide
    pub frontiers_data: PathBuf,
    /// Valid command - provide
    pub tool_unpacker: String,
    /// Valid command - provide
    pub tool_text: String,
}

pub struct Paths{
    // Relative to `proj_root`
    pub assets: PathBuf,
    pub unpack: PathBuf,
    pub repack: PathBuf,
    pub build: PathBuf,
    pub raw: PathBuf,
        pub text: PathBuf,
}

pub static PAT: Lazy<Paths> = Lazy::new(||{Paths{
    assets: PathBuf::from("assets"),
    unpack: PathBuf::from("build").join("unpac"),
    repack: PathBuf::from("build").join("repac"),
    build: PathBuf::from("build"),
    raw:    PathBuf::from("raw"),
        text: PathBuf::from("text"),
}});

/// Root directory of the project (contains mod.ini)
fn find_root() -> Result<PathBuf, ()>{
    let mut current = std::env::current_dir().expect("Don't know current directory. Cannot find project root.");
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

/// Confirms we are in the raw directory for sonic frontiers
fn is_frontiers_raw(check: &Path) -> bool {
    let mut found_text = false;
    let mut found_hedgehog = false;

    let Ok(entries) = std::fs::read_dir(check) else{ return false; };
    for entry in entries{
        let Ok(entry) = entry else {return false;};
        let Ok(filename) = entry.file_name().into_string() else {return false;};
        match filename.as_str(){
            "text" => found_text = true,
            "hedgehog" => found_hedgehog = true,
            _ => {}
        }
    }
    return found_text && found_hedgehog;
}

fn is_tool_unpacker(check: &str) -> bool{
    let check = PathBuf::from(check);
    std::fs::metadata(check).is_ok_and(|x|x.is_file())
}
fn is_tool_text(check: &str) -> bool{
    let check = PathBuf::from(check);
    std::fs::metadata(check).is_ok_and(|x|x.is_file())
}

