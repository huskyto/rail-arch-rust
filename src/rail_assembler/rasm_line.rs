
pub struct RasmLine {
    pub comment: String,
    pub tag_type: RasmTag,
    pub line_type: LineType,
    pub tags: Vec<String>,
    pub code_parts: Vec<String>
}

impl RasmLine {
    pub fn new(comment: String, tag_type: RasmTag, line_type: LineType,
               tags: &[&str], code_parts: &[&str]) -> Self {
        Self {
            comment,
            tag_type,
            line_type,
            tags: tags.iter().map(|s| s.to_string()).collect(),
            code_parts: code_parts.iter().map(|s| s.to_string()).collect(),
        }
    }
}

pub enum RasmTag {
    Const, Label, None
}

pub enum LineType {
    Empty, Tag, Code
}
