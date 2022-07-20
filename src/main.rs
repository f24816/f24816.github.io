#![allow(unused_variables)]
#![allow(unused_imports)]

use pulldown_cmark::{html, Parser};
use debug_print::debug_print;
use std::io::prelude::*;
use std::fs::File;
use colored::*; // felt cute, might delete later

mod postcompiler_processing;
mod precompiler_processing;

fn main() {

    // opening file
    let mut file = File::open("dummy.md")
        .expect("File not found")
    ;

    // creating the data variable
    let mut data = String::new();

    // we read from "file" into the variable data.
    file.read_to_string(&mut data)
        .expect("Error while reading file")
    ;

    // crlf converted string
    let data_lf_converted = data.replace("\r\n", "\n");

    // processing before HTML conversion
    let data_processed: String = precompiler_processing::main(data_lf_converted);

    // parsing markdown file using `pulldown_cmark`
    let mut data_html_converted = String::new();
    let parser = Parser::new(data_processed.as_str());
    html::push_html(&mut data_html_converted, parser);

    debug_print!("{}{}", "converted HTML:".red() ,data_html_converted);

    // processing after HTML conversion
    let data_html_converted_vector: Vec<&str> = data_html_converted.split("\n").collect();
    postcompiler_processing::main(data_html_converted_vector);

}
