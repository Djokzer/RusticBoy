use crate::cartridge::*;
use crate::bus::*;
use crate::cpu::*;

use std::{fs::{metadata, File}, io::Read};

pub struct Emulator
{
	pub cart : Cartridge,
	pub mem_bus : MemoryBus,
	pub cpu: Cpu,
	// ! TO DO
}


impl Emulator
{
	pub fn init_emulator() -> Emulator
	{
		Emulator
		{
			cart : Cartridge::init_cartridge(),
			mem_bus : MemoryBus::init_bus(),
			cpu : Cpu::init_cpu(),
		}
	}

	pub fn load_boot_rom(&mut self, filename : &str) -> bool
	{
		// READ BOOT ROM
		let mut f = match File::open(filename) {
			Ok(file) => file,
			Err(err) => {
				println!("Error: {}", err);
				return false;
			}
		};
		let metadata = metadata(filename).expect("unable to read metadata");
		let mut buffer = vec![0; metadata.len() as usize];
		f.read(&mut buffer).expect("buffer overflow");

		// LOAD BOOT ROM
		self.mem_bus.load_boot_rom(buffer);

		return true;
	}
	
	pub fn load_rom(&mut self, filename : &str)
	{
		//READ ROM FILE
		let mut f = File::open(filename).expect("no file found");
		let metadata = metadata(filename).expect("unable to read metadata");
		let mut buffer = vec![0; metadata.len() as usize];
		f.read(&mut buffer).expect("buffer overflow");

		//LOAD THE ROM DATA
		self.cart.load_cartridge(filename, buffer);

		//FOR NOW NO MBC, COPY THE 32KB ROM INTO THE MEMORY BUS
		for i in 0x0100..0x7FFF
		{
			self.mem_bus.write_byte(i, self.cart.data[(i - 0x100) as usize]); 
		}
	}

	pub fn emulation_cycle(&mut self) -> u32
	{
		// ! TO DO
		// ! CPU STEP
		let cycles = self.cpu.step(&mut self.mem_bus);

		// ! PPU STEP
		// ! APU STEP

		return cycles;
	}
}


