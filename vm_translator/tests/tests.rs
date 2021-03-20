use vm_translator::Parser;
use vm_translator::Command;
use vm_translator::command_type;

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
    let parser = Parser::new(SIMPLE_ADD_INPUT);
    assert_eq!(stripped_input1, parser.to_string());
}

#[test]
fn test_command_parser() {
    let c_arithmetic_1 = "add";
    let c_push_1 = "push constant 7";
    let c_pop_1 = "pop this 6";
    // let c_label_1 = "";
    // let c_goto_1 = "";
    // let c_if_1 = "";
    // let c_function_1 = "";
    // let c_return_1 = "";
    // let c_call_1 = "";
    assert_eq!(
        Command::ARITHMETIC,
        command_type(c_arithmetic_1).unwrap()
    );
    assert_eq!(
        Command::PUSH,
        command_type(c_push_1).unwrap()
    );
    assert_eq!(
        Command::POP,
        command_type(c_pop_1).unwrap()
    );
    // assert_eq!(
    //     Command::LABEL,
    //     command_type(c_label_1).unwrap()
    // );
    // assert_eq!(
    //     Command::GOTO,
    //     command_type(c_goto_1).unwrap()
    // );
    // assert_eq!(
    //     Command::IF,
    //     command_type(c_if_1).unwrap()
    // );
    // assert_eq!(
    //     Command::FUNCTION,
    //     command_type(c_function_1).unwrap()
    // );
    // assert_eq!(
    //     Command::RETURN,
    //     command_type(c_return_1).unwrap()
    // );
    // assert_eq!(
    //     Command::CALL,
    //     command_type(c_call_1).unwrap()
    // );
}