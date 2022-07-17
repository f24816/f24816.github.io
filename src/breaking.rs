use pulldown_cmark::html;
use colored::*;

pub fn breaking(input: Vec<&str>) {

    let mut output: Vec<String> = Vec::new();
    let mut br = false;

    for i in 0..input.len() {

        let x: String = input[i].to_string();
    /*

    if <p>
        and </p>
            null
        else
            brake
    else
        if </p>
            null
        if br = true
            brake
        else
            null

    */
        if input[i].contains("<p>"){
            if input[i].contains("/p>"){
                let xx: String = x.clone();
                print!("{}{}\n", "<p></p> was found: ".red(), &xx);
                output.push(xx);
                br = false;
            } else {
                // add <br>
                let mut xx: String = x.clone();
                xx.push_str("<br>");
                print!("{}{}\n", "<p> was found: ".red(), &xx);
                output.push(xx);
                br = true;
            }
        } else {
            if input[i].contains("/p>"){
                let xx: String = x.clone();
                print!("{}{}\n", "</p> was found: ".red(), &xx);
                output.push(xx);
                br = false;

            }
            else {
                let xx: String = x.clone();

                if br == true{
                    let mut xx: String = x.clone();
                    xx.push_str("<br>");
                    print!("{}{}\n", "brake was true: ".red(), &xx);
                }
                if br == false{
                    print!("{}{}\n", "continue: ".red(), &xx);
                    output.push(xx);
                }
            }
        }
    }


    // 4 debuging
    for i in 0..output.len() {
        ////print!("{}\n", output[i]);
    }

    std::fs::write("write_test.html", output.concat()).expect("Unable to write file");

}