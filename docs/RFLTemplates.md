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