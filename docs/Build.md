### Using the project:
Some assembly required:
- Sonic Frontiers must be installed.
- Some utilities are written in rust. So you'll want the compiler installed (along with an internet connection to download some packages)
- Requires `HedgeArcPack` as part of [HedgeLib](https://github.com/Radfordhound/HedgeLib)
- Requires [`PuyoTextEditor`](https://github.com/nickworonekin/puyo-text-editor) (for now)

Setting up for yourself:
1. `tools/main/src/config.rs`, change `frontiers_data`, `tool_unpacker`, and `tool_text` paths to ones appropriate to your installation.
2. Run the main tool (`cd tools/main` && `cargo run`) and if all goes well you should get a help message.
3. Confirm your paths were set up correctly using `cargo run -- --check-config`
4. (if making text edits) You can extract all text from the game directory using `cargo run -- extract-text`
5. When you have finished your edits, build the mod with `cargo run -- compile`