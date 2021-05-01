use vm_translator::Parser;
use vm_translator::CodeWriter;
use vm_translator::Command;
use vm_translator::read_file;
use tempfile::NamedTempFile;

const SIMPLE_ADD_INPUT: &str = r#"
// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/07/StackArithmetic/SimpleAdd/SimpleAdd.vm

// Pushes and adds two constants.
push constant 7
push constant 8
add

"#;

#[test]
fn test_init_parser() {
    let stripped_input1 = r#"push constant 7
push constant 8
add
"#;
    let file = NamedTempFile::new().unwrap();
    let mut cw = CodeWriter::new(file.path());
    let parser = Parser::new("test", SIMPLE_ADD_INPUT, &mut cw);
    assert_eq!(stripped_input1, parser.to_string());
}

#[test]
fn test_command_parser() {
    let c_arithmetic_1 = "add";
    let c_push_1 = "push constant 7";
    let c_pop_1 = "pop this 6";
    let c_label_1 = "label LOOP_START";
    let c_goto_1 = "goto END_PROGRAM";
    let c_if_1 = "if-goto LOOP_START";
    let c_function_1 = "function SimpleFunction.test 2";
    let c_return_1 = "return";
    let c_call_1 = "call Main.fibonacci 1";
    assert_eq!(
        Command::ARITHMETIC,
        Parser::command_type(c_arithmetic_1).unwrap()
    );
    assert_eq!(
        Command::PUSHPOP,
        Parser::command_type(c_push_1).unwrap()
    );
    assert_eq!(
        Command::PUSHPOP,
        Parser::command_type(c_pop_1).unwrap()
    );
    assert_eq!(
        Command::LABEL,
        Parser::command_type(c_label_1).unwrap()
    );
    assert_eq!(
        Command::GOTO,
        Parser::command_type(c_goto_1).unwrap()
    );
    assert_eq!(
        Command::IF,
        Parser::command_type(c_if_1).unwrap()
    );
    assert_eq!(
        Command::FUNCTION,
        Parser::command_type(c_function_1).unwrap()
    );
    assert_eq!(
        Command::RETURN,
        Parser::command_type(c_return_1).unwrap()
    );
    assert_eq!(
        Command::CALL,
        Parser::command_type(c_call_1).unwrap()
    );
}

#[test]
fn test_push() {
    let input: &str = r#"push constant 7
"#;

let expected_output: &str = r#"@7
D=A
@SP
A=M
M=D
@SP
M=M+1
"#;

    let file = NamedTempFile::new().unwrap();
    let mut cw = CodeWriter::new(file.path());
    let mut parser = Parser::new("test", input, &mut cw);
    parser.to_assembler();
    let output = read_file(file.path().to_str().unwrap()).unwrap();
    assert_eq!(output, expected_output);
}

#[test]
fn test_add() {
    let input: &str = r#"push constant 7
push constant 8
add
"#;

let expected_output: &str = r#"@7
D=A
@SP
A=M
M=D
@SP
M=M+1
@8
D=A
@SP
A=M
M=D
@SP
M=M+1
@SP
M=M-1
A=M
D=M
@SP
M=M-1
A=M
D=D+M
@SP
A=M
M=D
@SP
M=M+1
"#;

    let file = NamedTempFile::new().unwrap();
    let mut cw = CodeWriter::new(file.path());
    let mut parser = Parser::new("test", input, &mut cw);
    parser.to_assembler();
    let output = read_file(file.path().to_str().unwrap()).unwrap();
    assert_eq!(output, expected_output);
}

#[test]
fn test_sub() {
    let input: &str = r#"push constant 8
push constant 7
sub
"#;

let expected_output: &str = r#"@8
D=A
@SP
A=M
M=D
@SP
M=M+1
@7
D=A
@SP
A=M
M=D
@SP
M=M+1
@SP
M=M-1
A=M
D=M
@SP
M=M-1
A=M
D=M-D
@SP
A=M
M=D
@SP
M=M+1
"#;

    let file = NamedTempFile::new().unwrap();
    let mut cw = CodeWriter::new(file.path());
    let mut parser = Parser::new("test", input, &mut cw);
    parser.to_assembler();
    let output = read_file(file.path().to_str().unwrap()).unwrap();
    assert_eq!(output, expected_output);
}

#[test]
fn test_neg() {
    let input: &str = r#"push constant 8
neg
"#;

let expected_output: &str = r#"@8
D=A
@SP
A=M
M=D
@SP
M=M+1
@SP
M=M-1
A=M
D=M
D=-D
@SP
A=M
M=D
@SP
M=M+1
"#;

    let file = NamedTempFile::new().unwrap();
    let mut cw = CodeWriter::new(file.path());
    let mut parser = Parser::new("test", input, &mut cw);
    parser.to_assembler();
    let output = read_file(file.path().to_str().unwrap()).unwrap();
    assert_eq!(output, expected_output);
}

#[test]
fn test_eq() {
    let input: &str = r#"push constant 8
push constant 7
eq
"#;

let expected_output: &str = r#"@8
D=A
@SP
A=M
M=D
@SP
M=M+1
@7
D=A
@SP
A=M
M=D
@SP
M=M+1
@SP
M=M-1
A=M
D=M
@SP
M=M-1
A=M
D=M-D
@EQ_TRUE_0
D;JEQ
D=0
@EQ_RESULT_0
0;JMP
(EQ_TRUE_0)
D=-1
(EQ_RESULT_0)
@SP
A=M
M=D
@SP
M=M+1
"#;

    let file = NamedTempFile::new().unwrap();
    let mut cw = CodeWriter::new(file.path());
    let mut parser = Parser::new("test", input, &mut cw);
    parser.to_assembler();
    let output = read_file(file.path().to_str().unwrap()).unwrap();
    assert_eq!(output, expected_output);
}

#[test]
fn test_lt() {
    let input: &str = r#"push constant 7
push constant 8
lt
"#;

let expected_output: &str = r#"@7
D=A
@SP
A=M
M=D
@SP
M=M+1
@8
D=A
@SP
A=M
M=D
@SP
M=M+1
@SP
M=M-1
A=M
D=M
@SP
M=M-1
A=M
D=M-D
@LT_TRUE_0
D;JLT
D=0
@LT_RESULT_0
0;JMP
(LT_TRUE_0)
D=-1
(LT_RESULT_0)
@SP
A=M
M=D
@SP
M=M+1
"#;

    let file = NamedTempFile::new().unwrap();
    let mut cw = CodeWriter::new(file.path());
    let mut parser = Parser::new("test", input, &mut cw);
    parser.to_assembler();
    let output = read_file(file.path().to_str().unwrap()).unwrap();
    assert_eq!(output, expected_output);
}

#[test]
fn test_gt() {
    let input: &str = r#"push constant 8
push constant 7
gt
"#;

let expected_output: &str = r#"@8
D=A
@SP
A=M
M=D
@SP
M=M+1
@7
D=A
@SP
A=M
M=D
@SP
M=M+1
@SP
M=M-1
A=M
D=M
@SP
M=M-1
A=M
D=M-D
@GT_TRUE_0
D;JGT
D=0
@GT_RESULT_0
0;JMP
(GT_TRUE_0)
D=-1
(GT_RESULT_0)
@SP
A=M
M=D
@SP
M=M+1
"#;

    let file = NamedTempFile::new().unwrap();
    let mut cw = CodeWriter::new(file.path());
    let mut parser = Parser::new("test", input, &mut cw);
    parser.to_assembler();
    let output = read_file(file.path().to_str().unwrap()).unwrap();
    assert_eq!(output, expected_output);
}

#[test]
fn test_or() {
    let input: &str = r#"push constant 8
push constant 7
or
"#;

let expected_output: &str = r#"@8
D=A
@SP
A=M
M=D
@SP
M=M+1
@7
D=A
@SP
A=M
M=D
@SP
M=M+1
@SP
M=M-1
A=M
D=M
@SP
M=M-1
A=M
D=D|M
@SP
A=M
M=D
@SP
M=M+1
"#;

    let file = NamedTempFile::new().unwrap();
    let mut cw = CodeWriter::new(file.path());
    let mut parser = Parser::new("test", input, &mut cw);
    parser.to_assembler();
    let output = read_file(file.path().to_str().unwrap()).unwrap();
    assert_eq!(output, expected_output);
}

#[test]
fn test_and() {
    let input: &str = r#"push constant 8
push constant 7
and
"#;

let expected_output: &str = r#"@8
D=A
@SP
A=M
M=D
@SP
M=M+1
@7
D=A
@SP
A=M
M=D
@SP
M=M+1
@SP
M=M-1
A=M
D=M
@SP
M=M-1
A=M
D=D&M
@SP
A=M
M=D
@SP
M=M+1
"#;

    let file = NamedTempFile::new().unwrap();
    let mut cw = CodeWriter::new(file.path());
    let mut parser = Parser::new("test", input, &mut cw);
    parser.to_assembler();
    let output = read_file(file.path().to_str().unwrap()).unwrap();
    assert_eq!(output, expected_output);
}

#[test]
fn test_not() {
    let input: &str = r#"push constant 8
not
"#;

let expected_output: &str = r#"@8
D=A
@SP
A=M
M=D
@SP
M=M+1
@SP
M=M-1
A=M
D=M
D=!D
@SP
A=M
M=D
@SP
M=M+1
"#;

    let file = NamedTempFile::new().unwrap();
    let mut cw = CodeWriter::new(file.path());
    let mut parser = Parser::new("test", input, &mut cw);
    parser.to_assembler();
    let output = read_file(file.path().to_str().unwrap()).unwrap();
    assert_eq!(output, expected_output);
}

#[test]
fn test_pop_local() {
    let input: &str = r#"push constant 7
pop local 0
"#;

let expected_output: &str = r#"@7
D=A
@SP
A=M
M=D
@SP
M=M+1
@SP
M=M-1
A=M
D=M
@LCL
A=M
M=D
"#;

    let file = NamedTempFile::new().unwrap();
    let mut cw = CodeWriter::new(file.path());
    let mut parser = Parser::new("test", input, &mut cw);
    parser.to_assembler();
    let output = read_file(file.path().to_str().unwrap()).unwrap();
    assert_eq!(output, expected_output);
}
