# TD3 Pattern Utility

TD3 Pattern Utility is utility for import/export patterns from Behringer TD-3 via midi sysex.

## Description

I hit the wall trying to modify pattern in TD3 to fit chord change because editor is complicated and buggy. As much as like 303/TD sequencer as creative tool with happy accidents modifying patterns is nightmare on TD3.

* I don't like the way Synthribe (Behringer Utility) is displaying or editing TD3 patterns. And it still hase bugs which is making it quite unusable.
* Paper patch sheets for 303 are way more suitable for actual editor.
* I like readable formats.
* Conosole utilities are automatable.
* I'm learning Rust :)

## How to build it

I'm planning to provide binaries for Windows/Mac/Linux later, for now you should build it form sources

1. Install Rust https://www.rust-lang.org/tools/install or https://forge.rust-lang.org/infra/other-installation-methods.html
2. git clone https://github.com/beholder-d/td3-pattern.git
3. cargo build --release


## File Format

```
TD-3 Pattern
Active Steps: 16, Triplet mode: Off

// Step:    00, 01, 02, 03, 04, 05, 06, 07, 08, 09, 0A, 0B, 0C, 0D, 0E, 0F
Note:       D#, D#, C#, C#, C#, G#, D#, D#, G#, D#, E , D#, D#, C#, D#, G#  // C -C# .. B -C^
Transpose:    ,   , DN,   , DN, UP, UP, UP, UP, UP,   , UP,   , UP, DN, UP  // DN-  -UP
Accent:       , AC,   ,   ,   ,   ,   ,   ,   ,   ,   ,   ,   ,   ,   ,     //   -AC
Slide:        , SL,   ,   , SL,   ,   , SL,   ,   ,   ,   ,   , SL,   , SL  //   -SL
Tied note:  TI,   , TI, TI, TI,   ,   , TI, TI,   ,   , TI, TI,   ,   , TI  //   -TI
Rest:         ,   ,   ,   ,   ,   ,   ,   ,   ,   ,   ,   ,   , RS,   ,     //   -RS
```

## Usage

```
Usage:
    td3pattern [-in_port=\"name\"] [-out_port=\"name\"] <group> <pattern><a|b> [-file=filename]
    td3pattern [-in_port=\"name\"] [-out_port=\"name\"] upload <group> <pattern><a|b> -file=filename
Where:
    -in_port=\"name\" -- name of TD-3 midi in
    -out_port=\"name\" -- name of TD-3 midi out
    -file=filename -- file for saving or loading pattern, in case of saving if not specified stdin is used
    <group> -- Group 1-4
    <pattern><a|b> - Pattern 1-8 AB

Example -- view group 1 pattern 1A:
    td3pattern 1 1A
Example -- using loopback drivers save group 4 pattern 2B to file
    td3pattern -in_port=\"Loopback in 1\" -out_port=\"Loopback out 1\" 1 2B -file=pattern1-2B.txt
Example -- load file and upload it to group 3 pattern 8A
    td3pattern upload 1 1A -file=confusion-pattern.txt
```

## TD3 Sysex format

Midi sysex format and communications are described in https://303patterns.com/td3-midi.html by people from https://audiopump.co/ kudos to them.

