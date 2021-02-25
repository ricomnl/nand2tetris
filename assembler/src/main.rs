use std::env;
use std::process;

use assembler::Parser;

fn main() {
	let args: Vec<String> = env::args().collect();

	let filename = assembler::parse_input(&args).unwrap_or_else(|err| {
		println!("Problem parsing arguments: {}", err);
		process::exit(1);
	});
	let out_filename = filename.replace(".asm", ".hack");

    let content = assembler::read_file(filename).unwrap_or_else(|err| {
    	println!("Application error: {}", err);

    	process::exit(1);
    });

    let mut parser = Parser::new(&content);
    let byte_code = parser.to_bytes().unwrap_or_else(|err| {
    	println!("Couldn't parse to byte code: {}", err);

    	process::exit(1);
    });

    // println!("{}", byte_code);
    if let Err(err) = assembler::write_file(&out_filename, &byte_code) {
    	println!("Failed writing file: {}", err);

    	process::exit(1);
    }
}

