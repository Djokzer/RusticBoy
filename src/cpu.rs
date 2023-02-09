use crate::register;
use crate::register::*;
use crate::bus::*;
use std::process;

pub struct Cpu
{
    pub reg : Register,
}

impl Cpu
{
    pub fn init_cpu() -> Cpu
    {
        Cpu
        {
            reg : Register::init_register(),
        }
    }

    pub fn get_opcode(&mut self, mem_bus : &MemoryBus) -> u8
    {
        let opcode = mem_bus.read_byte(self.reg.program_counter);
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
                self.reg.set_bc(mem_bus.read_short(nn));
                self.reg.program_counter += 2;
                12   
            },
            0x11 => // LD DE, d16
            {
                self.reg.set_de(mem_bus.read_short(nn));
                self.reg.program_counter += 2;
                12   
            },
            0x21 => // LD HL, d16
            {
                self.reg.set_hl(mem_bus.read_short(nn));
                self.reg.program_counter += 2;
                12   
            },
            0x31 => // LD SP, d16
            {
                self.reg.stack_pointer = nn;
                self.reg.program_counter += 2;
                12   
            },
            0xA8 => self.xor_a_r(self.reg.b),  // XOR A, B
            0xA9 => self.xor_a_r(self.reg.c),  // XOR A, C
            0xAA => self.xor_a_r(self.reg.d),  // XOR A, D
            0xAB => self.xor_a_r(self.reg.e),  // XOR A, E
            0xAC => self.xor_a_r(self.reg.h),  // XOR A, H
            0xAD => self.xor_a_r(self.reg.l),  // XOR A, L
            0xAF => self.xor_a_r(self.reg.a),  // XOR A, A
            _=> {
                println!("Unknown Opcode : {:02x}", opcode);
                process::exit(1);
            },
        
        }
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

}