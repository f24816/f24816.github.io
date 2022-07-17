
#![allow(unused_variables)]
#![allow(unused_imports)]

use debug_print::debug_print;
use colored::*;
use std::fs::File;
use std::io::prelude::*;
use pulldown_cmark::{html, Parser};

mod breaking;

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




    // litle bird is here to wish a good day
    print!("-----🐦-----\n");

    let parser = Parser::new(data_lf.as_str());
    let mut html_buf = String::new();
    html::push_html(&mut html_buf, parser);

    // write to file
    std::fs::write("write_test.html", &html_buf).expect("Unable to write file");

    let html_buf_vector: Vec<&str> = html_buf.split("\n").collect();

    // breaking mod
    breaking::breaking(html_buf_vector);


    // debuging
    println!("{}{}", "\nNumber of lines: ".green(), data_srt_vector.len());
    println!("{}{}", "Number of \"!\" : ".green(), data_srt_vector.concat().matches("!").count());
    println!("{}{}", "Does it contain a list: ".green(), html_buf.contains("<li>"));

}
