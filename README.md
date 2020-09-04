# Yeeboy

Yet another Game Boy emulator.

Essentially a rewrite in Rust of my other [Game Boy emulator](https://github.com/DuoSRX/yobml) written in ReasonML.

## TODO:

* [x] Basic architecture and organization
* [ ] CPU Instructions
  * [ ] Still missing a few (check opcodes.rs)
  * [x] ~~CB Instructions~~
* [x] Timers
* [x] Interrupts
* [ ] GPU
  * [x] ~~Background~~
  * [x] Sprites
    * Y Flip doesn't work properly
    * ~~Sprite a flipped horizontally on themselves~~
  * [x] Windows
    * Needs more testing
* [ ] Cartridges
  * [x] ROM
  * [x] Headers
  * [ ] MBC
    [ ] MBC1 (WIP)
    [ ] MBC3
* [ ] Graphics
  * [x] ~~SDL Frontend~~
  * [ ] WASM in browser
* [x] ~~Controllers~~
* [x] CLI flags
* [ ] Audio
* [ ] Save
* [ ] Logger
* [ ] Debugger
* [ ] Config file

## Blaargs tests:

* [x] ~~01-special~~
* [ ] 02-interrupts
  * Everything passes except Timer #4. Timing are probably off somewhere...
* [x] ~~03-op_sp_hl~~
* [x] ~~04-op_r_imm~~
* [x] ~~05-op_rp~~
* [x] ~~06-ld_r_r~~
* [x] ~~07-jr_jp_call_ret_rst~~
* [x] ~~08-misc_instrs~~
* [x] ~~09-op_r_r~~
* [x] ~~10-bit_ops~~
* [x] ~~11-op_a_hl~~

## Games that boot

* Dr Mario
* Tetris

## Games that work well

NONE :(
