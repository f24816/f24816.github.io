
#![allow(unused_variables)]
#![allow(unused_imports)]

use debug_print::debug_print;
use colored::Colorize;
use std::fs::File;
use std::io::prelude::*;
use pulldown_cmark::{html, Parser};

fn main() {

    // open file
    let mut file = File::open("example.txt")
        .expect("File not found")
    ;

    // creating the data variable
    let mut data = String::new();

    // we read from "file" to the variable data.
    file.read_to_string(&mut data)
        .expect("Error while reading file")
    ;

    // crlf conversion
    let data_lf = data.replace("\r\n", "<br>\n");
    // split all the lines and put them inside a vector
    let data_srt_vector: Vec<&str> = data_lf.split("\n").collect();

    // debug - printing all lines
    for i in 0..data_srt_vector.len() {
        println!("{}", data_srt_vector[i]);
    }

    // debuging
    println!("Number of lines: {}", data_srt_vector.len());
    println!("Number of \"!\" : {}", data_srt_vector.concat().matches("!").count());


    // testing

    let parser = Parser::new(data_lf.as_str());
    let mut html_buf = String::new();
    html::push_html(&mut html_buf, parser);

    std::fs::write("write_test.txt", data_lf).expect("Unable to write file");

    print!("{}", &html_buf);

}
