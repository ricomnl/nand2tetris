#[macro_use]
extern crate simple_error;
extern crate regex;
use lazy_static::lazy_static;
use regex::Regex;
use std::error::Error;
use std::fmt;
use std::fs;
use std::path::Path;
use std::collections::HashMap;
use std::io::{BufRead, Cursor, LineWriter, Write};

type BoxResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, PartialEq)]
pub enum Command {
    ARITHMETIC,
    PUSH_POP,
    // LABEL,
    // GOTO,
    // IF,
    // FUNCTION,
    // RETURN,
    // CALL
}

lazy_static! {
    static ref RE_ARITHMETIC: Regex = Regex::new(r"^(?P<op>(add|sub|neg|eq|gt|lt|and|or|not))").unwrap();
    static ref RE_PUSH_POP: Regex = Regex::new(r"^(?P<command>(push|pop)).(?P<segment>(argument|local|static|constant|this|that|pointer|temp)).(?P<index>\d+)").unwrap();
}

pub struct CodeWriter {
    out_writer: LineWriter<fs::File>,
    operator_count: HashMap<String, u32>,
}

impl CodeWriter {
    pub fn new(filepath: &Path) -> Self {
        let fw = fs::File::create(filepath).expect("Unable to create file");
        let fw = LineWriter::new(fw);
        let operator_count = HashMap::new();
        Self {
            out_writer: fw,
            operator_count,
        }
    }

    pub fn write(&mut self, line: &str) {
        self.out_writer.write(&line.as_bytes()).expect("Unable to write line");
    }

    pub fn write_push_pop(&mut self, name: &str, command_type: &str, segment: &str, index: usize) {
        // TODO: can this be done with overloading?
        // Store the value temporarily with @index and D=A
        // sets the address to the Stack Pointer address with @0 and A=M
        // sets the value at M[Address] to temporarily stored value with M=D
        // increases stack pointer with @0 and M=M+1

        // if segment type is static, add symbol
        if segment == "static" {
            self.write(&format!("({}.{})\n", name, index));
        }

        let segment_str = match segment {
            "local" => String::from("@LCL\nA=M"),
            "argument" => String::from("@ARG\nA=M"),
            "this" => String::from("@THIS\nA=M"),
            "that" => String::from("@THAT\nA=M"),
            "pointer" => String::from("@3"),
            "temp" => String::from("@5"),
            "static" => format!("@{}.{}", name, index),
            _ => String::from("constant"),
        };

        let mut index_str = String::from("");
        // if segment is static, the index is not used in the usual way to address the memory
        if segment != "static" {
            index_str = "A=A+1\n".repeat(index);
        }

        let push_value_str = match segment {
            "constant" => format!("@{}\nD=A", index),
            "static" => format!("@{}.{}\nD=M", name, index),
            _ => format!("{}\n{}D=M", segment_str, index_str)
        };
        
        let translated_code = match command_type {
            "push" => format!(r#"{}
@SP
A=M
M=D
@SP
M=M+1
"#, push_value_str),
            "pop" => format!(r#"@SP
M=M-1
A=M
D=M
{}
{}M=D
"#, segment_str, index_str),
            _ => String::from(""),
        };
        self.write(&translated_code);
    }

    pub fn write_arithmetic(&mut self, operator: &str) {
        // Gets the element on top of the stack and stores it temporarily with @0; A=M and D=M (M[address])
        // (+|-|eq|gt|lt|and|or) adds the topmost element to the second element from the top at M[address]
        // Writes the result back to the current stack top
        // Increases the stack pointer
        let get_first = r#"@SP
M=M-1
A=M
D=M
"#;
        // if not unary operator (neg and not), get second operand
        let mut get_second = "";
        if operator != "neg" && operator != "not" {
            get_second = r#"@SP
M=M-1
A=M
"#;
        }
        
        let current_count = self.operator_count.entry(operator.to_string()).or_insert(0);
        let operation = match operator {
            "add" => String::from("D=D+M"),
            "sub" => String::from("D=M-D"),
            "neg" => String::from("D=-D"),
            "eq" => format!(r#"D=M-D
@EQ_TRUE_{n}
D;JEQ
D=0
@EQ_RESULT_{n}
0;JMP
(EQ_TRUE_{n})
D=-1
(EQ_RESULT_{n})"#, n=current_count),
            "gt" => format!(r#"D=M-D
@GT_TRUE_{n}
D;JGT
D=0
@GT_RESULT_{n}
0;JMP
(GT_TRUE_{n})
D=-1
(GT_RESULT_{n})"#, n=current_count),
            "lt" => format!(r#"D=M-D
@LT_TRUE_{n}
D;JLT
D=0
@LT_RESULT_{n}
0;JMP
(LT_TRUE_{n})
D=-1
(LT_RESULT_{n})"#, n=current_count),
            "and" => String::from("D=D&M"),
            "or" => String::from("D=D|M"),
            "not" => String::from("D=!D"),
            _ => String::from(""),
        };
        // increase operator counter for given operator
        *current_count += 1;
        
        let writeAndIncr = r#"@SP
A=M
M=D
@SP
M=M+1
"#;

        let translated_code = format!("{}{}{}\n{}", get_first, get_second, operation, writeAndIncr);
        
        self.write(&translated_code);
    }

    pub fn close(&mut self) {
        self.out_writer.flush().expect("Unable to flush line writer");
    }
}

pub struct Parser<'a> {
    name: &'a str,
    cursor: Cursor<String>,
    length: u64,
    code_writer: &'a mut CodeWriter
}

impl<'a> Parser<'a> {
    pub fn new(name: &'a str, input: &str, code_writer: &'a mut CodeWriter) -> Self {
        let content = input
            .lines()
            .filter(|line| !line.is_empty() && !line.starts_with("//"))
            .fold(String::new(), |acc, line| acc + line + "\n");
        let length = content.len() as u64;
        Self {
            name,
            cursor: Cursor::new(content),
            length,
            code_writer,
        }
    }

    fn has_more_commands(&self) -> bool {
        self.cursor.position() < self.length
    }

    fn read_next(&mut self) -> Result<String, &str> {
        let mut buffer = String::new();
        match self.cursor.read_line(&mut buffer) {
            Ok(_) => Ok(buffer),
            Err(_) => Err("Failed reading the next command."),
        }
    }

    pub fn to_assembler(&mut self) -> Result<(), Box<dyn Error>> {
        while self.has_more_commands() {
            let command = self.read_next().unwrap_or_else(|err| err.to_string());
            let parsed = match Parser::command_type(&command) {
                Ok(Command::PUSH_POP) => {
                    let regex = RE_PUSH_POP.captures(&command).unwrap();
                    let command_type = regex.name("command").unwrap();
                    let segment = regex.name("segment").unwrap();
                    let index = regex.name("index").unwrap();
                    self.code_writer.write_push_pop(self.name, command_type.as_str(), segment.as_str(), index.as_str().parse::<usize>().unwrap());
                },
                Ok(Command::ARITHMETIC) => {
                    let regex = RE_ARITHMETIC.captures(&command).unwrap();
                    let operator = regex.name("op").unwrap();
                    self.code_writer.write_arithmetic(operator.as_str());
                },
                Err(e) => return Err(e),
            };
            // bytes.push_str(&parsed.unwrap());
            // bytes.push_str("\n");
        }
        Ok(())
    }

    pub fn command_type(command: &str) -> BoxResult<Command> {
        match command {
            command if RE_ARITHMETIC.is_match(command) => Ok(Command::ARITHMETIC),
            command if RE_PUSH_POP.is_match(command) => Ok(Command::PUSH_POP),
            // command if RE_LABEL.is_match(command) => Ok(Command::LABEL),
            // command if RE_GOTO.is_match(command) => Ok(Command::GOTO),
            // command if RE_IF.is_match(command) => Ok(Command::IF),
            // command if RE_FUNCTION.is_match(command) => Ok(Command::FUNCTION),
            // command if RE_RETURN.is_match(command) => Ok(Command::RETURN),
            // command if RE_CALL.is_match(command) => Ok(Command::CALL),
            _ => bail!("Invalid input."),
        }
    }
}

impl fmt::Display for Parser<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let string = self.cursor.clone().into_inner();
        fmt.write_str(&string)?;
        Ok(())
    }
}

pub fn read_file(filename: &str) -> Result<String, Box<dyn Error>> {
    let content = fs::read_to_string(filename)?;

    Ok(content)
}

pub fn write_file(filename: &str, content: &str) -> Result<(), Box<dyn Error>> {
    fs::write(filename, content)?;

    Ok(())
}

pub fn parse_input(args: &[String]) -> Result<&str, &str> {
    if args.len() < 2 {
        return Err("not enough arguments");
    }
    let filename = &args[1];

    Ok(filename)
}
