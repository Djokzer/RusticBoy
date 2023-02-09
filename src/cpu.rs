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
                self.reg.set_bc(mem_bus.read_short(nn));
                self.reg.program_counter += 2;
                12   
            },
            0x0e => // LD C, d8
            {
                self.reg.c = n;
                self.reg.program_counter += 1;
                8   
            },
            0x11 => // LD DE, d16
            {
                self.reg.set_de(mem_bus.read_short(nn));
                self.reg.program_counter += 2;
                12   
            },
            0x18 => // JR r8,
            {
                self.reg.program_counter = Cpu::add_signed(self.reg.program_counter, n as u16); //RELATIVE JUMP
                12
            },
            0x1e => // LD E, d8
            {
                self.reg.e = n;
                self.reg.program_counter += 1;
                8   
            },
            0x20 => // JR NZ, r8,
            {
                if !self.reg.f.zero_flag
                {
                    self.reg.program_counter = Cpu::add_signed(self.reg.program_counter, n as u16); //RELATIVE JUMP
                    return 12;
                }
                self.reg.program_counter += 1;
                return  8;
            },
            0x21 => // LD HL, d16
            {
                self.reg.set_hl(mem_bus.read_short(nn));
                self.reg.program_counter += 2;
                12   
            },
            0x22 => // LD HL+, A
            {
                self.reg.set_hl(self.reg.a as u16);
                self.reg.set_hl(self.reg.get_hl() + 1);
                8
            },
            0x28 => // JR Z, r8,
            {
                if self.reg.f.zero_flag
                {
                    self.reg.program_counter = Cpu::add_signed(self.reg.program_counter, n as u16); //RELATIVE JUMP
                    return 12;
                }
                self.reg.program_counter += 1;
                return  8;
            },
            0x2e => // LD L, d8
            {
                self.reg.l = n;
                self.reg.program_counter += 1;
                8   
            },
            0x30 => // JR NC, r8,
            {
                if !self.reg.f.carry_flag
                {
                    self.reg.program_counter = Cpu::add_signed(self.reg.program_counter, n as u16); //RELATIVE JUMP
                    return 12;
                }
                self.reg.program_counter += 1;
                return  8;
            },
            0x32 => // LD HL-, A
            {
                self.reg.set_hl(self.reg.a as u16);
                self.reg.set_hl(self.reg.get_hl() - 1);
                8
            },
            0x31 => // LD SP, d16
            {
                self.reg.stack_pointer = nn;
                self.reg.program_counter += 2;
                12   
            },
            0x38 => // JR C, r8,
            {
                if self.reg.f.carry_flag
                {
                    self.reg.program_counter = Cpu::add_signed(self.reg.program_counter, n as u16); //RELATIVE JUMP
                    return 12;
                }
                self.reg.program_counter += 1;
                return  8;
            },
            0x3e => // LD A, d8
            {
                self.reg.a = n;
                self.reg.program_counter += 1;
                8   
            },
            0xA8 => self.xor_a_r(self.reg.b),  // XOR A, B
            0xA9 => self.xor_a_r(self.reg.c),  // XOR A, C
            0xAA => self.xor_a_r(self.reg.d),  // XOR A, D
            0xAB => self.xor_a_r(self.reg.e),  // XOR A, E
            0xAC => self.xor_a_r(self.reg.h),  // XOR A, H
            0xAD => self.xor_a_r(self.reg.l),  // XOR A, L
            0xAF => self.xor_a_r(self.reg.a),  // XOR A, A
            0xCB => self.cb_inst_set(mem_bus),  // 0xCB INSTRCTION SET 
            0xE2 => // LD (FF00+C), A   
            {
                mem_bus.write_byte(0xFF00 + self.reg.c as u16, self.reg.a);
                8
            },
            0xF2 => // LD A, (FF00+C) 
            {
                self.reg.a = mem_bus.read_byte(0xFF00 + self.reg.c as u16);
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
        println!("CB Opcode : {:02x}", op);    //DEBUG
        self.reg.program_counter += 1;

        match op 
        {
            0x7C => self.bit_test(self.reg.h, 7),
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

    pub fn add_signed(val_1 : u16, val_2 : u16) -> u16
    {
        let signed = val_2 as i16;

        if signed >= 0
        {
            return val_1 + val_2; 
        }
        return val_1 - val_2;
    }

}