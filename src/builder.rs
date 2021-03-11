use std::collections::HashMap;

pub fn build_content(content: String) -> (Vec<String>, HashMap<String, usize>) {
    let mut locations: HashMap<String, usize> = HashMap::new();
    let mut code_lines: Vec<String> = Vec::new();

    for line in content.lines() {
        let line = clean_line(line);

        if line.len() == 0 {
            continue;
        }

        if let Some(symbol) = get_symbol(&line) {
            locations.insert(symbol, code_lines.len());
        } else {
            code_lines.push(String::from(line));
        }
    }

    // for debug
    println!("CONTENT BUILT:");
    for (k, v) in &locations {
        println!("symbol: {} at {} line", k, v);
    }
    for line in &code_lines {
        println!("{}", line);
    }

    (code_lines, locations)
}

fn clean_line(line: &str) -> String {
    let line: Vec<&str> = line.split('/').collect();
    let line = line[0];
    let line = line.trim();
    line.replace(" ", "")
}

fn get_symbol(line: &str) -> Option<String> {
    if line.starts_with('(') {
        return Some(line.replace("(", "").replace(")", ""));
    }
    return None;
}
