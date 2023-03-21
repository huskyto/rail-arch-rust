
use std::collections::HashMap;
use crate::rail_assembler::rasm_dictionary::RasmDictionary;

use crate::rail_assembler::rasm_line::{LineType, RasmLine, RasmTag};

mod rasm_line;
mod rasm_dictionary;


const EMPTY: &'static[&str] = &[];
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
        let lines: Vec<&str> = text.split("\n").collect();
        let mut result: Vec<RasmLine> = Vec::new();

        for i in 0..lines.len() {
                // code is returned in uppercase
            let line = lines[i];
            let (code, comment) = self.extract_comment(line);

            if code.is_empty() {
                result.push(RasmLine::new(comment.to_string(), RasmTag::None,
                                          LineType::Empty, EMPTY, EMPTY));
            }
            else {
                    // check for tags
                if code.starts_with(LABEL) {
                    let parts = Self::get_parts(&code);
                    if parts.len() < 2 {
                        println!("Label has no value");
                    }
                    result.push(RasmLine::new(comment.to_string(), RasmTag::Label,
                                              LineType::Tag, &parts[1..2], EMPTY));
                }
                else if code.starts_with(CONST) {
                    let parts = Self::get_parts(&code);
                    if parts.len() < 3 {
                        println!("Incomplete Const");
                    }
                    result.push(RasmLine::new(comment.to_string(), RasmTag::Const,
                                              LineType::Tag, &parts[1..3], EMPTY));
                }
                else {
                    let parts = Self::get_parts(&code);
                    result.push(RasmLine::new(comment.to_string(), RasmTag::None,
                                              LineType::Code, EMPTY, &parts));
                }
            }
        }

        result
    }

    fn get_parts(code: &str) -> Vec<&str> {
        let parts: Vec<&str> = code.split(" ")
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
                            label_map.insert(&line.tags[0], code_line * 4);
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
                result.push(self.process_code(&code, &const_map, &label_map));
            }
        }

        result
    }

    fn extract_comment(&self, line: &str) -> (String, String) {
        let mut comment = String::new();
        let mut code = String::new();
        if line.contains("#") {   // has a comment
            let parts: Vec<&str> = line.split("#").collect();
            if parts.len() > 0
            {
                code.push_str(parts[0].trim());
            }
            if parts.len() > 1 {
                comment = parts[1].trim().parse().unwrap();
                // code.push_str(parts[1].trim());
            }
        } else {
            code.push_str(line.trim());
        }

        (code.to_uppercase(), comment)
    }

    fn process_code(&self, code: &str, const_map: &HashMap<&str, &str>, label_map: &HashMap<&str, u8>) -> u8 {
                // TODO add more arithmetic support
        let parts: Vec<&str> = code.split("+") .collect();
        let mut result = 0;
        for i in 0..parts.len() {
            let mut real_code: &str = parts[i];
            let mut num_code = 0;

            while const_map.contains_key(real_code) {
                real_code = const_map[real_code];
            }
            if label_map.contains_key(real_code) {
                num_code = label_map[real_code];
            }
            else {
                let trans: (bool, u8) = RasmDictionary::translate(real_code);
                num_code = if (trans.0) {
                    trans.1
                }
                else {
                    self.decode_num(real_code)
                };
            }

            result += num_code
        }

        return result
    }

    fn decode_num(&self, str: &str) -> u8 {
            // TODO transform according to prefix: 0x, 0o, 0b
        u8::from_str_radix(str, 16).unwrap()
    }

}
