#[macro_use]
extern crate simple_error;
extern crate regex;
use std::error::Error;
use std::fs;
use std::io::{Cursor, BufRead};
use std::fmt;
use regex::Regex;
use std::collections::HashMap;
use lazy_static::lazy_static;

type BoxResult<T> = Result<T,Box<dyn Error>>;

lazy_static! {
    static ref A_REGEX: Regex = Regex::new(r"@(?P<symbol>(\d+|(\w+\.?\w*\$?\w*)))").unwrap();
    static ref C_REGEX: Regex = Regex::new(r"((?P<dest>[AMD]{1,3})=(?P<comp1>(\d+|\w+)?[-!+&|]?(\d+|\w+)?))|((?P<comp2>(\d+|\w+)?[-!+&|]?(\d+|\w+)?);(?P<jump>(JGT|JEQ|JGE|JLT|JNE|JLE|JMP)))").unwrap();
    static ref L_REGEX: Regex = Regex::new(r"\((?P<symbol>\w+\.?\w*\$?\w*)\)").unwrap();
}

#[derive(Debug, PartialEq)]
pub enum Command {
	ACommand,
	CCommand,
	LCommand,
}

pub struct Code<'a> {
	a_comp_map: HashMap<&'a str, &'a str>,
	m_comp_map: HashMap<&'a str, &'a str>,
	dest_map: HashMap<&'a str, &'a str>,
	jump_map: HashMap<&'a str, &'a str>,
}

impl<'a> Code<'a> {
	pub fn new() -> Self {
		let a_comp_map: HashMap<&str, &str> = [
			 ("0"   , "101010"),
		     ("1"   , "111111"),
		     ("-1"  , "111010"),
		     ("D"   , "001100"),
		     ("A"   , "110000"),
		     ("!D"  , "001101"),
		     ("!A"  , "110001"),
		     ("-D"  , "001111"),
		     ("-A"  , "110011"),
		     ("D+1" , "011111"),
		     ("A+1" , "110111"),
		     ("D-1" , "001110"),
		     ("A-1" , "110010"),
		     ("D+A" , "000010"),
		     ("D-A" , "010011"),
		     ("A-D" , "000111"),
		     ("D&A" , "000000"),
		     ("D|A" , "010101")].iter().cloned().collect();

		let m_comp_map: HashMap<&str, &str> = [
			 ("M"   , "110000"),
		     ("!M"  , "110001"),
		     ("-M"  , "110011"),
		     ("M+1" , "110111"),
		     ("M-1" , "110010"),
		     ("D+M" , "000010"),
		     ("D-M" , "010011"),
		     ("M-D" , "000111"),
		     ("D&M" , "000000"),
		     ("D|M" , "010101")].iter().cloned().collect();

		let dest_map: HashMap<&str, &str> = [
			 (""    , "000"),
		     ("M"   , "001"),
		     ("D"   , "010"),
		     ("MD"  , "011"),
		     ("A"   , "100"),
		     ("AM"  , "101"),
		     ("AD"  , "110"),
		     ("AMD" , "111")].iter().cloned().collect();

		let jump_map: HashMap<&str, &str> = [
 			 (""    , "000"),
		     ("JGT" , "001"),
		     ("JEQ" , "010"),
		     ("JGE" , "011"),
		     ("JLT" , "100"),
		     ("JNE" , "101"),
		     ("JLE" , "110"),
		     ("JMP" , "111")].iter().cloned().collect();

		Self {
			a_comp_map,
			m_comp_map,
			dest_map,
			jump_map,
		}
	}

 	fn get_comp(&self, code: &str) -> (&str, &str) {
		let mut a_comp = "0";
		let comp;
		if code.contains("M") {
			a_comp = "1";
			comp = self.m_comp_map.get(code).unwrap_or(&"");
		} else {
			comp = self.a_comp_map.get(code).unwrap_or(&"");
		}
		(comp, a_comp)
	}

	fn get_dest(&self, code: &str) -> &str {
		self.dest_map.get(code).unwrap_or(&"")
	}

	fn get_jump(&self, code: &str) -> &str {
		self.jump_map.get(code).unwrap_or(&"")
	}
 
	pub fn parse(&self, instruction: String) -> BoxResult<String> {
		if instruction.contains(";") && instruction.contains("=") {
			bail!("Cannot contain both destination and jump.");
		}
		match C_REGEX.captures(&instruction) {
			Some(x) => {
				let mut comp = "";
				let mut a_comp = "0";
				let mut dest = "000";
				let mut jump = "000";
				let fixed_value: String = "111".to_owned();
				if let Some(c) = x.name("comp1") {
					let comp_result = self.get_comp(c.as_str());
					comp = comp_result.0;
					a_comp = comp_result.1;
				} else if let Some(c) = x.name("comp2") {
					let comp_result = self.get_comp(c.as_str());
					comp = comp_result.0;
					a_comp = comp_result.1;
				};
				if let Some(d) = x.name("dest") {
					dest = self.get_dest(d.as_str());
				};
				if let Some(j) = x.name("jump") {
					jump = self.get_jump(j.as_str());
				};
				let result_string = fixed_value + a_comp + comp + dest + jump;
				Ok(result_string)
			},
			None => bail!("Failed")
		}
	}
}


#[derive(Debug)]
pub struct SymbolTable {
	symbol_map: HashMap<String, u32>,
	ram_pointer: u32,
}

impl SymbolTable {
	pub fn new() -> Self {
		let symbol_map: HashMap<String, u32> = [
			 (String::from("SP")     , 0),
		     (String::from("LCL")    , 1),
		     (String::from("ARG")    , 2),
		     (String::from("THIS")   , 3),
		     (String::from("THAT")   , 4),
		     (String::from("R0")     , 0),
		     (String::from("R1")     , 1),
		     (String::from("R2")     , 2),
		     (String::from("R3")     , 3),
		     (String::from("R4")     , 4),
		     (String::from("R5")     , 5),
		     (String::from("R6")     , 6),
		     (String::from("R7")     , 7),
		     (String::from("R8")     , 8),
		     (String::from("R9")     , 9),
		     (String::from("R10")    , 10),
		     (String::from("R11")    , 11),
		     (String::from("R12")    , 12),
		     (String::from("R13")    , 13),
		     (String::from("R14")    , 14),
		     (String::from("R15")    , 15),
		     (String::from("SCREEN") , 16384),
		     (String::from("KBD")    , 24576)].iter().cloned().collect();
		let ram_pointer: u32 = 0;
		Self {
			symbol_map,
			ram_pointer,
		}
	}

	pub fn symbol_map(&self) -> &HashMap<String, u32> {
        &self.symbol_map
    }

	pub fn incr_ram_pointer(&mut self) -> u32 {
		let curr: u32 = self.ram_pointer;
		self.ram_pointer += 1;
		curr
	}

	pub fn add_entry(&mut self, key: String) {
		self.symbol_map.insert(key, self.ram_pointer);
	}

	pub fn get_address(&mut self, symbol: &str) -> BoxResult<String> {
		let parsed_address = symbol.parse::<u32>();
		let bytes = match parsed_address {
			Ok(_num) => to_binary(parsed_address.unwrap()),
			Err(_e) => {
				if !self.symbol_map.contains_key(symbol) {
					self.add_entry(symbol.to_string());
					self.incr_ram_pointer();
				}
				to_binary(*self.symbol_map.get(symbol).unwrap())
			},
		};
		Ok(bytes)
	}

	pub fn parse_symbol(symbol: String, command_type: Command) -> BoxResult<String> {
		let regex = match command_type {
			Command::ACommand => A_REGEX.captures(&symbol),
			Command::LCommand => L_REGEX.captures(&symbol),
			_ => bail!("This function only parses A and L commands."),
		};
		match regex {
			Some(x) => {
				let mut sym = String::from("");
				if let Some(s) = x.name("symbol") {
					sym = s.as_str().to_string();
				}
				Ok(sym)
			},
			None => bail!("Failed")
		}
	} 
}

pub struct Parser<'a> {
	cursor: Cursor<String>,
	length: u64,
	code_parser: Code<'a>,
	symbol_table: SymbolTable,
}

impl<'a> Parser<'a> {
	pub fn new(input: &str) -> Self {
		let content = input
				.lines()
				.filter(|line| !line.is_empty() && !line.starts_with("//"))
				.fold(String::new(), |acc, line| acc + line + "\n");
		let length = content.len() as u64;
		let code_parser = Code::new();
		let symbol_table = SymbolTable::new();
    	Self {
    		cursor: Cursor::new(content),
    		length: length,
    		code_parser,
    		symbol_table,
    	}
	}

	pub fn symbol_table(&self) -> &SymbolTable {
        &self.symbol_table
    }

	fn has_more_commands(&self) -> bool {
		self.cursor.position() < self.length
	}

	fn read_next(&mut self) -> String {
		let mut buffer = String::new();
		self.cursor.read_line(&mut buffer);
		buffer
	}

	pub fn fill_symbol_table(&mut self) -> Result<(), &str> {
		while self.has_more_commands() {
			let command = self.read_next();
			println!("{:?}", command);
			match command_type(&command) {
				Ok(command_type @ Command::LCommand) => {
					if let Ok(symbol) = SymbolTable::parse_symbol(command, command_type) {
						self.symbol_table.add_entry(symbol);
					}
				},
				Err(_e) => break,
				_ => {
					self.symbol_table.incr_ram_pointer();
				},
			};
		}
		if self.has_more_commands() {
			return Err("Failed to parse all commands.");
		}
		self.cursor.set_position(0);
		self.symbol_table.ram_pointer = 16; // starting at address 16 (just after the addresses allocated to the predefined symbols)
		Ok(())
	}

	pub fn to_bytes(&mut self) -> BoxResult<String> {
		self.fill_symbol_table()?;
		let mut bytes = String::from("");
		while self.has_more_commands() {
			let command = self.read_next();
			let parsed = match command_type(&command) {
				Ok(command_type @ Command::ACommand) => {
					let symbol = SymbolTable::parse_symbol(command, command_type).unwrap();
					self.symbol_table.get_address(&symbol)
				},
				Ok(Command::CCommand) => self.code_parser.parse(command),
				Ok(Command::LCommand) => continue,
				Err(e) => Err(e),
			};
			bytes.push_str(&parsed.unwrap());
			bytes.push_str("\n");
		}
		Ok(bytes)
	}
}

impl fmt::Display for Parser<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    	let string = self.cursor
    					.clone()
    					.into_inner();
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

pub fn command_type(command: &str) -> BoxResult<Command> {
	match command {
		command if A_REGEX.is_match(command) => Ok(Command::ACommand),
		command if C_REGEX.is_match(command) => Ok(Command::CCommand),
		command if L_REGEX.is_match(command) => Ok(Command::LCommand),
		_ => bail!("Invalid input.")
	}
}

pub fn to_binary(input: u32) -> String {
	format!("{:016b}", input)
}

