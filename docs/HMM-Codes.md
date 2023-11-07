## HedgeModManager Codes Documentation
Codes provide a convenient way of quickly hooking into the game's functionality using additional Libraries provided by 'community codes'.
When a user selects a mod or a code in HedgeModManager, it *compiles* all active codes into a `.dll` which the ModLoader (`HE2ModLoader`) runs as part of its functions.

Codes don't seem to be documented anywhere, so here is what I learnt from a cursory read of `HedgeModManager`.

### Header
```
code-type "NAME" by "AUTHORS" in "FOLDER_FOR_COMMUNITY_CODES" does "CODE_DESCRIPTION_FOR_COMMUNITY_CODES

code-type "NAME" by "AUTHORS" in "FOLDER_FOR_COMMUNITY_CODES" does
/*
This is a
long description
*/

```
`code-type`: one of `Code`, `Patch`, `Library`
- by, in, does are all optional
- `Code` runs every frame
- `Patch` runs during game initialisation
- `Library` is intended to be imported by other HMM codes

### Static space
Static declarations for your code.
```c#
Code "Does something"
//
#lib "Library"
#include "XYZ" noemit
#load "System.Numerics.dll"
using System.Collections.Generic;
static SomeStaticVariable = false;
//
```
- the `//` enclosing is mandatory
- `#lib "Library"` - Imports (via token pasting?) all code in the `Library` code of the same name under its own 
    namespace (e.g.: `Library.function()`)
    Major utility libraries exist in the community codes document, you ought to have a look.
- `#include "name" noemit` Use this if you need preprocessor `#defines` from the imported library.
- Regular C# global code ensues
- `#load "dll"` Loads C# code from .NET dlls

### Code space
```c#
Code "Does Something 2"
{
    // This is the scope of a function `public static void OnFrame()` or OnInit()
    // Anything goes here. Check the official Codes.hmm for example uses
    // C# allows nested functions, so go nuts
}
```

TODO: What happens when global vars conflict?


### Full sample code
Taken from `https://github.com/hedge-dev/HMMCodes`
```c#
Code "Allow Cyclone Kick for Traversal" in "Gameplay" by "WasifBoomz" does "Allows Sonic to use the Cyclone Kick for traversal by pressing the attack button while holding the jump button."
//
    #lib "Player"

    static bool allowChargeAttack = false;
//
{
    if (Player.GetPlayerType() != Player.PlayerType.Sonic)
        return;

    if (Player.Status.IsGroundedSpecial())
        allowChargeAttack = true;

    if (Player.State.GetCurrentStateID<Sonic.StateID>() == Sonic.StateID.StateAcceleCombo1 && Player.Input.IsDown(Player.InputActionType.PlayerJump) && !Player.Status.IsGroundedSpecial() && allowChargeAttack)
    {
        Player.State.SetState<Sonic.StateID>(Sonic.StateID.StateChargeAttack);
        allowChargeAttack = false;
    }
}
```

## Notes
Due to the way codes are written, VSCode intellisense is going to give you a hard time.
I don't particularly have a good solution for this, sorry.


There are a couple of globally imported functions as defined by the ModLoader.
I'm not sure of the exact specifics of these, but they seem to be related to
- `\HedgeModManager\Source\Libraries\HedgeModManager.CodeCompiler\Resources\MemoryService.cs`
- `\HE2ModLoader\CommonLoader\CommonLoader\HookService.h` & `CommonLoaderAPI.h`

C++ is written with .NET CLI extensions, make sure your intellisense supports / acknowledges this (use MSVC `cl.exe`)

These functions are incredibly important:
- `GetAsyncKeyState` for getting computer key presses - a Windows OS function.
- `WriteAsmHook` for hijacking the game's code at specific points in x64-NASM style assembly
- `ScanSignature` for finding memory locations to replace (especially important if the game loads code / data dynamically you wish to override)

Sometimes in the official codes you'll see comments saying `/* 0x140DA61A5 */`.
This refers to an address in the game's binary which has the interested code to be modified
- The game's code is obviously not public, so if you want to see what this is doing you ought reverse engineer the game's binary using Ghidra (and its auto-analysis tool). There may already be group projects - but I'm not privy to them.

As an aside, the HMM mod loader calls `Init` `OnFrame` and `OnExit` functions from mod dlls (and `PostInit`) which you can hook into from any language.
- Source: https://github.com/Sora-yx/SF-Super-Sonic/blob/master/FT-Super-Sonic/mod.cpp#L52