use unicode_segmentation::UnicodeSegmentation;

pub enum BorderStyle {
    None,
    Solid,
    Bold,
}

pub trait Border {
    fn border(&self, style: BorderStyle) -> Self;
}

impl Border for Vec<String> {
    fn border(&self, style: BorderStyle) -> Self {
        let border: Vec<&str> = match style {
            BorderStyle::Bold => "┏┓━┃┗┛",
            BorderStyle::Solid => "┌┐─│└┘",
            BorderStyle::None => "",
        }.graphemes(true).collect();
        let mut out: Vec<String> = Vec::new();
        let width = self.iter().map(|s| s.chars().count()).max().unwrap_or(0);

        out.push(format!(" {}{}{} ", border[0], border[2].repeat(width), border[1]));
        for line in self {
            out.push(format!(" {}{}{} ", border[3], line, border[3]));
        }
        out.push(format!(" {}{}{} ", border[4], border[2].repeat(width), border[5]));
        out
    }
}

pub trait Padding {
    fn pad(&self, padding: usize) -> Self;
}

impl Padding for Vec<String> {
    fn pad(&self, padding: usize) -> Self {
        let width = self.iter().map(|s| s.chars().count()).max().unwrap_or(0);
        let mut out: Vec<String> = Vec::new();
        for line in self {
            out.push(format!("{}{: <width$}{}", " ".repeat(padding), &line, " ".repeat(padding), width=width));
        }
        out
    }
}

pub trait Popup {
    fn popup(&self, lines: &Vec<String>, x: usize, y: usize) -> Self;
}

impl Popup for Vec<String> {
    fn popup(&self, lines: &Vec<String>, x: usize, y: usize) -> Self {
        let mut result = self.clone();
        for idx in 0..lines.len() {
            let line = &lines[idx];
            let length = line.chars().count();
            result[idx+y].replace_range(x..length+x, &line);
        }
        result
    }
}