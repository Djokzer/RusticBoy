
pub struct Flag
{
	pub value : u8,				//Flag 8-bit value
	pub zero_flag : bool,		//Bit 7
	pub sub_flag : bool,		//Bit 6
	pub half_carry_flag : bool,	//Bit 5
	pub carry_flag : bool,		//Bit 4
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
	pub program_counter : u16,
	pub stack_pointer : u16,
}

impl Flag
{
	//INIT FLAG
	pub fn init_flag() -> Flag
	{
		Flag
		{
			value : 0xb0,
			zero_flag : true,
			sub_flag : false,
			half_carry_flag : true,
			carry_flag : true,
		}
	}
	
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
	//INIT REGISTER
	pub fn init_register() -> Register
	{
		Register
		{
			a : 0x01,
			f : Flag::init_flag(),
			b : 0x00,
			c : 0x13,
			d : 0x00,
			e : 0xd8,
			h : 0x01,
			l : 0x4d,
			program_counter : 0x00, //0x100 if skip boot rom
			stack_pointer : 0xFFFE,
		}
	}

	pub fn get_af(&self) -> u16
	{
		return ((self.h as u16) << 8) | self.f.value as u16;
	}
	pub fn get_bc(&self) -> u16
	{
		return ((self.b as u16) << 8) | self.c as u16;
	}
	pub fn get_de(&self) -> u16
	{
		return ((self.d as u16) << 8) | self.e as u16;
	}
	pub fn get_hl(&self) -> u16
	{
		return ((self.h as u16) << 8) | self.l as u16;
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