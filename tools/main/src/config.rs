use std::path::PathBuf;

use once_cell::sync::Lazy;

use crate::util::SuperLazy;

pub static CFG: SuperLazy<Config> = SuperLazy::new(||{
    Config {
        proj_root: PathBuf::new(),
        // Change these
        frontiers_data: PathBuf::from(r#"A:\Games\Not Emu\Steam\steamapps\common\SonicFrontiers\image\x64\raw\"#),
        tool_unpacker: r#"a:\Games\Not Emu\Sonic\Frontiers\Modding Tools\HedgeLib-dev-21_10_2023\bin\HedgeArcPack.exe"#.into(),
        tool_text: r#"a:\Games\Not Emu\Sonic\Frontiers\Modding Tools\PuyoTextEditor-2.0.6\PuyoTextEditor.exe"#.into(),
    }
});

/// Tries to fill in missing parts of the CFG using current information
pub fn update_config_all(){
    if(CFG.proj_root.as_os_str().is_empty()){
        let proj_root = find_root().expect("Could not find the project's root folder (containing mod.ini)");
        unsafe{ CFG.elevate_mut().proj_root = proj_root; }
    }
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
    pub raw: PathBuf,
        pub text: PathBuf,
}

pub static PAT: Lazy<Paths> = Lazy::new(||{Paths{
    assets: PathBuf::from("assets"),
    unpack: PathBuf::from("build").join("unpac"),
    repack: PathBuf::from("build").join("repac"),
    raw:    PathBuf::from("raw"),
        text: PathBuf::from("text"),
}});

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