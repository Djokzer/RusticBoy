
pub struct MemoryBus
{
	pub rom_bank_0 : [u8; 0x4000],		//16KB ROM Bank 0			(0x0000	-	0x3FFF)
	pub rom_bank_n : [u8; 0x4000],		//16KB ROM Bank N			(0x4000	-	0x7FFF)
	pub vram : [u8; 0x2000],			//8KB Video RAM 			(0x8000	-	0x9FFF)
	pub ext_ram : [u8; 0x2000],			//8KB External RAM			(0xA000	-	0xBFFF)
	pub work_ram : [u8; 0x2000],		//8KB Work RAM				(0xC000	-	0xDFFF)
	pub sprite_attrib_ram : [u8; 0xa0],	//159B Sprite Attrib RAM	(0xFE00	-	0xFE9F)
	pub io_registers : [u8; 0x80],		//127B I/O Registers		(0xFF00	-	0xFF7F)
	pub high_ram : [u8; 0x80],			//126B High RAM				(0xFF80	-	0xFFFE)
	pub interrupt_enable : u8,			//1B Interrupt Enable		(0xFFFF)
}


impl MemoryBus
{
	pub fn init_bus() -> MemoryBus
	{
		MemoryBus
		{
			rom_bank_0 : [0; 0x4000],
			rom_bank_n : [0; 0x4000],
			vram : [0; 0x2000],
			ext_ram : [0; 0x2000],
			work_ram : [0; 0x2000],
			sprite_attrib_ram : [0; 0xa0],
			io_registers : [0; 0x80],
			high_ram : [0; 0x80],
			interrupt_enable : 0,
		}
	}
	
	pub fn read_byte(&self, address : u16) -> u8
	{
		match address
		{
			0x0000..=0x3FFF => self.rom_bank_0[address as usize],
			0x4000..=0x7FFF => self.rom_bank_n[address as usize - 0x4000],
			0x8000..=0x9FFF => self.vram[address as usize - 0x8000],
			0xA000..=0xBFFF => self.ext_ram[address as usize - 0xA000],
			0xC000..=0xDFFF => self.work_ram[address as usize - 0xC000],
			0xFE00..=0xFE9F => self.sprite_attrib_ram[address as usize - 0xFE00],
			0xFF00..=0xFF7F => self.io_registers[address as usize - 0xFF00],
			0xFF80..=0xFFFE => self.high_ram[address as usize - 0xFF80],
			0xFFFF => self.interrupt_enable,
			_ => 0,
		}
	}

	pub fn read_short(&self, address : u16) -> u16
	{
		let lo: u16 = self.read_byte(address) as u16;
		let hl: u16 = self.read_byte(address + 1) as u16 ;
		return (hl << 8) | (lo & 0x00FF);
	}

	pub fn write_byte(&mut self, address : u16, value : u8)
	{
		/*
		if address < 0x100
		{
			println!("ADDRESS {:0x} : VAL {:02x}", address, value);
		}
		*/

		match address
		{
			0x0000..=0x3FFF => self.rom_bank_0[address as usize] = value,
			0x4000..=0x7FFF => self.rom_bank_n[address as usize - 0x4000] = value,
			0x8000..=0x9FFF => self.vram[address as usize - 0x8000] = value,
			0xA000..=0xBFFF => self.ext_ram[address as usize - 0xA000] = value,
			0xC000..=0xDFFF => self.work_ram[address as usize - 0xC000] = value,
			0xFE00..=0xFE9F => self.sprite_attrib_ram[address as usize - 0xFE00] = value,
			0xFF00..=0xFF7F => self.io_registers[address as usize - 0xFF00] = value,
			0xFF80..=0xFFFE => self.high_ram[address as usize - 0xFF80] = value,
			0xFFFF => self.interrupt_enable = value,
			_ => (),
		}
	}

	pub fn write_short(&mut self, address : u16, value : u16)
	{
		let hl: u8 = ((value >> 8) & 0x00FF) as u8;
		let lo: u8 = (value & 0x00FF) as u8;
		self.write_byte(address, lo);
		self.write_byte(address + 1, hl);
	}
}