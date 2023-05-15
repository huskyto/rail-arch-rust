
use std::collections::HashMap;
use crate::rail_assembler::rasm_dictionary::RasmDictionary;

use crate::rail_assembler::rasm_line::{LineType, RasmLine, RasmTag};

mod rasm_line;
pub mod rasm_dictionary;


const EMPTY: &[&str] = &[];
const LABEL: &str = "LABEL";
const CONST: &str = "CONST";

pub trait RailAssemblerTrait {
    fn assemble(&self, code: &str) -> Vec<u8>;
}

pub struct RailAssembler { }

impl RailAssemblerTrait for RailAssembler {
    fn assemble(&self, code: &str) -> Vec<u8> {
        let lines = self.parse_lines(code);
        self.process_lines(&lines)
    }
}

impl RailAssembler {

    pub fn new() -> Self {
        Self {}
    }

    fn parse_lines(&self, text: &str) -> Vec<RasmLine> {
        let lines: Vec<&str> = text.split('\n').collect();
        let mut result: Vec<RasmLine> = Vec::new();
        let mut line_number: u32 = 0;


        for line in lines {
            line_number += 1;
            let (code, comment) = self.extract_comment(line);
            if code.is_empty() {
                result.push(RasmLine::new(comment.to_string(), RasmTag::None,
                                          LineType::Empty, EMPTY, EMPTY,
                                          line_number, line));
            }
            else {
                    // check for tags
                if code.starts_with(LABEL) {
                    let parts = Self::get_parts(&code);
                    if parts.len() < 2 {
                        println!("Label has no value");
                    }
                    result.push(RasmLine::new(comment.to_string(), RasmTag::Label,
                                              LineType::Tag, &parts[1..2], EMPTY,
                                              line_number, line));
                }
                else if code.starts_with(CONST) {
                    let parts = Self::get_parts(&code);
                    if parts.len() < 3 {
                        println!("Incomplete Const");
                    }
                    result.push(RasmLine::new(comment.to_string(), RasmTag::Const,
                                              LineType::Tag, &parts[1..3], EMPTY,
                                              line_number, line));
                }
                else {
                    let parts = Self::get_parts(&code);
                    result.push(RasmLine::new(comment.to_string(), RasmTag::None,
                                              LineType::Code, EMPTY, &parts,
                                              line_number, line));
                }
            }
        }

        result
    }

    fn get_parts(code: &str) -> Vec<&str> {
        let parts: Vec<&str> = code.split(' ')
            .map(|cd| cd.trim())
            .filter(|cd| !cd.is_empty())
            .collect();
        parts
    }

    fn process_lines(&self, lines: &Vec<RasmLine>) -> Vec<u8> {
        let mut const_map: HashMap<&str, &str> = HashMap::new();
        let mut label_map: HashMap<&str, u8> = HashMap::new();
        let mut result: Vec<u8> = Vec::new();
        let mut code_lines: Vec<&RasmLine> = Vec::new();
        let mut code_line = 0;

        for line in lines {
            match line.line_type {
                LineType::Empty => { }  // noop
                LineType::Tag => {
                    match line.tag_type {
                        RasmTag::Const => {
                            const_map.insert(&line.tags[0], &line.tags[1]);
                        }
                        RasmTag::Label => {
                            if label_map.insert(&line.tags[0], code_line * 4).is_some() {
                                Self::do_panic(line, &format!("Label {} already exists", line.tags[0]));
                            }
                        }
                        RasmTag::None => {} // noop
                    }
                }
                LineType::Code => {
                    code_lines.push(line);
                    code_line += 1;
                }
            }
        }
        for line in code_lines {
            for code in &line.code_parts {
                match self.process_code(code, &const_map, &label_map) {
                    Ok(res) => result.push(res),
                    Err(e) => Self::do_panic(line, &e.to_string()),
                }
            }
        }

        result
    }

    fn do_panic(line: &RasmLine, error_msg: &str) {
        panic!("{}", format!("\n\x00\x1B[33m Fatal error on line {}: {}\n Cause: {} \x00\x1B[0m\n",
                             line.line_number, line.original_line, error_msg));
    }

    fn extract_comment(&self, line: &str) -> (String, String) {
        let mut comment = String::new();
        let mut code = String::new();
        if line.contains('#') {   // has a comment
            let parts: Vec<&str> = line.split('#').collect();
            if !parts.is_empty()
            {
                code.push_str(parts[0].trim());
            }
            if parts.len() > 1 {
                comment = parts[1].trim().parse().unwrap();
            }
        } else {
            code.push_str(line.trim());
        }

        (code.to_uppercase(), comment)
    }

    fn process_code(&self, code: &str, const_map: &HashMap<&str, &str>, label_map: &HashMap<&str, u8>) -> Result<u8, String> {
                // TODO add more arithmetic support
        let parts: Vec<&str> = code.split('+') .collect();
        let mut result = 0;
        for part in parts {
            let mut real_code: &str = part;
            while const_map.contains_key(real_code) {
                real_code = const_map[real_code];
            }
            let num_code: u8 = if label_map.contains_key(real_code) {
                label_map[real_code]
            }
            else {
                match RasmDictionary::translate(real_code) {
                    Ok(code) => code,
                    Err(_) => {
                        match self.decode_num(real_code) {
                            Ok(num) => num,
                            Err(e) => {
                                return Err(format!("Parsing error: {}", e));
                            }
                        }
                    }
                }
            };

            result += num_code
        }

        Ok(result)
    }

    fn decode_num(&self, str: &str) -> Result<u8, String> {
        match
            if let Some(value) = str.strip_prefix("0X") {
                u8::from_str_radix(value, 16)
            }
            else if let Some(value) = str.strip_prefix("0O") {
                u8::from_str_radix(value, 8)
            }
            else if let Some(value) = str.strip_prefix("0B") {
                u8::from_str_radix(value, 2)
            }
            else {
                str.parse::<u8>()  // base10
            } {
                Ok(res) => Ok(res),
                Err(_) => Err(format!("Error parsing {}; not a valid value.", str))
        }
    }

}
