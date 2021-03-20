#[macro_use]
extern crate simple_error;
extern crate regex;
use lazy_static::lazy_static;
use regex::Regex;
use std::error::Error;
use std::fmt;
use std::fs;
use std::io::{BufRead, Cursor, LineWriter, Write};

type BoxResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, PartialEq)]
pub enum Command {
    ARITHMETIC,
    PUSH,
    POP,
    // LABEL,
    // GOTO,
    // IF,
    // FUNCTION,
    // RETURN,
    // CALL
}

lazy_static! {
    static ref RE_ARITHMETIC: Regex = Regex::new(r"^(?P<type>(add|sub|neg|eq|gt|lt|and|or|not))").unwrap();
    static ref RE_PUSH: Regex = Regex::new(r"^push.(?P<segment>(argument|local|static|constant|this|that|pointer|temp)).(?P<index>\d+)").unwrap();
    static ref RE_POP: Regex = Regex::new(r"^pop.(?P<segment>(argument|local|static|constant|this|that|pointer|temp)).(?P<index>\d+)").unwrap();
}

pub fn command_type(command: &str) -> BoxResult<Command> {
    match command {
        command if RE_ARITHMETIC.is_match(command) => Ok(Command::ARITHMETIC),
        command if RE_PUSH.is_match(command) => Ok(Command::PUSH),
        command if RE_POP.is_match(command) => Ok(Command::POP),
        // command if RE_LABEL.is_match(command) => Ok(Command::LABEL),
        // command if RE_GOTO.is_match(command) => Ok(Command::GOTO),
        // command if RE_IF.is_match(command) => Ok(Command::IF),
        // command if RE_FUNCTION.is_match(command) => Ok(Command::FUNCTION),
        // command if RE_RETURN.is_match(command) => Ok(Command::RETURN),
        // command if RE_CALL.is_match(command) => Ok(Command::CALL),
        _ => bail!("Invalid input."),
    }
}

pub struct CodeWriter {
    out_writer: LineWriter<fs::File>,
}

impl CodeWriter {
    pub fn new(filename: &str) -> Self {
        let fw = fs::File::create(filename).expect("Unable to create file");
        let fw = LineWriter::new(fw);
        Self {
            out_writer: fw
        }
    }

    pub fn write(&mut self, line: &str) {
        self.out_writer.write(&line.as_bytes()).expect("Unable to write line");
    }

    pub fn close(&mut self) {
        self.out_writer.flush().expect("Unable to flush line writer");
    }
}

pub struct Parser<'a> {
    cursor: Cursor<String>,
    length: u64,
    code_writer: &'a mut CodeWriter
}

impl<'a> Parser<'a> {
    pub fn new(input: &str, code_writer: &'a mut CodeWriter) -> Self {
        let content = input
            .lines()
            .filter(|line| !line.is_empty() && !line.starts_with("//"))
            .fold(String::new(), |acc, line| acc + line + "\n");
        let length = content.len() as u64;
        Self {
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

    pub fn to_assembler(&mut self) {
        while self.has_more_commands() {
            let command = self.read_next().unwrap_or_else(|err| err.to_string());
            self.code_writer.write(&command);
            // let parsed = match command_type(&command) {
            //     Ok(command_type @ Command::ACommand) => {
            //         let symbol = SymbolTable::parse_symbol(command, command_type).unwrap();
            //         self.symbol_table.get_address(&symbol)
            //     }
            //     Ok(Command::CCommand) => self.code_parser.parse(command),
            //     Ok(Command::LCommand) => continue,
            //     Err(e) => Err(e),
            // };
            // bytes.push_str(&parsed.unwrap());
            // bytes.push_str("\n");
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
