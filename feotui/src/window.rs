pub enum BorderStyle {
    None,
    Solid,
    Bold,
}

pub struct Window {
    width: usize,
    height: usize,
    pub lines: Vec<String>,
}

impl Window {
    pub fn new() -> Self {
        Window {
            width: 0,
            height: 0,
            lines: Vec::new(),
        }
    }
    pub fn build_string(&self) -> String {
        self.lines.join("\n")
    }
    pub fn with_width(mut self, width: usize) -> Self {
        self.width = width;
        self
    }
    pub fn with_height(mut self, height: usize) -> Self {
        self.height = height;
        self
    }
    pub fn with_lines(mut self, lines: Vec<String>) -> Self {
        self.lines = lines;
        self
    }
}

pub trait Blit {
    fn blit(&self);
}

pub trait Popup {
    fn popup(&self, lines: &Vec<String>, x: usize, y: usize) -> String;
}

impl Popup for Vec<String> {
    fn popup(&self, lines: &Vec<String>, x: usize, y: usize) -> String {
        let mut result = self.clone();
        for idx in 0..lines.len() {
            let line = &lines[idx];
            let length = line.chars().count();
            result[idx+y].replace_range(x..length+x, &line);
        }
        result.join("\n")
    }
}