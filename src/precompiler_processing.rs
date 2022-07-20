use colored::*;

pub fn main(input: String) -> String {

    let input_vector: Vec<&str> = input.split("\n").collect();
    let mut output: Vec<String> = Vec::new();

    for i in 0..input_vector.len() {

        let current_line: String = input_vector[i].to_string();

        if current_line.starts_with("::"){

            let num : usize = current_line.as_str().chars().nth(2).unwrap() as usize - 0x30;
            let prefix : String = format!("{}{}", "::", num);

            let x = format!("{:#<1$}", "", num);

            let converted_line: String = current_line.clone().replace(prefix.as_str(), &x);
            output.push(converted_line);
        } else {
            output.push(current_line);
        }
    }
    return output.join("\n");
}

