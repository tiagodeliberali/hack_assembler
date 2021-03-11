use std::collections::HashMap;

fn number_to_binary(number: &usize) -> String {
    format!("{:016b}", number)
}

struct SymbolMapper {
    locations: HashMap<String, usize>,
    current_position: usize,
}

impl SymbolMapper {
    pub fn new(locations: HashMap<String, usize>) -> SymbolMapper {
        let symbols = SymbolMapper::build_symbol_list(locations);

        SymbolMapper {
            locations: symbols,
            current_position: 16,
        }
    }

    pub fn process_symbol(&mut self, symbol: String) -> String {
        match self.locations.get(&symbol) {
            Some(value) => number_to_binary(value),
            None => self.create_variable(symbol),
        }
    }

    fn create_variable(&mut self, symbol: String) -> String {
        self.locations.insert(symbol.clone(), self.current_position);
        self.current_position += 1;
        self.process_symbol(symbol)
    }

    fn build_symbol_list(locations: HashMap<String, usize>) -> HashMap<String, usize> {
        let mut result = HashMap::from(locations);

        // register
        result.insert(String::from("R0"), 0);
        result.insert(String::from("R1"), 1);
        result.insert(String::from("R2"), 2);
        result.insert(String::from("R3"), 3);
        result.insert(String::from("R4"), 4);
        result.insert(String::from("R5"), 5);
        result.insert(String::from("R6"), 6);
        result.insert(String::from("R7"), 7);
        result.insert(String::from("R8"), 8);
        result.insert(String::from("R9"), 9);
        result.insert(String::from("R10"), 10);
        result.insert(String::from("R11"), 11);
        result.insert(String::from("R12"), 12);
        result.insert(String::from("R13"), 13);
        result.insert(String::from("R14"), 14);
        result.insert(String::from("R15"), 15);

        // devices
        result.insert(String::from("SCREEN"), 16_384);
        result.insert(String::from("KBD"), 24_576);

        // others
        result.insert(String::from("SP"), 0);
        result.insert(String::from("LCL"), 1);
        result.insert(String::from("ARG"), 2);
        result.insert(String::from("THIS"), 3);
        result.insert(String::from("THAT"), 4);

        result
    }
}

pub fn parse_content(lines: Vec<String>, locations: HashMap<String, usize>) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    let mut mapper = SymbolMapper::new(locations);

    for line in lines {
        if line.starts_with('@') {
            result.push(build_a_instruction(&line, &mut mapper))
        } else {
            result.push(buld_c_instruction(&line));
        }
    }

    // for debug
    println!("CONTENT PARSED:");
    for line in &result {
        println!("{}", line);
    }

    result
}

fn build_a_instruction(line: &String, mapper: &mut SymbolMapper) -> String {
    let line = line.replace("@", "");

    match line.parse::<usize>() {
        Ok(value) => number_to_binary(&value),
        Err(_) => mapper.process_symbol(line),
    }
}

fn buld_c_instruction(line: &String) -> String {
    // dest=comp;jump
    let mut dest = String::new();
    let mut comp = String::from(line);
    let mut jump = String::new();

    if line.contains('=') {
        let line_split: Vec<&str> = line.split('=').collect();
        dest = String::from(*line_split.get(0).unwrap());
        comp = String::from(*line_split.get(1).unwrap());
    }

    if comp.contains(';') {
        let clone_comp = comp.clone();
        let line_split: Vec<&str> = clone_comp.split(';').collect();
        jump = String::from(*line_split.get(1).unwrap());
        comp = String::from(*line_split.get(0).unwrap());
    }

    format!(
        "111{}{}{}",
        build_comp(comp),
        build_dest(dest),
        build_jump(jump)
    )
}

fn build_comp(value: String) -> String {
    let result = match value.as_str() {
        "0" => "0101010",
        "1" => "0111111",
        "-1" => "0111010",

        "D" => "0001100",
        "A" => "0110000",
        "M" => "1110000",

        "!D" => "0001101",
        "!A" => "0110001",
        "!M" => "1110001",

        "-D" => "0001111",
        "-A" => "0110011",
        "-M" => "1110011",

        "D+1" => "0011111",
        "A+1" => "0110111",
        "M+1" => "1110111",

        "D-1" => "0001110",
        "A-1" => "0110010",
        "M-1" => "1110010",

        "D+A" => "0000010",
        "D+M" => "1000010",

        "A+D" => "0000010",
        "M+D" => "1000010",

        "D-A" => "0010011",
        "D-M" => "1010011",

        "A-D" => "0000111",
        "M-D" => "1000111",

        "D&A" => "0000000",
        "D&M" => "1000000",

        "A&D" => "0000000",
        "M&D" => "1000000",

        "D|A" => "0010101",
        "D|M" => "1010101",

        "A|D" => "0010101",
        "M|D" => "1010101",
    
        _ => "ERRO!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!",
    };

    format!("{}", result)
}

fn build_dest(value: String) -> String {
    let result = match value.as_str() {
        "M" => "001",
        "D" => "010",
        "MD" => "011",
        "A" => "100",
        "AM" => "101",
        "AD" => "110",
        "AMD" => "111",
        _ => "000",
    };

    String::from(result)
}

fn build_jump(value: String) -> String {
    let result = match value.as_str() {
        "JGT" => "001",
        "JEQ" => "010",
        "JGE" => "011",
        "JLT" => "100",
        "JNE" => "101",
        "JLE" => "110",
        "JMP" => "111",
        _ => "000",
    };

    String::from(result)
}
