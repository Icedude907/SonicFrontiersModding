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
Some resources I've used on my modding escapades (not all used here):
- [CamoRF/Sonic-Frontiers-Mod-Hub](https://github.com/CamoRF/Sonic-Frontiers-Mod-Hub)
- [blueskythlikesclouds/RflTemplates](https://github.com/blueskythlikesclouds/RflTemplates)
- The source code to [HedgeModManager](https://github.com/thesupersonic16/HedgeModManager)
  and [its Community Codes](https://github.com/hedge-dev/HMMCodes)
- [HedgeDocs](https://hedgedocs.com/)

--------
### Samples
![Main](/mdres/Other.jpg)
![Supreme](/mdres/Supreme.jpg)