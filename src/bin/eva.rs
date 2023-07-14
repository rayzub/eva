use std::{fs, f32::consts::E, path::Path};
use clap::{Arg, Command};
use eva::new_dissasembler;

extern crate eva;


fn main() {
    let matches = Command::new("Eva (EVM Dissassembler)")
                           .version("0.1.0")
                           .author("Rayhan Zuberi (rayzub)")
                           .arg(Arg::new("file")
                                .short('f')
                                .help("Specify the file containing binary to disassemble")
                                .required(true)
                            )
                            .get_matches();

    let file_path = matches.get_one::<String>("file").expect("File path is required!");
    let absolute_file_path =  match std::env::current_dir() {
        Ok(mut path) => {
            path.push(file_path);
            path
        },
        Err(err) => panic!("{}", err),
    };

    let bin_content = std::fs::read_to_string(absolute_file_path).unwrap();
    let decoded_bytecode = hex::decode(bin_content).unwrap();
    let analyzer = new_dissasembler(decoded_bytecode);

    
}