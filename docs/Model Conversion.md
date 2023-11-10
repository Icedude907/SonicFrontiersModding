
Extracted data `tool extract <path>`
Then ran HedgeNeedle (HedgeLib) on the `.model` file to unpack it 
- Had an `ERROR: bad pipe` coming from the application, I moved the `.model` file into the same folder as the HedgeNeedle tool and it worked fine (im presuming because the parent directory has an extension)

Get a list of subfiles and a `.lodinfo` document. The game switches between 0 (highest quality model) and other numbers with distance and other settings.

Now these `.model` files (`chr_sonic_crown.0.model`) are the ones that can be edited.
Find the shared data document `.skl.pxd` and rename it to the same as the `chr_sonic_crown.0.skl.pxd`
- This contains armature data (and uv mapping?)

Import them into blender using `modelfbx` (libgens-sonicglvl) to convert them to fbx
- the output of this is better than the blender plugin

Learn how to blender (select vertex groups, UV Mapping, selecting material uses, memes)

Export FBX - Untick Armature > Export Leaf Bones and tick Transform -> Apply Scalings: FBX Units Scale
- Update blender if its old and bugs out
- I had a problem with UV Maps having NaN values limiting fbx imports, I deleted the dodgy maps and everything was fine.

Use Skyth's ModelConverter `ModelConverter.exe chr_sonic.fbx` or use the batch file.
The tool may fail silently so have fun

Store the model and the material files modelconverter outputs in the pac.
(Note, if you haven't properly configured your materials in blender - uv maps and such, just use the game's material files.)

Apply needle to the model if doing lods, else just replace the model folder without applying needle and it will work.
Apply pac to folder
Profit

DDS Format: first gimp option DXT1
NOTE: In an attempt to make my texture load correctly in game, I had to resize it to be a power of 2 (512x512) is what I did (`burger_crown.dds`)

Aid: https://www.youtube.com/watch?v=B_-YJ2I1_M4

TODO: Automate the needling