use debug_print::debug_print;
use pulldown_cmark::html;

pub fn main(input: &Vec<&str>) {

    let mut output: Vec<String> = Vec::new();
    let mut adding_break = false;

    for i in 0..input.len() {

        let current_line: String = input[i].to_string();

        // adding breaks
        if input[i].contains("<p>"){
            if input[i].contains("/p>"){
                let converted_line: String = current_line.clone();
                debug_print!("{}{}\n", "<p></p> was found: ", &converted_line);
                output.push(converted_line);
                adding_break = false;
            } else {
                let mut converted_line: String = current_line.clone();
                converted_line.push_str("<br>");
                debug_print!("{}{}\n", "<p> was found: ", &converted_line);
                output.push(converted_line);
                adding_break = true;
            }
        } else {
            if input[i].contains("/p>"){
                let converted_line: String = current_line.clone();
                debug_print!("{}{}\n", "</p> was found: ", &converted_line);
                output.push(converted_line);
                adding_break = false;

            }
            else {
                let converted_line: String = current_line.clone();

                if adding_break == true{
                    let mut converted_line: String = current_line.clone();
                    converted_line.push_str("<br>");
                    debug_print!("{}{}\n", "brake was true: ", &converted_line);
                    output.push(converted_line);
                }
                if adding_break == false{
                    debug_print!("{}{}\n", "continue: ", &converted_line);
                    output.push(converted_line);
                }
            }
        }
    }

    std::fs::write("write_test.html", output.join("\n")).expect("Unable to write file");

}