
#![allow(unused_variables)]
#![allow(unused_imports)]

use debug_print::debug_print;
use colored::Colorize;
use std::fs::File;
use std::io::prelude::*;

fn main() {

    // open file
    let mut file = File::open("poem.txt")
        .expect("File not found")
    ;


    // creating the data variable
    let mut data = String::new().replace("\r\n", "<br>\n");

    // we read from "file" to the variable data.
    file.read_to_string(&mut data)
        .expect("Error while reading file")
    ;

    //debug_print!("{}",&data);
    data.to_string();

    let data_array: Vec<&str> = data.split("\n").collect();

    // printing all characters of the array
    for i in 0..data_array.len() {
        println!("{}", data_array[i]);
    }

    for line in data.lines() {
        print!("line nr?\n");
    }

    //println!("{}{}",format!("\nAfter Replace\n").red(), data.replace("\r\n", "<br>\n"));

    // do this with all the string form array
    // let P = format!("<p>{}</p>", data_array[i]);

}
