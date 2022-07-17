/*
    for line in data.lines() {
        print!("line nr?\n");
    }
*/

    //println!("{}{}",format!("\nAfter Replace\n").red(), data.replace("\r\n", "<br>\n"));

    // do this with all the string form array
    // let P = format!("<p>{}</p>", data_array[i]);


pub fn html_comp(input: String) -> String {
    let mut output = String::new();
    for c in input.chars() {
        match c {
            '<' => output.push_str("&lt;"),
            '>' => output.push_str("&gt;"),
            '&' => output.push_str("&amp;"),
            _ => output.push(c)
        }
    }
    output
}
