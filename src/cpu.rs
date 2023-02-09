use crate::register::*;
use crate::bus::*;

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
        

        return 1;
    }

}