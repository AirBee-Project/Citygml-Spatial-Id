use std::fs::File;
pub mod parser;

pub struct Building {
    file: File,
}

impl Building {
    ///Building型を作成する
    pub fn new(file: File) -> Self {
        Building { file }
    }
}
