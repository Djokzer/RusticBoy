//IMPORT

//STRUCT
pub struct Flag
{
	value : u8,				//Flag 8-bit value
	zero_flag : bool,		//Bit 7
	sub_flag : bool,		//Bit 6
	half_carry_flag : bool,	//Bit 5
	carry_flag : bool,		//Bit 4
}

pub struct Register
{
	pub a : u8,
	pub f : Flag,
	pub b : u8,
	pub c : u8,
	pub d : u8,
	pub e : u8,
	pub h : u8,
	pub l : u8,
	pub stack_pointer : u16,
	pub program_counter : u16,
}

//INIT STRUCTS
pub fn init_flag() -> Flag
{
	Flag
	{
		value : 0,
		zero_flag : false,
		sub_flag : false,
		half_carry_flag : false,
		carry_flag : false,
	}
}

pub fn init_register() -> Register
{
	Register
	{
		a : 0,
		f : init_flag(),
		b : 0,
		c : 0,
		d : 0,
		e : 0,
		h : 0,
		l : 0,
		stack_pointer : 0,
		program_counter : 0x100, //After the cartridge header
	}
}

//STRUCT FUNCTIONS
impl Flag
{
	pub fn set_zero_flag(&mut self, val : bool)
	{
		self.zero_flag = val;
		if val {self.value |= 0x80;}
		else {self.value &= !0x80;}
	}
	pub fn set_sub_flag(&mut self, val : bool)
	{
		self.sub_flag = val;
		if val {self.value |= 0x40;}
		else {self.value &= !0x40;}
	}
	pub fn set_half_carry_flag(&mut self, val : bool)
	{
		self.half_carry_flag = val;
		if val {self.value |= 0x20;}
		else {self.value &= !0x20;}
	}
	pub fn set_caryy_flag(&mut self, val : bool)
	{
		self.carry_flag = val;
		if val {self.value |= 0x10;}
		else {self.value &= !0x10;}
	}

	pub fn set_value(&mut self, val : u8)
	{
		self.value = val;
		self.set_zero_flag((self.value & 0x80) == 0x80);
		self.set_sub_flag((self.value & 0x40) == 0x40);
		self.set_half_carry_flag((self.value & 0x20) == 0x20);
		self.set_caryy_flag((self.value & 0x10) == 0x10);
	}
}

impl Register
{
	pub fn get_af(&mut self) -> u16
	{
		return (self.a << 8) as u16 | self.f.value as u16;
	}
	pub fn get_bc(&mut self) -> u16
	{
		return (self.b << 8) as u16 | self.c as u16;
	}
	pub fn get_de(&mut self) -> u16
	{
		return (self.d << 8) as u16 | self.e as u16;
	}
	pub fn get_hl(&mut self) -> u16
	{
		return (self.h << 8) as u16 | self.l as u16;
	}

	pub fn set_af(&mut self, value : u16)
	{
		self.a = (value >> 8) as u8;
		self.f.set_value((value & 0x00f0) as u8);
	}
	pub fn set_bc(&mut self, value : u16)
	{
		self.b = (value >> 8) as u8;
		self.c = (value & 0x00ff) as u8;
	}
	pub fn set_de(&mut self, value : u16)
	{
		self.d = (value >> 8) as u8;
		self.e = (value & 0x00ff) as u8;
	}
	pub fn set_hl(&mut self, value : u16)
	{
		self.h = (value >> 8) as u8;
		self.l = (value & 0x00ff) as u8;
	}
}