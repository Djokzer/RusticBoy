use crate::register::*;
use crate::bus::*;
use std::process;

pub struct Cpu
{
	pub reg : Register,	// CPU REGISTERS
	pub ime : bool,		// Interrupt Master Enable Flag
}

impl Cpu
{
	pub fn init_cpu() -> Cpu
	{
		Cpu
		{
			reg : Register::init_register(),
			ime : false,
		}
	}

	pub fn get_opcode(&mut self, mem_bus : &MemoryBus) -> u8
	{
		let opcode = mem_bus.read_byte(self.reg.program_counter);
		println!("PC : {:0x} - Opcode : {:02x}", self.reg.program_counter, opcode);    //DEBUG
		self.reg.program_counter += 1;
		return opcode;
	}

	pub fn step(&mut self, mem_bus : &mut MemoryBus) -> u32
	{
		let opcode = self.get_opcode(mem_bus);
		let n = mem_bus.read_byte(self.reg.program_counter);
		let nn = mem_bus.read_short(self.reg.program_counter);

		match opcode
		{
			0x00 => 4, //NOP
			0x01 => // LD BC, d16
			{
				self.reg.program_counter += 2;
				self.reg.set_bc(nn);
				12   
			},
			0x03 => self.inc_rr("BC"),// INC BC
			0x04 => Cpu::inc_r(&mut self.reg.f, &mut self.reg.b),	// INC B 
			0x05 => Cpu::dec_r(&mut self.reg.f, &mut self.reg.b),	// DEC B 
			0x06 => Cpu::ld_n_to_r(&mut self.reg.b, n, &mut self.reg.program_counter),   // LD B,d8
			0x07 =>	// RLCA
			{
				self.reg.f.set_caryy_flag((self.reg.a & 0x80) == 0x80);
				self.reg.a = self.reg.a << 1;
				if self.reg.f.carry_flag
				{
					self.reg.a |= 0x01;
				}
				self.reg.f.set_zero_flag(false);
				self.reg.f.set_sub_flag(false);
				self.reg.f.set_half_carry_flag(false);
				4
			},
			0x0A => self.ld_mem_rr_to_a(mem_bus, self.reg.get_bc()),		// LD A,(BC) 
			0x0c => Cpu::inc_r(&mut self.reg.f, &mut self.reg.c),	// INC C  
			0x0d => Cpu::dec_r(&mut self.reg.f, &mut self.reg.c),	// DEC C 
			0x0e => Cpu::ld_n_to_r(&mut self.reg.c, n, &mut self.reg.program_counter),  // LD C, d8
			0x0F =>	// RRCA
			{
				self.reg.f.set_caryy_flag((self.reg.a & 0x01) == 0x01);
				self.reg.a = self.reg.a >> 1;
				if self.reg.f.carry_flag
				{
					self.reg.a |= 0x80;
				}
				self.reg.f.set_zero_flag(false);
				self.reg.f.set_sub_flag(false);
				self.reg.f.set_half_carry_flag(false);
				4
			}
			0x11 => // LD DE, d16
			{ 
				self.reg.set_de(nn);
				self.reg.program_counter += 2;
				12   
			},
			0x13 => self.inc_rr("DE"),// INC DE
			0x14 => Cpu::inc_r(&mut self.reg.f, &mut self.reg.d),   // INC D
			0x15 => Cpu::dec_r(&mut self.reg.f, &mut self.reg.d),	// DEC D
			0x16 => Cpu::ld_n_to_r(&mut self.reg.d, n, &mut self.reg.program_counter),   // LD D,d8
			0x17 =>	// RLA
			{
				let old_carry = self.reg.f.carry_flag;
				self.reg.f.set_caryy_flag((self.reg.a & 0x80) == 0x80);
				self.reg.a = self.reg.a << 1;
				if old_carry
				{
					self.reg.a |= 0x01;
				}
				self.reg.f.set_zero_flag(false);
				self.reg.f.set_sub_flag(false);
				self.reg.f.set_half_carry_flag(false);
				4
			},
			0x18 => // JR r8,
			{
				self.reg.program_counter += 1;
				self.reg.program_counter = Cpu::add_signed(self.reg.program_counter, n); //RELATIVE JUMP
				12
			},
			0x1A => self.ld_mem_rr_to_a(mem_bus, self.reg.get_de()),        // LD A,(DE)
			0x1C => Cpu::inc_r(&mut self.reg.f, &mut self.reg.e),   // INC E
			0x1D => Cpu::dec_r(&mut self.reg.f, &mut self.reg.e),	// DEC E
			0x1E => Cpu::ld_n_to_r(&mut self.reg.e, n, &mut self.reg.program_counter),  // LD E, d8
			0x1F =>	// RRA
			{
				let old_carry = self.reg.f.carry_flag;
				self.reg.f.set_caryy_flag((self.reg.a & 0x01) == 0x01);
				self.reg.a = self.reg.a >> 1;
				if old_carry
				{
					self.reg.a |= 0x80;
				}
				self.reg.f.set_zero_flag(false);
				self.reg.f.set_sub_flag(false);
				self.reg.f.set_half_carry_flag(false);
				4
			}
			0x20 => // JR NZ, r8
			{
				self.reg.program_counter += 1;
				if !self.reg.f.zero_flag
				{
					self.reg.program_counter = Cpu::add_signed(self.reg.program_counter, n); //RELATIVE JUMP
					return 12;
				}
				return  8;
			},
			0x21 => // LD HL, d16
			{
				self.reg.set_hl(nn);
				self.reg.program_counter += 2;
				12   
			},
			0x22 => // LD (HL+), A
			{   
				mem_bus.write_byte(self.reg.get_hl(), self.reg.a);
				self.reg.set_hl(self.reg.get_hl() + 1);
				8
			},
			0x23 => self.inc_rr("HL"),// INC HL
			0x24 => Cpu::inc_r(&mut self.reg.f, &mut self.reg.h), // INC H
			0x25 => Cpu::dec_r(&mut self.reg.f, &mut self.reg.h),	// DEC H 
			0x26 => Cpu::ld_n_to_r(&mut self.reg.h, n, &mut self.reg.program_counter),   // LD H,d8
			0x28 => // JR Z, r8,
			{
				self.reg.program_counter += 1;
				if self.reg.f.zero_flag
				{
					self.reg.program_counter = Cpu::add_signed(self.reg.program_counter, n); //RELATIVE JUMP
					return 12;
				}
				return  8;
			},
			0x2c => Cpu::inc_r(&mut self.reg.f, &mut self.reg.l),	// INC L
			0x2D => Cpu::dec_r(&mut self.reg.f, &mut self.reg.l),	// DEC L
			0x2e => Cpu::ld_n_to_r(&mut self.reg.l, n, &mut self.reg.program_counter), // LD L, d8
			0x30 => // JR NC, r8,
			{
				self.reg.program_counter += 1;
				if !self.reg.f.carry_flag
				{
					self.reg.program_counter = Cpu::add_signed(self.reg.program_counter, n); //RELATIVE JUMP
					return 12;
				}
				return  8;
			},
			0x31 => // LD SP, d16
			{
				self.reg.stack_pointer = nn;
				self.reg.program_counter += 2;
				12   
			},
			0x32 => // LD (HL-), A
			{
				mem_bus.write_byte(self.reg.get_hl(), self.reg.a);
				self.reg.set_hl(self.reg.get_hl() - 1);
				8
			},
			0x34 => // INC (HL)
			{
				let mut data = mem_bus.read_byte(self.reg.get_hl());
				self.reg.f.half_carry_flag = (data & 0x0F) == 0x0F;
				data += 1;
				mem_bus.write_byte(self.reg.get_hl(), data);
				self.reg.f.set_zero_flag(data == 0);
				self.reg.f.set_sub_flag(false);
				12
			},
			0x38 => // JR C, r8,
			{
				self.reg.program_counter += 1;
				if self.reg.f.carry_flag
				{
					self.reg.program_counter = Cpu::add_signed(self.reg.program_counter, n); //RELATIVE JUMP
					return 12;
				}
				return  8;
			},
			0x3C => Cpu::inc_r(&mut self.reg.f, &mut self.reg.a), // INC A
			0x3D => Cpu::dec_r(&mut self.reg.f, &mut self.reg.a), // DEC A
			0x3e => Cpu::ld_n_to_r(&mut self.reg.a, n, &mut self.reg.program_counter),	// LD A, d8
			0x40 => Cpu::ld_r_to_r(&mut self.reg.b, self.reg.a),	// LD B,A
			0x41 => Cpu::ld_r_to_r(&mut self.reg.b, self.reg.a),	// LD B,A
			0x42 => Cpu::ld_r_to_r(&mut self.reg.b, self.reg.a),	// LD B,A
			0x43 => Cpu::ld_r_to_r(&mut self.reg.b, self.reg.a),	// LD B,A
			0x44 => Cpu::ld_r_to_r(&mut self.reg.b, self.reg.a),	// LD B,A
			0x45 => Cpu::ld_r_to_r(&mut self.reg.b, self.reg.a),	// LD B,A
			0x47 => Cpu::ld_r_to_r(&mut self.reg.b, self.reg.a),	// LD B,A
			0x48 => Cpu::ld_r_to_r(&mut self.reg.c, self.reg.b),	// LD C,B 
			0x49 =>	4,														   // LD C,C (NOP)
			0x4A => Cpu::ld_r_to_r(&mut self.reg.c, self.reg.d),	// LD C,D
			0x4B => Cpu::ld_r_to_r(&mut self.reg.c, self.reg.e),	// LD C,E
			0x4C => Cpu::ld_r_to_r(&mut self.reg.c, self.reg.h),	// LD C,H
			0x4D => Cpu::ld_r_to_r(&mut self.reg.c, self.reg.l),	// LD C,L
			0x4F => Cpu::ld_r_to_r(&mut self.reg.c, self.reg.a),	// LD C,A
			0x50 => Cpu::ld_r_to_r(&mut self.reg.d, self.reg.b),	// LD D,B
			0x51 => Cpu::ld_r_to_r(&mut self.reg.d, self.reg.c),	// LD D,C	
			0x52 => 4,														   // LD D,D (NOP)
			0x53 => Cpu::ld_r_to_r(&mut self.reg.d, self.reg.e),	// LD D,E	
			0x54 => Cpu::ld_r_to_r(&mut self.reg.d, self.reg.h),	// LD D,H	
			0x55 => Cpu::ld_r_to_r(&mut self.reg.d, self.reg.l),	// LD D,L	
			0x56 => self.ld_mem_hl_to_r(mem_bus, "D"),						// LD D,(HL)		
			0x57 => Cpu::ld_r_to_r(&mut self.reg.d, self.reg.b),	// LD D,A	
			0x60 =>	Cpu::ld_r_to_r(&mut self.reg.h, self.reg.b),	// LD H,B
			0x61 =>	Cpu::ld_r_to_r(&mut self.reg.h, self.reg.c),	// LD H,C
			0x62 =>	Cpu::ld_r_to_r(&mut self.reg.h, self.reg.d),	// LD H,D
			0x63 =>	Cpu::ld_r_to_r(&mut self.reg.h, self.reg.e),	// LD H,E
			0x64 =>	4,														   // LD H,H (NOP)
			0x65 =>	Cpu::ld_r_to_r(&mut self.reg.h, self.reg.l),	// LD H,L
			0x66 => self.ld_mem_hl_to_r(mem_bus, "H"),						// LD H,(HL)
			0x67 =>	Cpu::ld_r_to_r(&mut self.reg.h, self.reg.a),	// LD H,A
			0x70 => self.ld_r_to_mem_hl(mem_bus, self.reg.b),  			// LD (HL), B
			0x71 => self.ld_r_to_mem_hl(mem_bus, self.reg.c),  			// LD (HL), C
			0x72 => self.ld_r_to_mem_hl(mem_bus, self.reg.d),  			// LD (HL), D
			0x73 => self.ld_r_to_mem_hl(mem_bus, self.reg.e),  			// LD (HL), E
			0x74 => self.ld_r_to_mem_hl(mem_bus, self.reg.h),  			// LD (HL), H
			0x75 => self.ld_r_to_mem_hl(mem_bus, self.reg.l),  			// LD (HL), L
			0x77 => self.ld_r_to_mem_hl(mem_bus, self.reg.a),  			// LD (HL), A
			0x78 => Cpu::ld_r_to_r(&mut self.reg.a, self.reg.b),	// LD A,B
			0x79 => Cpu::ld_r_to_r(&mut self.reg.a, self.reg.c),	// LD A,C
			0x7A => Cpu::ld_r_to_r(&mut self.reg.a, self.reg.d),	// LD A,D
			0x7B => Cpu::ld_r_to_r(&mut self.reg.a, self.reg.e),	// LD A,E
			0x7C => Cpu::ld_r_to_r(&mut self.reg.a, self.reg.h),	// LD A,H
			0x7D => Cpu::ld_r_to_r(&mut self.reg.a, self.reg.l),	// LD A,L
			0x7E => self.ld_mem_rr_to_a(mem_bus, self.reg.get_hl()),	    // LD A, (HL)
			0x7F => 4,														   // LD A,A (NOP)
			0xA8 => self.xor_a_r(self	.reg.b), // XOR A, B
			0xA9 => self.xor_a_r(self.reg.c),  // XOR A, C
			0xAA => self.xor_a_r(self.reg.d),  // XOR A, D
			0xAB => self.xor_a_r(self.reg.e),  // XOR A, E
			0xAC => self.xor_a_r(self.reg.h),  // XOR A, H
			0xAD => self.xor_a_r(self.reg.l),  // XOR A, L
			0xAF => self.xor_a_r(self.reg.a),  // XOR A, A
			0xC1 => self.pop_rr(&mem_bus, "BC"),// POP BC
			0xC5 => self.push_rr(mem_bus, self.reg.get_bc()),	// PUSH BC
			0xC9 =>	// RET
			{
				self.reg.program_counter = self.pop_short(mem_bus);
				//println!("RET : {:0x}", self.reg.program_counter);
				16
			},
			0xCB => self.cb_inst_set(mem_bus),      // 0xCB INSTRCTION SET
			0xCD => // CALL d16
			{
				self.reg.program_counter += 2;
				self.push_short(mem_bus, self.reg.program_counter);
				self.reg.program_counter = nn;
				//println!("CALL : {:0x}", self.reg.program_counter);
				24

			},
			0xD1 => self.pop_rr(&mem_bus, "DE"),// POP DE
			0xD5 => self.push_rr(mem_bus, self.reg.get_de()),   // PUSH DE
			0xE0 => // LDH (a8),A
			{
				mem_bus.write_byte(0xFF00 + n as u16, self.reg.a);
				self.reg.program_counter += 1;
				12
			},
			0xE1 => self.pop_rr(&mem_bus, "HL"),// POP HL
			0xE2 => // LD (FF00+C), A   
			{
				mem_bus.write_byte(0xFF00 + self.reg.c as u16, self.reg.a);
				8
			},
			0xE5 => self.push_rr(mem_bus, self.reg.get_hl()),	// PUSH HL
			0xEA =>	// LD (a16), A
			{
				self.reg.program_counter += 2;
				mem_bus.write_byte(nn, self.reg.a);
				16	
			}
			0xF0 => // LDH A,(a8)
			{
				self.reg.a = mem_bus.read_byte(0xFF00 + n as u16);
				self.reg.program_counter += 1;
				12
			},
			0xF1 => self.pop_rr(&mem_bus, "AF"),// POP AF
			0xF2 => // LD A, (FF00+C) 
			{
				self.reg.a = mem_bus.read_byte(0xFF00 + self.reg.c as u16);
				8
			},
			0xF5 => self.push_rr(mem_bus, self.reg.get_af()),   // PUSH AF
			0xFE => // CP A, d8
			{
				self.reg.program_counter += 1;
				self.cp(n);
				8
			},
			_=> {
				println!("Unknown Opcode : {:02x}", opcode);
				process::exit(1);	
			},
		}
	}
	
	pub fn cb_inst_set(&mut self, mem_bus : &mut MemoryBus) -> u32
	{
		let op = mem_bus.read_byte(self.reg.program_counter);
		//println!("CB Opcode : {:02x}", op);    //DEBUG
		self.reg.program_counter += 1;

		match op 
		{
			0x11 => Cpu::rl_r(&mut self.reg.c, &mut self.reg.f),		// RL C
			0x7C => self.bit_test(self.reg.h, 7),				// BIT 7,H
			_=> {
				println!("Unknown Opcode : {:02x}", op);
				println!("STOPED AT PC : {:02x}", self.reg.program_counter);
				process::exit(1);
			},
		}

		if (op & 0x0F == 0x06) || (op & 0x0F == 0x0E)
		{
			return 16;
		}
		return 8;
	}

	pub fn bit_test(&mut self, reg : u8, n : u8)
	{
		self.reg.f.set_zero_flag((1 << n) & reg != 0);
		self.reg.f.set_sub_flag(false);
		self.reg.f.set_half_carry_flag(true);
	}

	pub fn xor_a_r(&mut self, reg : u8) -> u32
	{
		self.reg.a = self.reg.a ^ reg;
		self.reg.f.set_zero_flag(self.reg.a == 0);
		self.reg.f.set_sub_flag(false); 
		self.reg.f.set_half_carry_flag(false);
		self.reg.f.set_caryy_flag(false);
		return 4;
	}

	pub fn add_signed(val_1 : u16, val_2 : u8) -> u16
	{
		let signed = val_2 as i8;

		if signed >= 0
		{
			return val_1 + val_2 as u16; 
		}
		return (val_1 as i32 + signed as i32) as u16;
	}

	pub fn inc_r(flags : &mut Flag, reg : &mut u8) -> u32
	{   
		flags.set_half_carry_flag((*reg & 0xF) == 0xF);
		*reg += 1;
		flags.set_zero_flag(*reg == 0);
		flags.set_sub_flag(false);

		return 4
	}

	pub fn inc_rr(&mut self, rr : &str) -> u32
	{
		match rr 
		{
			"BC" =>	self.reg.set_bc(self.reg.get_bc() + 1),
			"DE" =>	self.reg.set_de(self.reg.get_de() + 1),
			"HL" =>	self.reg.set_hl(self.reg.get_hl() + 1),
			"SP" =>	self.reg.stack_pointer += 1,
			_ => 
			{
				println!("Unknown register {} !", rr);
				process::exit(1);
			}	
		}
		8
	}

	pub fn dec_r(flags : &mut Flag, reg : &mut u8) -> u32
	{   
		*reg -= 1;
		flags.set_half_carry_flag((*reg & 0xF) == 0xF);
		flags.set_zero_flag(*reg == 0);
		flags.set_sub_flag(true);

		return 4
	}

	pub fn ld_r_to_mem_hl(&self, mem_bus : &mut MemoryBus, reg : u8) -> u32
	{
		mem_bus.write_byte(self.reg.get_hl(), reg);
		8
	}

	pub fn ld_mem_rr_to_a(&mut self, mem_bus : &MemoryBus, rr : u16) -> u32
	{
		self.reg.a = mem_bus.read_byte(rr);
		8
	}

	pub fn ld_mem_hl_to_r(&mut self, mem_bus : &MemoryBus, r : &str) -> u32
	{
		match r 
		{
			"A" =>	self.reg.a = mem_bus.read_byte(self.reg.get_hl()),
			"B" =>	self.reg.b = mem_bus.read_byte(self.reg.get_hl()),
			"C" =>	self.reg.c = mem_bus.read_byte(self.reg.get_hl()),
			"D" =>	self.reg.d = mem_bus.read_byte(self.reg.get_hl()),
			"H" =>	self.reg.h = mem_bus.read_byte(self.reg.get_hl()),
			"L" =>	self.reg.l = mem_bus.read_byte(self.reg.get_hl()),
			_ => 
			{
				println!("Unknown register {} !", r);
				process::exit(1);
			}	
		} 
		8
	}

	pub fn push_short(&mut self, mem_bus : &mut MemoryBus, short : u16)
	{
		self.reg.stack_pointer -= 2;
		mem_bus.write_short(self.reg.stack_pointer, short);
	}   

	pub fn pop_short(&mut self, mem_bus : &MemoryBus) -> u16
	{
		//println!("sp : {}", self.reg.stack_pointer);	//DEBUG
		let short = mem_bus.read_short(self.reg.stack_pointer);
		self.reg.stack_pointer += 2;
		return short;
	}
	
	pub fn pop_rr(&mut self, mem_bus : &MemoryBus, rr : &str) -> u32
	{
		let short = self.pop_short(mem_bus);
		match rr
		{
			"AF" =>	self.reg.set_af(short),
			"BC" =>	self.reg.set_bc(short),
			"DE" =>	self.reg.set_de(short),
			"HL" =>	self.reg.set_hl(short),
			_ => 
			{
				println!("REGISTER {} doesn't exist !", rr);
				process::exit(1);
			}
		}
		12
	}

	pub fn push_rr(&mut self, mem_bus : &mut MemoryBus, rr : u16) -> u32
	{
		self.push_short(mem_bus, rr);
		16
	}

	pub fn ld_r_to_r(r_dst : &mut u8, r_src : u8) -> u32
	{
		*r_dst = r_src;
		4
	}

	pub fn ld_n_to_r(r_dst : &mut u8, n : u8, pc : &mut u16) -> u32
	{
		*r_dst = n;
		*pc += 1;
		8
	}

	pub fn rl_r(r : &mut u8, f : &mut Flag)
	{
		let old_carry = f.carry_flag;
		f.set_caryy_flag((*r & 0x80) == 0x80);
		*r = *r << 1;
		if old_carry
		{
			*r |= 0x01;
		}
		f.set_zero_flag(*r == 0x00);
		f.set_sub_flag(false);
		f.set_half_carry_flag(false);
	}

	pub fn cp(&mut self, val : u8)
	{
		self.reg.f.set_zero_flag(self.reg.a == val);
		self.reg.f.set_sub_flag(true);
		self.reg.f.set_caryy_flag(self.reg.a < val);
		self.reg.f.set_half_carry_flag((self.reg.a & 0x0F) < (val & 0x0F))
	}

	pub fn cp_r(&mut self, r : u8) -> u32
	{
		self.cp(r);
		4
	}
}