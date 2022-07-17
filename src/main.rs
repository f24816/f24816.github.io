
#![allow(unused_variables)]
#![allow(unused_imports)]


use pulldown_cmark::{html, Parser};
use debug_print::debug_print;
use std::io::prelude::*;
use std::fs::File;
use colored::*;

mod breaking;

fn main() {

    // open file
    let mut file = File::open("dummy.md")
        .expect("File not found")
    ;

    // creating the data variable
    let mut data = String::new();

    // we read from "file" to the variable data.
    file.read_to_string(&mut data)
        .expect("Error while reading file")
    ;

    // crlf conversion
    let data_lf = data.replace("\r\n", "\n");
    // split all the lines and put them inside a vector
    let data_srt_vector: Vec<&str> = data_lf.split("\n").collect();

    // litle bird is here to wish a good day
    print!("-----🐦-----\n");

    let parser = Parser::new(data_lf.as_str());
    let mut html_buf = String::new();
    html::push_html(&mut html_buf, parser);

    //debug_print!("{}", data_lf);
    debug_print!("{}{}", "converted HTML:".red() ,html_buf);

    // write to file
    std::fs::write("write_test.html", &html_buf).expect("Unable to write file");

    let html_buf_vector: Vec<&str> = html_buf.split("\n").collect();

    // for i in 0..html_buf_vector.len() {
    //     println!("{}", html_buf_vector[i]);
    // }

    // breaking mod
    breaking::breaking(html_buf_vector);

    // debuging
    println!("{}{}", "\nNumber of lines: ".green(), data_srt_vector.len());
    println!("{}{}", "Number of \"!\" : ".green(), data_srt_vector.concat().matches("!").count());
    println!("{}{}", "Does it contain a list: ".green(), html_buf.contains("<li>"));

}
