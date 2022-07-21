use std::io::Split;

use colored::*;

pub fn main(input: &String) -> String {

    let input_vector: Vec<&str> = input.split("\n").collect();
    let mut output: Vec<String> = Vec::new();

    for i in 0..input_vector.len() {

        let current_line: String = input_vector[i].to_string();

        if current_line.starts_with("::"){

            let n = &current_line.replace("::", "");
            let vec: Vec<&str> = n.split(" ").collect();
            let suffix = vec[0];
            let argument = vec[1];

            dbg!(&suffix);
            dbg!(&argument);

            match argument {
                "4" => println!("Heading!"),
                "1" => print!("Heading!"),
                _ => println!("Nothing!"),
            }

            //output.push(converted_line);
        } else {
            output.push(current_line);
        }
    }
    return output.join("\n");
}

