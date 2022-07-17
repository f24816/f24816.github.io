use pulldown_cmark::html;

pub fn breaking(input: Vec<&str>) {

    let mut output: Vec<String> = Vec::new();
    let mut br = false;

    for i in 0..input.len() {

        let x: String = input[i].to_string();

        if input[i].contains("<p"){
            if input[i].contains("/p>"){
                let xx: String = x.clone();
                //print!("{}{}\n", "added nothing: ".red(), &xx);
                output.push(xx);
                br = false;
            } else {
                // add <br>
                let mut xx: String = x.clone();
                xx.push_str("<br>");
                ////print!("{}{}\n", "added <br>: ".red(), &xx);
                output.push(xx);
                br = true;
            }
        } else {
            if input[i].contains("</p"){
                let xx: String = x.clone();
                ////print!("{}{}\n", "added nothing: ".red(), &xx);
                output.push(xx);
                br = false;
            }
            if br == true{
                let mut xx: String = x.clone();
                xx.push_str("<br>");
                ////print!("{}{}\n", "added <br>: ".red(), &xx);
                output.push(xx);
            } else {
                let xx: String = x.clone();
                output.push(xx);
            }
        }
    }

    for i in 0..output.len() {
        ////print!("{}\n", output[i]);
    }

    std::fs::write("write_test.html", output.concat()).expect("Unable to write file");

}