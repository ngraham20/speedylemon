pub enum BorderStyle {
    None,
    Solid,
    Bold,
}

pub struct Window {
    width: usize,
    height: usize,
    lines: Vec<String>,
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