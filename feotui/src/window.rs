use crossterm::style;

pub enum BorderStyle {
    None,
    Solid,
    Bold,
}

pub struct Window {
    pub lines: Vec<String>,
    border: BorderStyle,
    padding: usize,
}

impl Window {
    pub fn new() -> Self {
        Window {
            lines: Vec::new(),
            border: BorderStyle::None,
            padding: 1,
        }
    }
    pub fn build_lines(&self) -> Vec<String> {
        let mut res: Vec<String> = Vec::new();
        let width = self.lines.iter().map(|s| s.chars().count()).max().unwrap_or(0);
        println!("width: {}", width);
        println!("padding: {}", self.padding);
        match self.border {
            BorderStyle::Bold => {
                res.push(format!(" ┏{}┓ ", "━".repeat(width+(self.padding*2))));
                for line in &self.lines {
                    res.push(format!(" {: <padding$}{: <width$}{: >padding$} ", "┃", &line, "┃", padding = self.padding+1, width=width));
                }
                res.push(format!(" ┗{}┛ ", "━".repeat(width+(self.padding*2))));
            },
            _ => {
                res = self.lines.clone()
            },
        }
        res
    }
    pub fn with_lines(mut self, lines: Vec<String>) -> Self {
        self.lines = lines;
        self
    }
    pub fn with_border(mut self, style: BorderStyle) -> Self {
        self.border = style;
        self
    }
    pub fn with_padding(mut self, padding: usize) -> Self {
        self.padding = padding;
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