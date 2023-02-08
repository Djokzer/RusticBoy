use crate::cartridge::*;
use crate::bus::*;

use std::{fs::{metadata, File}, io::Read};

pub struct Emulator
{
	pub cart : Cartridge,
	pub mem_bus : MemoryBus,
	//TO DO
}


impl Emulator
{
	pub fn init_emulator() -> Emulator
	{
		Emulator
		{
			cart : Cartridge::init_cartridge(),
			mem_bus : MemoryBus::init_bus(),
		}
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
		for i in 0x0000..0x7FFF
		{
			self.mem_bus.write_byte(i, self.cart.data[i as usize]); 
		}
	}

	pub fn emulation_cycle(&mut self) -> bool
	{
		//TO DO
		return false;
	}
}


