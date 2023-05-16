
use std::collections::HashMap;
use crate::rail_assembler::rasm_dictionary::RasmDictionary;

use crate::rail_assembler::rasm_line::{LineType, RasmLine, RasmTag};

mod rasm_line;
pub mod rasm_dictionary;


const EMPTY: &[&str] = &[];
const EMPTY_VEC: Vec<String> = Vec::new();
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
        let mut is_v2 = false;


        for line in lines {
            line_number += 1;
            let (code, comment) = self.extract_comment(line);
            if code.is_empty() {
                if comment.contains("&rail-asm-v2") {
                    is_v2 = true;
                }
                result.push(RasmLine::new(comment.to_string(), RasmTag::None,
                                          LineType::Empty, EMPTY, EMPTY_VEC,
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
                                              LineType::Tag, &parts[1..2], EMPTY_VEC,
                                              line_number, line));
                }
                else if code.starts_with(CONST) {
                    let parts = Self::get_parts(&code);
                    if parts.len() < 3 {
                        println!("Incomplete Const");
                    }
                    result.push(RasmLine::new(comment.to_string(), RasmTag::Const,
                                              LineType::Tag, &parts[1..3], EMPTY_VEC,
                                              line_number, line));
                }
                else {
                    let parts = Self::get_parts(&code);
                    let (parts, add_parts) = if is_v2 {
                        Self::preprocess_parts(parts)
                    }
                    else {
                        Self::materialize_parts(parts)
                    };
                    if !parts.is_empty() {
                        result.push(RasmLine::new(comment.to_string(), RasmTag::None,
                                              LineType::Code, EMPTY, parts,
                                              line_number, line));
                    }
                    match add_parts {
                        Some(ext_parts) => {
                            for ext_part in ext_parts {
                                result.push(RasmLine::new(comment.to_string(), RasmTag::None,
                                                      LineType::Code, EMPTY, ext_part,
                                                      line_number, line));
                            }
                        },
                        None => { },
                    }
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

    fn materialize_parts(parts: Vec<&str>) -> (Vec<String>, Option<Vec<Vec<String>>>) {
        let mut res = Vec::new();
        for part in parts {
            res.push(String::from(part));
        }

        (res, None)
    }

    fn preprocess_parts(parts: Vec<&str>) -> (Vec<String>, Option<Vec<Vec<String>>>) {
        if parts.len() == 0 {
            return (Vec::new(), None);
        }

        let mut res = Vec::new();
        let mut op = String::from(parts[0]);

        match parts.get(1) {
            Some(p) => {
                let part = if p.starts_with('*') {
                    op.push_str("+IM1");
                    p.replacen('*', "", 1)
                }
                else {
                    p.to_string()
                };
                res.push(part);
            },
            None => { },
        }

        match parts.get(2) {
            Some(p) => {
                let part = if p.starts_with('*') {
                    op.push_str("+IM2");
                    p.replacen('*', "", 1)
                }
                else {
                    p.to_string()
                };
                res.push(part);
            },
            None => { },
        }

        for p in 3..parts.len() {
            res.push(parts[p].to_string());
        }

        res.insert(0, op);
        Self::preprocess_expand(res)
    }

    fn preprocess_expand(mut parts: Vec<String>) -> (Vec<String>, Option<Vec<Vec<String>>>) {
        let op = parts.get(0).unwrap();
        let mut opt = None;
            // TODO: should be a better way to do this, without doing it manually. But can wait.
        if op.contains("JMP") {
            if parts.len() == 2 {
                for _ in 0..2 {
                    parts.insert(1, String::from("0"));
                }
            }
        }
        else if op.contains("MOV") {
            if parts.len() == 3 {
                parts.insert(2, String::from("0"));
            }
        }
        else if op.contains("NOOP") {
            if parts.len() == 1 {
                for _ in 0..3 {
                    parts.push(String::from("0"));
                }
            }
        }
        else if op.contains("CALL") {
            if parts.len() == 2 {
                for _ in 0..2 {
                    parts.push(String::from("0"));
                }
            }
        }
        else if op.contains("RET") {
            if parts.len() == 1 {
                for _ in 0..3 {
                    parts.push(String::from("0"));
                }
            }
        }
        else if op.contains("S_POP") {
            if parts.len() == 2 {
                for _ in 0..2 {
                    parts.insert(1, String::from("0"));
                }
            }
        }
        else if op.contains("S_PUSH") {
            if parts.len() == 2 {
                for _ in 0..2 {
                    parts.push(String::from("0"));
                }
            }
        }
        else if op.contains("HALT") {
            if parts.len() == 1 {
                for _ in 0..3 {
                    parts.push(String::from("0"));
                }
            }
        }
        else if op.starts_with("!ST<") {
            let mut expansion = Vec::new();
            for i in 1..parts.len() {
                expansion.push(vec![String::from("S_PUSH+IM1"), parts[i].to_string(),
                                    String::from("0"), String::from("0")]);
            }
            parts.clear();
            opt = Some(expansion);
        }
        else if op.starts_with("!ST>") {
            let mut expansion = Vec::new();
            for i in (1..parts.len()).rev() {
                expansion.push(vec![String::from("S_POP"), String::from("0"),
                                    String::from("0"), parts[i].to_string()]);
            }
            parts.clear();
            opt = Some(expansion);
        }

        (parts, opt)
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
