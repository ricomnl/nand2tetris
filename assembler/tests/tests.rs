use assembler::Code;
use assembler::Command;
use assembler::Parser;
use assembler::SymbolTable;
use assembler::command_type;
use std::collections::HashMap;

const INPUT_WITHOUT_SYMBOLS: &str = r#"// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/06/add/Add.asm

// Computes R0 = 2 + 3  (R0 refers to RAM[0])

@2
D=A
@3
D=D+A
@0
M=D

"#;

const INPUT_WITH_SYMBOLS: &str = r#"// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/06/max/Max.asm

// Computes R2 = max(R0, R1)  (R0,R1,R2 refer to RAM[0],RAM[1],RAM[2])

   @R0
   D=M              // D = first number
   @R1
   D=D-M            // D = first number - second number
   @OUTPUT_FIRST
   D;JGT            // if D>0 (first is greater) goto output_first
   @R1
   D=M              // D = second number
   @OUTPUT_D
   0;JMP            // goto output_d
(OUTPUT_FIRST)
   @R0             
   D=M              // D = first number
(OUTPUT_D)
   @R2
   M=D              // M[2] = D (greatest number)
(INFINITE_LOOP)
   @INFINITE_LOOP
   0;JMP            // infinite loop
"#;

const INPUT_WITH_SYMBOLS2: &str = r#"// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/06/rect/Rect.asm

// Draws a rectangle at the top-left corner of the screen.
// The rectangle is 16 pixels wide and R0 pixels high.

   @0
   D=M
   @INFINITE_LOOP
   D;JLE 
   @counter
   M=D
   @SCREEN
   D=A
   @address
   M=D
(LOOP)
   @address
   A=M
   M=-1
   @address
   D=M
   @32
   D=D+A
   @address
   M=D
   @counter
   MD=M-1
   @LOOP
   D;JGT
(INFINITE_LOOP)
   @INFINITE_LOOP
   0;JMP
"#;

#[test]
fn test_init_parser() {
    let stripped_input1 = r#"@2
D=A
@3
D=D+A
@0
M=D
"#;
    let parser = Parser::new(INPUT_WITHOUT_SYMBOLS);
    assert_eq!(stripped_input1, parser.to_string());
}

#[test]
fn test_command_parser() {
    let a_command_1 = "@2";
    let c_command_1 = "D=A";
    let c_command_2 = "D=D+A";
    let c_command_3 = "D;JGT";
    let l_command_1 = "(LOOP)";
    assert_eq!(
        Command::ACommand,
        command_type(a_command_1).unwrap()
    );
    assert_eq!(
        Command::CCommand,
        command_type(c_command_1).unwrap()
    );
    assert_eq!(
        Command::CCommand,
        command_type(c_command_2).unwrap()
    );
    assert_eq!(
        Command::CCommand,
        command_type(c_command_3).unwrap()
    );
    assert_eq!(
        Command::LCommand,
        command_type(l_command_1).unwrap()
    );
}

#[test]
fn test_valid_codes() {
    let c_command_1 = "D=A".to_string();
    let c_command_2 = "D=D+A".to_string();
    let c_command_3 = "M=D".to_string();
    let c_command_4 = "D;JGT".to_string();
    let code_parser = Code::new();
    assert_eq!("1110110000010000", code_parser.parse(c_command_1).unwrap());
    assert_eq!("1110000010010000", code_parser.parse(c_command_2).unwrap());
    assert_eq!("1110001100001000", code_parser.parse(c_command_3).unwrap());
    assert_eq!("1110001100000001", code_parser.parse(c_command_4).unwrap());
}

#[test]
#[should_panic]
fn test_invalid_code_nonexistent_comp() {
    let invalid_command = "Z=A".to_string();
    let code_parser = Code::new();
    code_parser.parse(invalid_command).unwrap();
}

#[test]
#[should_panic]
fn test_invalid_code_nonexistent_jmp() {
    let invalid_command = "D;JUMP".to_string();
    let code_parser = Code::new();
    code_parser.parse(invalid_command).unwrap();
}

#[test]
#[should_panic]
fn test_invalid_code_dest_and_jmp() {
    let invalid_command = "D=A;JGT".to_string();
    let code_parser = Code::new();
    code_parser.parse(invalid_command).unwrap();
}

#[test]
fn test_valid_address() {
    let address_1 = "@2".to_string();
    let mut address_parser = SymbolTable::new();
    let symbol = SymbolTable::parse_symbol(address_1, Command::ACommand).unwrap();
    assert_eq!(
        "0000000000000010",
        address_parser.get_address(&symbol).unwrap()
    );
}

#[test]
fn test_valid_long_address() {
    let address_1 = "@16000".to_string();
    let mut address_parser = SymbolTable::new();
    let symbol = SymbolTable::parse_symbol(address_1, Command::ACommand).unwrap();
    assert_eq!(
        "0011111010000000",
        address_parser.get_address(&symbol).unwrap()
    );
}

#[test]
fn test_valid_address_with_dot() {
    let address_1 = "@sys.init".to_string();
    let symbol = SymbolTable::parse_symbol(address_1, Command::ACommand).unwrap();
    assert_eq!("sys.init", symbol);
}

#[test]
#[should_panic]
fn test_invalid_address_no_int() {
    let invalid_address = "@".to_string();
    let mut address_parser = SymbolTable::new();
    let symbol = SymbolTable::parse_symbol(invalid_address, Command::ACommand).unwrap();
    address_parser.get_address(&symbol).unwrap();
}

#[test]
#[should_panic]
fn test_invalid_address_neg_int() {
    let invalid_address = "@-1".to_string();
    let mut address_parser = SymbolTable::new();
    let symbol = SymbolTable::parse_symbol(invalid_address, Command::ACommand).unwrap();
    address_parser.get_address(&symbol).unwrap();
}

#[test]
fn test_valid_symbol() {
    let symbol = "(LOOP)".to_string();
    assert_eq!(
        "LOOP",
        SymbolTable::parse_symbol(symbol, Command::LCommand).unwrap()
    );
}

#[test]
fn test_valid_symbol_with_dot() {
    let symbol = "(ball.new)".to_string();
    assert_eq!(
        "ball.new",
        SymbolTable::parse_symbol(symbol, Command::LCommand).unwrap()
    );
}

#[test]
fn test_valid_symbol_with_dot_and_dollar() {
    let symbol = "(ball.setdestination$if_true0)".to_string();
    assert_eq!(
        "ball.setdestination$if_true0",
        SymbolTable::parse_symbol(symbol, Command::LCommand).unwrap()
    );
}

#[test]
#[should_panic]
fn test_invalid_symbol_no_parentheses() {
    let symbol = "LOOP".to_string();
    SymbolTable::parse_symbol(symbol, Command::LCommand).unwrap();
}

#[test]
fn test_fill_symbol_table() {
    let test_symbol_map: HashMap<String, u32> = [
        (String::from("SP"), 0),
        (String::from("LCL"), 1),
        (String::from("ARG"), 2),
        (String::from("THIS"), 3),
        (String::from("THAT"), 4),
        (String::from("R0"), 0),
        (String::from("R1"), 1),
        (String::from("R2"), 2),
        (String::from("R3"), 3),
        (String::from("R4"), 4),
        (String::from("R5"), 5),
        (String::from("R6"), 6),
        (String::from("R7"), 7),
        (String::from("R8"), 8),
        (String::from("R9"), 9),
        (String::from("R10"), 10),
        (String::from("R11"), 11),
        (String::from("R12"), 12),
        (String::from("R13"), 13),
        (String::from("R14"), 14),
        (String::from("R15"), 15),
        (String::from("OUTPUT_FIRST"), 10),
        (String::from("OUTPUT_D"), 12),
        (String::from("INFINITE_LOOP"), 14),
        (String::from("SCREEN"), 16384),
        (String::from("KBD"), 24576),
    ]
    .iter()
    .cloned()
    .collect();
    let mut parser = Parser::new(INPUT_WITH_SYMBOLS);
    parser.fill_symbol_table().unwrap();
    assert_eq!(test_symbol_map, *parser.symbol_table().symbol_map());
}

#[test]
fn test_assembler_without_symbols() {
    let assembler_output = r#"0000000000000010
1110110000010000
0000000000000011
1110000010010000
0000000000000000
1110001100001000
"#;
    let mut parser = Parser::new(INPUT_WITHOUT_SYMBOLS);
    assert_eq!(assembler_output, parser.to_bytes().unwrap());
}

#[test]
fn test_assembler_with_symbols_max() {
    let assembler_output = r#"0000000000000000
1111110000010000
0000000000000001
1111010011010000
0000000000001010
1110001100000001
0000000000000001
1111110000010000
0000000000001100
1110101010000111
0000000000000000
1111110000010000
0000000000000010
1110001100001000
0000000000001110
1110101010000111
"#;
    let mut parser = Parser::new(INPUT_WITH_SYMBOLS);
    assert_eq!(assembler_output, parser.to_bytes().unwrap());
}

#[test]
fn test_assembler_with_symbols_rect() {
    let assembler_output = r#"0000000000000000
1111110000010000
0000000000010111
1110001100000110
0000000000010000
1110001100001000
0100000000000000
1110110000010000
0000000000010001
1110001100001000
0000000000010001
1111110000100000
1110111010001000
0000000000010001
1111110000010000
0000000000100000
1110000010010000
0000000000010001
1110001100001000
0000000000010000
1111110010011000
0000000000001010
1110001100000001
0000000000010111
1110101010000111
"#;
    let mut parser = Parser::new(INPUT_WITH_SYMBOLS2);
    assert_eq!(assembler_output, parser.to_bytes().unwrap());
}
