use crate::cpu::Storage::*;
use crate::cpu::Instruction::{self, *};
use crate::register::{Flag, Register8::*, Register16::*};

// Array containing all the instructions indexed by opcode.
// Tuple format: (Instruction, number of cycles, human readable string)
// Does not include the CB instructions which will be stored in a different array.
// Idea: what if instead of the enum, the first item as fn(&mut cpu) -> () ?
// FIXME: OH GOD THE FORMATTING IS ALL BROKEN AAAAAAAAAA
pub static OPCODES: [(Instruction, u64, &'static str); 0x100] = [
    // 0x
    (NOP,            4,  "NOP"),
    (LdNN(BC),       12, "LD BC, nn"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (Inc16(BC),      8,  "INC BC"),
    (Inc(Register(B)),         4,  "INC B"),
    (Dec(Register(B)), 4,  "DEC B"),
    (LdN(B),         8,  "LD B, n"),
    (Rlca,           4,  "RLCA"),
    (Lda16Sp,        20, "LD (a16), SP"),
    (AddHlR16(BC),   8,  "ADD HL, BC"),
    (LdAR16(BC),     8,  "LD A, [BC]"),
    (Dec16(BC),     8,  "DEC BC"),
    (Inc(Register(C)), 4,  "INC C"),
    (Dec(Register(C)), 4,  "DEC C"),
    (LdN(C),         8,  "LD C, n"),
    (Rrca,           4,  "RRCA"),
    // 1x
    (NotImplemented, 4,  "STOP"),
    (LdNN(DE),       12, "LD DE, nn"),
    (LdR16A(DE),     8, "LD [DE], A"),
    (Inc16(DE),      8,  "INC DE"),
    (Inc(Register(D)),         4,  "INC D"),
    (Dec(Register(D)),         4,  "DEC D"),
    (LdN(D),         8,  "LD D, n"),
    (Rla,            4,  "RLA"),
    (JrE8,          12,  "JR e8"),
    (AddHlR16(DE),   8,  "ADD HL, DE"),
    (LdAR16(DE),     8,  "LD A, [DE]"),
    (Dec16(DE),     8,  "DEC DE"),
    (Inc(Register(E)),         4,  "INC E"),
    (Dec(Register(E)),         4,  "DEC E"),
    (LdN(E),         8,  "LD E, n"),
    (Rra,            4,  "RRA"),
    // 2x
    (Jr(Flag::Z, false), 8, "JR NZ, nn"),
    (LdNN(HL),       12, "LD HL, nn"),
    (LdiHlA,         8,  "LDI (HL), A"),
    (Inc16(HL),      8,  "INC HL"),
    (Inc(Register(H)),         4,  "INC H"),
    (Dec(Register(H)),         4,  "DEC H"),
    (LdN(H),         8,  "LD H, n"),
    (Daa,            4,  "DAA"),
    (Jr(Flag::Z, true), 8, "JR Z, nn"),
    (AddHlR16(HL),   8,  "ADD HL, HL"),
    (LdiAHl,         8,  "LDI A, (HL)"),
    (Dec16(HL),     8,  "DEC HL"),
    (Inc(Register(L)),         4,  "INC L"),
    (Dec(Register(L)), 4,  "DEC L"),
    (LdN(L),         8,  "LD L, n"),
    (Cpl,            4,  "CPL"),
    // 3x
    (Jr(Flag::C, false), 8, "JR NC, nn"),
    (LdSp,          12,  "LD SP, nn"),
    (LddHlA,         8,  "LDD (HL), A"),
    (Inc16(SP),      8,  "INC SP"),
    (Inc(Pointer(HL)),   8,  "INC (HL)"),
    (Dec(Pointer(HL)),   8,  "DEC (HL)"),
    (LdHlD8,        12,  "LD n, (HL)"),
    (Scf,            4,  "SCF"),
    (Jr(Flag::C, true), 8, "JR C, nn"),
    (AddHlR16(SP),   8,  "ADD HL, SP"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (Dec16(SP),     8,  "DEC SP"),
    (Inc(Register(A)),         4,  "INC A"),
    (Dec(Register(A)),         4,  "DEC A"),
    (LdN(A),         8,  "LD A, n"),
    (Ccf,            4,  "CCF"),
    // 4x
    (LdRR(B, B),     4,  "LD B, B"),
    (LdRR(B, C),     4,  "LD B, C"),
    (LdRR(B, D),     4,  "LD B, D"),
    (LdRR(B, E),     4,  "LD B, E"),
    (LdRR(B, H),     4,  "LD B, H"),
    (LdRR(B, L),     4,  "LD B, L"),
    (LdRHl(B),       8,  "LD B, (HL)"),
    (LdRR(B, A),     4,  "LD B, A"),
    (LdRR(C, B),     4,  "LD C, B"),
    (LdRR(C, C),     4,  "LD C, C"),
    (LdRR(C, D),     4,  "LD C, D"),
    (LdRR(C, E),     4,  "LD C, E"),
    (LdRR(C, H),     4,  "LD C, H"),
    (LdRR(C, L),     4,  "LD C, L"),
    (LdRHl(C),       8,  "LD C, (HL)"),
    (LdRR(C, A),     4,  "LD C, A"),
    // 5x
    (LdRR(D, B),     4,  "LD D, B"),
    (LdRR(D, C),     4,  "LD D, C"),
    (LdRR(D, D),     4,  "LD D, D"),
    (LdRR(D, E),     4,  "LD D, E"),
    (LdRR(D, H),     4,  "LD D, H"),
    (LdRR(D, L),     4,  "LD D, L"),
    (LdRHl(D),       8,  "LD D, (HL)"),
    (LdRR(D, A),     4,  "LD D, A"),
    (LdRR(E, B),     4,  "LD E, B"),
    (LdRR(E, C),     4,  "LD E, C"),
    (LdRR(E, D),     4,  "LD E, D"),
    (LdRR(E, E),     4,  "LD E, E"),
    (LdRR(E, H),     4,  "LD E, H"),
    (LdRR(E, L),     4,  "LD E, L"),
    (LdRHl(E),       8,  "LD E, (HL)"),
    (LdRR(E, A),     4,  "LD E, A"),
    // 6x
    (LdRR(H, B),     4,  "LD H, B"),
    (LdRR(H, C),     4,  "LD H, C"),
    (LdRR(H, D),     4,  "LD H, D"),
    (LdRR(H, E),     4,  "LD H, E"),
    (LdRR(H, H),     4,  "LD H, H"),
    (LdRR(H, L),     4,  "LD H, L"),
    (LdRHl(H),       8,  "LD H, (HL)"),
    (LdRR(H, A),     4,  "LD H, A"),
    (LdRR(L, B),     4,  "LD L, B"),
    (LdRR(L, C),     4,  "LD L, C"),
    (LdRR(L, D),     4,  "LD L, D"),
    (LdRR(L, E),     4,  "LD L, E"),
    (LdRR(L, H),     4,  "LD L, H"),
    (LdRR(L, L),     4,  "LD L, L"),
    (LdRHl(L),       8,  "LD L, (HL)"),
    (LdRR(L, A),     4,  "LD L, A"),
    // 7x
    (LdHlR(B),       8,  "LD (HL), B"),
    (LdHlR(C),       8,  "LD (HL), C"),
    (LdHlR(D),       8,  "LD (HL), D"),
    (LdHlR(E),       8,  "LD (HL), E"),
    (LdHlR(H),       8,  "LD (HL), H"),
    (LdHlR(L),       8,  "LD (HL), L"),
    (NotImplemented, 4,  "HALT"),
    (LdHlR(A),       8,  "LD (HL), A"),
    (LdRR(A, B),     4,  "LD A, B"),
    (LdRR(A, C),     4,  "LD A, C"),
    (LdRR(A, D),     4,  "LD A, D"),
    (LdRR(A, E),     4,  "LD A, E"),
    (LdRR(A, H),     4,  "LD A, H"),
    (LdRR(A, L),     4,  "LD A, L"),
    (LdRHl(A),       8,  "LD A, (HL)"),
    (LdRR(A, A),     4,  "LD A, A"),
    // 8x
    (Add(Register(B)), 4,  "ADD B"),
    (Add(Register(C)), 4,  "ADD C"),
    (Add(Register(D)), 4,  "ADD D"),
    (Add(Register(E)), 4,  "ADD E"),
    (Add(Register(H)), 4,  "ADD H"),
    (Add(Register(L)), 4,  "ADD L"),
    (Add(Pointer(HL)), 8,  "ADD (HL)"),
    (Add(Register(A)), 4,  "ADD A"),
    (Adc(Register(B)), 4,  "ADC B"),
    (Adc(Register(C)), 4,  "ADC C"),
    (Adc(Register(D)), 4,  "ADC D"),
    (Adc(Register(E)), 4,  "ADC E"),
    (Adc(Register(H)), 4,  "ADC H"),
    (Adc(Register(L)), 4,  "ADC L"),
    (Adc(Pointer(HL)), 8,  "ADC (HL)"),
    (Adc(Register(A)), 4,  "ADC A"),
    // 9x
    (Sub(Register(B)), 4,  "SUB B"),
    (Sub(Register(C)), 4,  "SUB C"),
    (Sub(Register(D)), 4,  "SUB D"),
    (Sub(Register(E)), 4,  "SUB E"),
    (Sub(Register(H)), 4,  "SUB H"),
    (Sub(Register(L)), 4,  "SUB L"),
    (Sub(Pointer(HL)), 8,  "SUB (HL)"),
    (Sub(Register(A)), 4,  "SUB A"),
    (Sbc(Register(B)), 4,  "SBC B"),
    (Sbc(Register(C)), 4,  "SBC C"),
    (Sbc(Register(D)), 4,  "SBC D"),
    (Sbc(Register(E)), 4,  "SBC E"),
    (Sbc(Register(H)), 4,  "SBC H"),
    (Sbc(Register(L)), 4,  "SBC L"),
    (Sbc(Pointer(HL)), 8,  "SBC (HL)"),
    (Sbc(Register(A)), 4,  "SBC A"),
    // Ax
    (And(Register(B)), 4,  "AND B"),
    (And(Register(C)), 4,  "AND C"),
    (And(Register(D)), 4,  "AND D"),
    (And(Register(E)), 4,  "AND E"),
    (And(Register(H)), 4,  "AND H"),
    (And(Register(L)), 4,  "AND L"),
    (And(Pointer(HL)), 8,  "AND (HL)"),
    (And(Register(A)), 4,  "AND A"),
    (Xor(B),         4,  "XOR B"),
    (Xor(C),         4,  "XOR C"),
    (Xor(D),         4,  "XOR D"),
    (Xor(E),         4,  "XOR E"),
    (Xor(H),         4,  "XOR H"),
    (Xor(L),         4,  "XOR L"),
    (XorHl,          8,  "XOR A, (HL)"),
    (Xor(A),         4,  "XOR A"),
    // Bx
    (Or(Register(B)), 4,  "OR B"),
    (Or(Register(C)), 4,  "OR C"),
    (Or(Register(D)), 4,  "OR D"),
    (Or(Register(E)), 4,  "OR E"),
    (Or(Register(H)), 4,  "OR H"),
    (Or(Register(L)), 4,  "OR L"),
    (Or(Pointer(HL)), 4,  "OR (HL)"),
    (Or(Register(A)), 4,  "OR A"),
    (Cp(Register(B)), 4,  "CP B"),
    (Cp(Register(C)), 4,  "CP C"),
    (Cp(Register(D)), 4,  "CP D"),
    (Cp(Register(E)), 4,  "CP E"),
    (Cp(Register(H)), 4,  "CP H"),
    (Cp(Register(L)), 4,  "CP L"),
    (Cp(Pointer(HL)), 4,  "CP (HL)"),
    (Cp(Register(A)), 4,  "CP A"),
    // Cx
    (RetCond(Flag::Z, false), 8, "RET NZ"),
    (Pop16(BC),     12,  "POP BC"),
    (JpCond(Flag::Z, false), 12,  "JP NZ"),
    (Jp,             16, "JP"),
    (CallCond(Flag::Z, false), 4,  "CALL NZ, d16"),
    (Push16(BC),    16,  "PUSH BC"),
    (Add(NextByte),  8,  "ADD d8"),
    (Rst(0x00),    16,  "RST 00H"),
    (RetCond(Flag::Z, true), 8, "RET Z"),
    (Ret,           16,  "RET"),
    (JpCond(Flag::Z, true), 12,  "JP Z"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (CallCond(Flag::Z, true), 4,  "CALL Z, d16"),
    (Call,          24,  "CALL d16"),
    (Adc(NextByte),  8,  "ADC d8"),
    (Rst(0x08),    16,  "RST 08H"),
    // Dx
    (RetCond(Flag::C, false), 8, "RET NC"),
    (Pop16(DE),     12,  "POP DE"),
    (JpCond(Flag::C, false), 12,  "JP NC"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (CallCond(Flag::C, false), 4,  "CALL NC, d16"),
    (Push16(DE),    16,  "PUSH DE"),
    (Sub(NextByte),  8,  "SUB d8"),
    (Rst(0x10),    16,  "RST 10H"),
    (RetCond(Flag::C, true), 8, "RET C"),
    (Reti,          16,  "RETI"),
    (JpCond(Flag::C, true), 12,  "JP C"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (CallCond(Flag::C, true), 4,  "CALL C, d16"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (Sbc(NextByte),  4,  "SBC d8"),
    (Rst(0x18),     16,  "RST 18H"),
    // Ex
    (LdWriteIoN,    12,  "LDH (FF00+n), A"),
    (Pop16(HL),     12,  "POP HL"),
    (LdWriteIoC,    12,  "LD (FF00+C), A"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (Push16(HL),    16,  "PUSH HL"),
    (And(NextByte),          4,  "AND d8"),
    (Rst(0x20),     16,  "RST 20H"),
    (AddSpE8,       16,  "ADD SP, e8"),
    (JpHl,           4,  "JP HL"),
    (Lda16A,        16,  "LD (a16), A"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (XorD8,          8,  "XOR d8"),
    (Rst(0x28),     16,  "RST 28H"),
    // Fx
    (LdReadIoN,     12,  "LDH A, (FF00+n)"),
    (Pop16(AF),     12,  "POP AF"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (Di,             4,  "DI"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (Push16(AF),    16,  "PUSH AF"),
    (Or(NextByte),   8,  "OR d8"),
    (Rst(0x30),     16,  "RST 30H"),
    (LdHlSpE8,      12,  "LD HL, sp+e8"),
    (LdSpHl,         8,  "LD SP, HL"),
    (LdAA16,        16,  "LD A,(a16)"),
    (Ei,             4,  "EI"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (Cp(NextByte),   8,  "CP n"),
    (Rst(0x38),     16,  "RST 38H"),
];

pub static CB_OPCODES: [(Instruction, u64, &'static str); 0x100] = [
    // 0x
    (Rlc(Register(B)), 8,  "RLC B"),
    (Rlc(Register(C)), 8,  "RLC C"),
    (Rlc(Register(D)), 8,  "RLC D"),
    (Rlc(Register(E)), 8,  "RLC E"),
    (Rlc(Register(H)), 8,  "RLC H"),
    (Rlc(Register(L)), 8,  "RLC L"),
    (Rlc(Pointer(HL)), 16, "RLC (HL)"),
    (Rlc(Register(A)), 8,  "RLC A"),
    (Rrc(Register(B)), 8,  "RrC B"),
    (Rrc(Register(C)), 8,  "RrC C"),
    (Rrc(Register(D)), 8,  "RrC D"),
    (Rrc(Register(E)), 8,  "RrC E"),
    (Rrc(Register(H)), 8,  "RrC H"),
    (Rrc(Register(L)), 8,  "RrC L"),
    (Rrc(Pointer(HL)), 16, "RrC (HL)"),
    (Rrc(Register(A)), 8,  "RrC A"),
    // 1x
    (Rl(Register(B)), 8,  "RL B"),
    (Rl(Register(C)), 8,  "RL C"),
    (Rl(Register(D)), 8,  "RL D"),
    (Rl(Register(E)), 8,  "RL E"),
    (Rl(Register(H)), 8,  "RL H"),
    (Rl(Register(L)), 8,  "RL L"),
    (Rl(Pointer(HL)), 16, "RL (HL)"),
    (Rl(Register(A)), 8,  "RL A"),
    (Rr(Register(B)), 8,  "RR B"),
    (Rr(Register(C)), 8,  "RR C"),
    (Rr(Register(D)), 8,  "RR D"),
    (Rr(Register(E)), 8,  "RR E"),
    (Rr(Register(H)), 8,  "RR H"),
    (Rr(Register(L)), 8,  "RR L"),
    (Rr(Pointer(HL)), 16, "RR (HL)"),
    (Rr(Register(A)), 8,  "RR A"),
    // 2x
    (Sla(Register(B)), 8,  "SLA B"),
    (Sla(Register(C)), 8,  "SLA C"),
    (Sla(Register(D)), 8,  "SLA D"),
    (Sla(Register(E)), 8,  "SLA E"),
    (Sla(Register(H)), 8,  "SLA H"),
    (Sla(Register(L)), 8,  "SLA L"),
    (Sla(Pointer(HL)), 16, "SLA (HL)"),
    (Sla(Register(A)), 8,  "SLA A"),
    (Sra(Register(B)), 8,  "SRA B"),
    (Sra(Register(C)), 8,  "SRA C"),
    (Sra(Register(D)), 8,  "SRA D"),
    (Sra(Register(E)), 8,  "SRA E"),
    (Sra(Register(H)), 8,  "SRA H"),
    (Sra(Register(L)), 8,  "SRA L"),
    (Sra(Pointer(HL)), 16, "SRA (HL)"),
    (Sra(Register(A)), 8,  "SRA A"),
    // 3x
    (Swap(Register(B)), 8,  "SWAP B"),
    (Swap(Register(C)), 8,  "SWAP C"),
    (Swap(Register(D)), 8,  "SWAP D"),
    (Swap(Register(E)), 8,  "SWAP E"),
    (Swap(Register(H)), 8,  "SWAP H"),
    (Swap(Register(L)), 8,  "SWAP L"),
    (Swap(Pointer(HL)), 16,  "SWAP (HL)"),
    (Swap(Register(A)), 8,  "SWAP A"),
    (Srl(B),         8,  "SRL B"),
    (Srl(C),         8,  "SRL C"),
    (Srl(D),         8,  "SRL D"),
    (Srl(E),         8,  "SRL E"),
    (Srl(H),         8,  "SRL H"),
    (Srl(L),         8,  "SRL L"),
    (NotImplemented, 4,  "SRL (HL)"),
    (Srl(A),         8,  "SRL A"),
    // 4x
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    // 5x
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    // 6x
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    // 7x
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    // 8x
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    // 9x
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    // Ax
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    // Bx
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    // Cx
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    // Dx
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    // Ex
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    // Fx
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
];
