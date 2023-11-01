## Sonic Frontiers Text Editing Workflow
I know nothing about modding Sonic games.

This is a project I used to modify in-game text.  
It generates a working HedgeModManager mod for use in the game.

Since the required files for a mod are in the root of the project (`./raw/`, `mod.ini`)
this repository can be cloned into your mods directory for convenient development.

### Using the project:
Some assembly required:
- Sonic Frontiers must be installed.
- Some utilities are written in rust. So you'll want it installed (along with an internet connection to download some packages)
- Requires `HedgeArcPack` as part of [HedgeLib](https://github.com/Radfordhound/HedgeLib)
- Requires [`PuyoTextEditor`](https://github.com/nickworonekin/puyo-text-editor) (for now)

Setting up for yourself:
1. `tools/main/src/config.rs`, change `frontiers_data`, `tool_unpacker`, and `tool_text` paths to ones appropriate to your installation.
2. Run the main tool (`cd tools/main` && `cargo run`) and if all goes well you should get a help message.
3. Confirm your paths were set up correctly using `cargo run -- --check-config`
4. Extract text from the game directory using `cargo run -- extract-text`
5. When you have finished your edits, build the mod with `cargo run -- compile`

#### Additional Resources
I've found learning resources for Sonic modding to be incredibly hidden across the internet, these were of interest to me and they might be of interest to you.
- [CamoRF/Sonic-Frontiers-Mod-Hub](https://github.com/CamoRF/Sonic-Frontiers-Mod-Hub)
- [blueskythlikesclouds/RflTemplates](https://github.com/blueskythlikesclouds/RflTemplates)
- The source code to [HedgeModManager](https://github.com/thesupersonic16/HedgeModManager)
  and [its Community Codes](https://github.com/hedge-dev/HMMCodes)
  and [its Mod Loader](https://github.com/hedge-dev/HE2ModLoader)
- [HedgeDocs](https://hedgedocs.com/)

##### Some basic documentation about RFL templates
- They are auto-generated from game data. No idea how. HedgeLib's `frontiers.json` has different data. Seems to be courtesy of Mr Skyth
- These `.rfl` files are designed for `010 editor`, and I don't think other software exists that will open them (though they are C-styled so if you wanted to manually you could read them)
- They do not correspond in file name to the ones in the game
- Anyway, open 010, 
  open `.rfl` in it, 
  then open the `.bt` file you think might work in it,
  Then go `templates`>`run template`>`*.bt`
  In the "template results" tab (if you are lucky) you'll see conveniently editable fields and comments (which are mostly Japaneese so use a translation software).
  Known working combination `character/playercommon.pac/?/player_common.rfl` and `SonicParameters.bt` (this is part of how most people make physics-y mods I think.)

> If you want to start looking for the origins of these templates, I took a string `戦闘開始座標・中心座標` (a comment from BossKnightConfig)
> converted it to shift-jis `e6 88 a6 e9 97 98 e9 96 8b e5 a7 8b e5 ba a7 e6 a8 99 e3 83 bb e4 b8 ad e5 bf 83 e5 ba a7 e6 a8 99`
> and found it in `SonicFrontiers.exe` so thats something
> Try using Ghidra (just leave it on overnight to analyse the game)

##### Diffing binary files
- No built-in method of doing this in git
- Consider using VBinDiff or https://github.com/8051Enthusiast/biodiff to help you? (Check here: https://github.com/microsoft/vscode-hexeditor/issues/47)

--------
### Samples
![Main](/mdres/Other.jpg)
![Supreme](/mdres/Supreme.jpg)