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
2. `git clone https://github.com/beholder-d/td3-pattern.git`
3. `cargo build --release`
4. `./target/release/td3-pattern`

## File Format

```
TD-3 Pattern
Active Steps: 16, Triplet mode: Off

// Step:    01, 02, 03, 04, 05, 06, 07, 08, 09, 10, 11, 12, 13, 14, 15, 16
Note:       D#, D#, C#, C#, C#, G#, D#, D#, G#, D#, E , D#, D#, C#, D#, G#  // C -C# .. B -C^
Transpose:    ,   , DN,   , DN, UP, UP, UP, UP, UP,   , UP,   , UP, DN, UP  // DN-  -UP
Accent:       , AC,   ,   ,   ,   ,   ,   ,   ,   ,   ,   ,   ,   ,   ,     //   -AC
Slide:        , SL,   ,   , SL,   ,   , SL,   ,   ,   ,   ,   , SL,   , SL  //   -SL
Tie/Rest:     , TI,   ,   ,   , TI,   ,   , TI,   ,   , TI, TI, RE,   , TI  //   -TI-RE
```

*I wanted to use to YAML, but alas it doesn't like empty entries in arrays like `Accent: [ , AC]`*

## Usage

```
Usage:
    td3-pattern [-in=\"name\"] [-out=\"name\"] <group> <pattern><a|b> [-file=filename]
    td3-pattern [-in=\"name\"] [-out=\"name\"] upload <group> <pattern><a|b> -file=filename
Where:
    -in=\"name\" -- name of TD-3's midi in
    -out=\"name\" -- name of TD-3's midi out
    -file=filename -- file for saving or loading pattern, in case of saving if not specified stdin is used
    <group> -- Group 1-4
    <pattern><a|b> - Pattern 1-8 AB

Example -- view group 1 pattern 1A:
    td3-pattern 1 1A
Example -- using loopback drivers save group 4 pattern 2B to file
    td3-pattern -in=\"Loopback in 1\" -out=\"Loopback out 1\" 1 2B -file=pattern1-2B.txt
Example -- load file and upload it to group 3 pattern 8A
    td3-pattern upload 1 1A -file=confusion-pattern.txt
```

## Sysex

### Format

Midi sysex format and communications are described in https://303patterns.com/td3-midi.html by people from https://audiopump.co/ kudos to them.

### Pattern sysex payload

```
78 <-- message ID
03, 0f, <-- group, pattern
00, 01, <-- unknown 1
01, 0b, 01, 0b, 00, 0d, 01, 09, 00, 0d, 02, 0c, 02, 07, 02, 07, <-- note
02, 0c, 02, 07, 01, 0c, 02, 07, 01, 0b, 02, 05, 00, 0f, 02, 0c,
00, 00, 00, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, <-- accent
00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00,
00, 00, 00, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 01, <-- slide
00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00,
00, 00, <-- triplet
01, 00, <-- steps
00, 00, <-- unknown 2
09, 0d, 09, 09, <-- ties
00, 00, 02, 00  <-- rests
```

### Questions

1. I'm seeing 01 in second byte of unknown 1, which is not like on 303patterns.com or some other places

## Sequencer Quirks

Original 303 sequencer is quirky, TD-3 is mimicking that quirks. Let's start with pattern below:

```
// Step:    01, 02, 03, 04
Note:        C,  D,  E,  F
Transpose:    ,   , UP, DN
Accent:     AC,   ,   ,
Slide:        ,   ,   ,
Tie/Rest:     , TI,   ,
```

You think it would be pattern playing C - D - E - F or if we interpret Tie note as two Ds C - D...D - F pattern? You're wrong! Actual pattern would be:

```
// Step:    01, 02, 03, 04
Note:        C,  D... ,  E
Transpose:    ,   ,   , UP
Accent:     AC,   ,   ,
Slide:        ,   ,   ,
Tie/Rest:     , TI... ,
```

And this pattern:

```
// Step:    01, 02, 03, 04, 05
Note:        C,  D,  E,  F,  G
Transpose:    ,   , UP, DN,
Accent:       ,   , AC,   ,
Slide:        ,   ,   ,   ,
Tie/Rest:     , TI,   , RE,
```

Would actually turn to:

```
// Step:    01, 02, 03, 04, 05
Note:        C,  D... ,   ,  E
Transpose:    ,   ,   ,   , UP
Accent:       ,   ,   ,   , AC
Slide:        ,   ,   ,   ,
Tie/Rest:     , TI... , RE,
```

So as you see Tie/Rest is constantly advancing but Note/Transpose/Accent/Slide are advancing differently when RS/TI are hit. If TI hit it's waiting for when string of TI is over and then stepping forward, if RS hit it's waiting for string of RS is over and then playing current step.

``¯\_(ツ)_/¯``

## Additional notes

Thanks to https://github.com/alebastr for answering noob questions

