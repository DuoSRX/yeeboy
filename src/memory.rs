use crate::gpu::Gpu;
use crate::cartridge::Cartridge;

pub struct Memory  {
    cartridge: Cartridge,
    gpu: Gpu,
    wram: Vec<u8>,
    hram: Vec<u8>,
    io: Vec<u8>,
}
