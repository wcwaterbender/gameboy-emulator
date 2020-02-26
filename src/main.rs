const ZERO_FLAG_BYTE_POSITION: u8 = 7;
const SUBTRACT_FLAG_BYTE_POSITION: u8 = 6;
const HALF_CARRY_FLAG_BYTE_POSITION: u8 = 5;
const CARRY_FLAG_BYTE_POSITION: u8 = 4;


//DMG-01 has 8 8-bit physical register to operate with
struct Registers {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: FlagsRegister, 
    h: u8,
    l: u8,
    sp: u16,
}

//however there are instructions that allow for 16-bit operations on virtual registers
//virtual registers are af, bc, de, and hl

impl Registers{
    fn get_bc(&self) -> u16 {
        (self.b as u16) << 8
        | self.c as u16
    }

    fn set_bc(&mut self, value: u16){
        self.b = ((value & 0xFF00) >> 8) as u8;
        self.c = (value & 0xFF) as u8; 
    }

    fn get_af(&self) -> u16 {
        (self.a as u16) << 8
        | u8::from(&self.f) as u16
    }

    fn set_af(&mut self, value: u16){
        self.a = ((value & 0xFF00) >> 8) as u8;
        self.f = FlagsRegister::from((value & 0xFF) as u8); 
    }

    fn get_de(&self) -> u16 {
        (self.d as u16) << 8
        | self.e as u16
    }

    fn set_de(&mut self, value: u16){
        self.d = ((value & 0xFF00) >> 8) as u8;
        self.e = (value & 0xFF) as u8; 
    }

    fn get_hl(&self) -> u16 {
        (self.h as u16) << 8
        | self.l as u16
    }

    fn set_hl(&mut self, value: u16){
        self.h = ((value & 0xFF00) >> 8) as u8;
        self.l = (value & 0xFF) as u8; 
    }
}


//f register is the flags register, so special handling
struct FlagsRegister {
    zero: bool,
    subtract: bool,
    half_carry: bool,
    carry: bool
}

impl std::convert::From<&FlagsRegister> for u8 {
    fn from(flag: &FlagsRegister) -> u8 {
       (if flag.zero { 1 } else { 0 })          << ZERO_FLAG_BYTE_POSITION |
       (if flag.subtract { 1 } else { 0 })      << SUBTRACT_FLAG_BYTE_POSITION |
       (if flag.half_carry { 1 } else { 0 })    << HALF_CARRY_FLAG_BYTE_POSITION |
       (if flag.carry { 1 } else { 0 })         << CARRY_FLAG_BYTE_POSITION 
    }
}

impl std::convert::From<u8> for FlagsRegister {
    fn from(byte: u8) -> Self {

        let zero = (byte & 0xFF) >> ZERO_FLAG_BYTE_POSITION != 0;
        let subtract = (byte & 0xFF) >> SUBTRACT_FLAG_BYTE_POSITION != 0;
        let half_carry = (byte & 0xFF) >> HALF_CARRY_FLAG_BYTE_POSITION != 01;
        let carry = (byte & 0xFF) >> CARRY_FLAG_BYTE_POSITION != 0;

        FlagsRegister{
            zero, 
            subtract,
            half_carry,
            carry
        }
    }
}

struct CPU {
    registers: Registers,
    pc: u16,
    bus: MemoryBus
}

struct MemoryBus {
    memory: [u8; 0xFFFF]
}
  
impl MemoryBus {
    fn read_byte(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }
}

//enumerate instructions for CPU
enum Instruction {
    ADD(ArithmeticTarget), //add to A reg
    ADDHL(ArithmeticTarget), //add to HL reg
    ADC(ArithmeticTarget), //add with carry
    SUBTRACT(ArithmeticTarget), //subtract from A reg
    SBC(ArithmeticTarget), //subtract with carry
    AND(ArithmeticTarget), //logical AND with A reg
    OR(ArithmeticTarget), //logical OR with A reg
    XOR(ArithmeticTarget), //logical XOR with A reg
    CP(ArithmeticTarget), //compare (SUB but subtraction not stored in A reg)
    INC(ArithmeticTarget), //increment target by 1
    DEC(ArithmeticTarget), //decrement target by 1
    CCF(ArithmeticTarget), //complement carry flag (toggle carry flag)
    SCF(ArithmeticTarget), //set carry flag to true
    RRA(ArithmeticTarget), //rotate right A reg
    RLA(ArithmeticTarget), //(rotate left A register) - bit rotate A register left through the carry flag
    RRCA(ArithmeticTarget), //(rotate right A register) - bit rotate A register right (not through the carry flag)
    RRLA(ArithmeticTarget), //(rotate right A register) - bit rotate A register right (not through the carry flag)
    CPL(ArithmeticTarget), //(complement) - toggle every bit of the A register
    BIT(ArithmeticTarget), //(bit test) - test to see if a specific bit of a specific register is set
    RESET(ArithmeticTarget), //(bit reset) - set a specific bit of a specific register to 0
    SET(ArithmeticTarget), //(bit set) - set a specific bit of a specific register to 1
    SRL(ArithmeticTarget), // (shift right logical) - bit shift a specific register right by 1
    RR(ArithmeticTarget), // (rotate right) - bit rotate a specific register right by 1 through the carry flag
    RL(ArithmeticTarget), //(rotate left) - bit rotate a specific register left by 1 through the carry flag
    RRC(ArithmeticTarget), // (rotate right) - bit rotate a specific register right by 1 (not through the carry flag)
    RLC(ArithmeticTarget), // (rotate left) - bit rotate a specific register left by 1 (not through the carry flag)
    SRA(ArithmeticTarget), //(shift right arithmetic) - arithmetic shift a specific register right by 1
    SLA(ArithmeticTarget), //(shift left arithmetic) - arithmetic shift a specific register left by 1
    SWAP(ArithmeticTarget) //(swap nibbles) - switch upper and lower nibble of a specific register
}

enum ArithmeticTarget {
    A,B,C,D,E,H,L,BC,DE,HL,SP
}

impl CPU {
    fn execute (&mut self, instruction: Instruction) {
        match instruction {
            Instruction::ADD(target) =>{
                match target {
                    ArithmeticTarget::A => {
                        let value = self.registers.a;
                        let new_value = self.add(value);
                        self.registers.a = new_value;
                    }
                    ArithmeticTarget::B => {
                        let value = self.registers.b;
                        let new_value = self.add(value);
                        self.registers.a = new_value;
                    }
                    ArithmeticTarget::C => {
                        let value = self.registers.c;
                        let new_value = self.add(value);
                        self.registers.a = new_value;
                    }
                    ArithmeticTarget::D => {
                        let value = self.registers.d;
                        let new_value = self.add(value);
                        self.registers.a = new_value;
                    }
                    ArithmeticTarget::E => {
                        let value = self.registers.e;
                        let new_value = self.add(value);
                        self.registers.a = new_value;
                    }
                    ArithmeticTarget::H => {
                        let value = self.registers.h;
                        let new_value = self.add(value);
                        self.registers.a = new_value;
                    }
                    ArithmeticTarget::L => {
                        let value = self.registers.l;
                        let new_value = self.add(value);
                        self.registers.a = new_value;
                    }
                    //more targets
                }

            }
            Instruction::ADDHL(target) => {
                match target {
                    ArithmeticTarget::BC => {
                        let value = self.registers.get_bc();
                        let new_value = self.addhl(value);
                        self.registers.set_hl(new_value);
                    }
                    ArithmeticTarget::DE => {
                        let value = self.registers.get_de();
                        let new_value = self.addhl(value);
                        self.registers.set_hl(new_value);
                    }
                    ArithmeticTarget::HL => {
                        let value = self.registers.get_hl();
                        let new_value = self.addhl(value);
                        self.registers.set_hl(new_value);
                    }
                    ArithmeticTarget::SP => {
                        let value = self.registers.sp;
                        let new_value = self.addhl(value);
                        self.registers.set_hl(new_value);
                    }
                    
                    //more targets
                }
                //more instructions
            }
        }
    }
    

    fn add(&mut self, value: u8) -> u8{
        let (new_value, did_overflow) = self.registers.a.overflowing_add(value);
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.carry = did_overflow;
        self.registers.f.half_carry = (self.registers.a & 0xF) + (value & 0xF) > 0xF;
        new_value
    }

    fn addhl(&mut self, value: u16) -> u16{
        let (new_value, did_overflow) = self.registers.get_hl().overflowing_add(value);
        self.registers.f.subtract = false;
        self.registers.f.carry = did_overflow;
        self.registers.f.half_carry = (self.registers.get_hl() & 0xF) + (value & 0xF) > 0xF; //TODO look this up
        new_value
    }
}

fn main() {
    println!("Hello, world!");
}
