# Yeeboy

Yet another Game Boy emulator.
Heavily WIP. Nothing work. Literally nothing is useable. Go away. SHOO

Essentially a rewrite in Rust of my other [Game Boy emulator](https://github.com/DuoSRX/yobml) written in ReasonML.

## TODO:

* [x] Basic architecture and organization
* [ ] CPU Instructions
  * [ ] Still missing some (check opcodes.rs)
  * [x] ~~CB Instructions~~
* [ ] Timers
* [ ] Interrupts
* [ ] GPU
  * [x] ~~Background~~
  * [ ] Sprites
  * [ ] Windows
* [ ] Cartridges
  * [x] ROM
  * [ ] Headers
  * [ ] MBC
* [ ] Graphics
  * [x] ~~SDL Frontend~~
  * [ ] WASM in browser
* [ ] Controllers
* [ ] CLI flags
* [ ] Audio
* [ ] Save
* [ ] Logger
* [ ] Debugger
* [ ] Config file

## Blaargs tests:

* [x] ~~01-special~~
* [ ] 02-interrupts
* [x] ~~03-op_sp_hl~~
* [x] ~~04-op_r_imm~~
* [x] ~~05-op_rp~~
* [x] ~~06-ld_r_r~~
* [x] ~~07-jr_jp_call_ret_rst~~
* [ ] 08-misc_instrs
* [x] ~~09-op_r_r~~
* [x] ~~10-bit_ops~~
* [ ] 11-op_a_hl

## Games that boot

* Dr Mario

## Games that work

NONE :(
