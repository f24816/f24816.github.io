use debug_print::debug_print;
use pulldown_cmark::html;

pub fn main(input: Vec<&str>) {

    let mut output: Vec<String> = Vec::new();
    let mut adding_break = false;

    for i in 0..input.len() {

        let x: String = input[i].to_string();

        if input[i].contains("<p>"){
            if input[i].contains("/p>"){
                let xx: String = x.clone();
                debug_print!("{}{}\n", "<p></p> was found: ", &xx);
                output.push(xx);
                adding_break = false;
            } else {
                // add <br>
                let mut xx: String = x.clone();
                xx.push_str("<br>");
                debug_print!("{}{}\n", "<p> was found: ", &xx);
                output.push(xx);
                adding_break = true;
            }
        } else {
            if input[i].contains("/p>"){
                let xx: String = x.clone();
                debug_print!("{}{}\n", "</p> was found: ", &xx);
                output.push(xx);
                adding_break = false;

            }
            else {
                let xx: String = x.clone();

                if adding_break == true{
                    let mut xx: String = x.clone();
                    xx.push_str("<br>");
                    debug_print!("{}{}\n", "brake was true: ", &xx);
                    output.push(xx);
                }
                if adding_break == false{
                    debug_print!("{}{}\n", "continue: ", &xx);
                    output.push(xx);
                }
            }
        }
    }

    std::fs::write("write_test.html", output.join("\n")).expect("Unable to write file");

}