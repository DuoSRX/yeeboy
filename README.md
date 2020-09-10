# Yeeboy

Yet another Game Boy emulator.

Essentially a rewrite in Rust of my other [Game Boy emulator](https://github.com/DuoSRX/yobml) written in ReasonML.

## Usage

`$ cargo run --release -- path/to/rom-file`

Optionally provide `--trace` to have the emulator spew out every instruction while it's running.
Keep in mind that it has a negative impact on performance, depending on your terminal of choice.

## Controls (not configurable yet)

* A -> X
* B -> Z
* Select -> Left Shit
* Start -> Space
* D-Pad -> Arrow keys
* Escape -> Quit

## TODO:

* [x] Basic architecture and organization
* [ ] CPU Instructions
  * [ ] Still missing a few (check opcodes.rs)
  * [x] ~~CB Instructions~~
* [x] Timers
* [x] Interrupts
* [x] GPU
  * [x] ~~Background~~
  * [x] Sprites
  * [x] Windows
    * Needs more testing
* [ ] Cartridges
  * [x] ROM
  * [x] Headers
  * [X] MBC
    [ ] MBC1 (WIP)
    [X] MBC3
* [x] Graphics
  * [x] ~~SDL Frontend~~
  * [x] WASM in browser
* [x] ~~Controllers~~
* [x] CLI flags
* [ ] Timing is off (need to add a frame limiter)
* [ ] Audio
* [ ] Save
* [ ] Logger
* [ ] Debugger
* [ ] GPU Debugging windows
  * [X] OAM Viewer
  * [ ]Tile map
  * [ ]Background map
  * [ ]Palettes
* [ ] Config file
* [ ] Automated testing

## Blaargs tests:

* [x] ~~01-special~~
* [x] ~~02-interrupts~~
* [x] ~~03-op_sp_hl~~
* [x] ~~04-op_r_imm~~
* [x] ~~05-op_rp~~
* [x] ~~06-ld_r_r~~
* [x] ~~07-jr_jp_call_ret_rst~~
* [x] ~~08-misc_instrs~~
* [x] ~~09-op_r_r~~
* [x] ~~10-bit_ops~~
* [x] ~~11-op_a_hl~~

## Playable games

* Dr Mario
* Tetris
* Super Mario Land

## Games that boot but are glitchy/unplayble

* Pokemon Red

## Games that won't even start or black screen

Any game that isn't RomOnly, MBC1 or MBC3
