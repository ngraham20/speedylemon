use unicode_segmentation::UnicodeSegmentation;

pub enum BorderStyle {
    None,
    Solid,
    Bold,
    Ascii,
}

pub trait Border {
    fn border(&self, style: BorderStyle) -> Self;
}

impl Border for Vec<String> {
    fn border(&self, style: BorderStyle) -> Self {
        let border: Vec<&str> = match style {
            BorderStyle::Bold => "┏┓━┃┗┛",
            BorderStyle::Solid => "┌┐─│└┘",
            BorderStyle::Ascii => "++-|++",
            BorderStyle::None => "",
        }.graphemes(true).collect();
        let mut out: Vec<String> = Vec::new();
        let width = self.iter().map(|s| s.graphemes(true).count()).max().unwrap_or(0);

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
        let width = self.iter().map(|s| s.graphemes(true).count()).max().unwrap_or(0);
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
    // fn popup(&self, lines: &Vec<String>, x: usize, y: usize) -> Self {
    //     let mut result = self.clone();
    //     for idx in 0..lines.len() {
    //         let line = &lines[idx];
    //         let length = line.chars().count();
    //         result[idx+y].replace_range(x..length+x, &line);
    //     }
    //     result
    // }
    fn popup(&self, lines: &Vec<String>, x: usize, y: usize) -> Self {
        // start with the unaffected lines before it
        let mut out: Vec<String> = self[0..y].to_vec();
        let width = self.iter().map(|s| s.graphemes(true).count()).max().unwrap_or(0);

        println!("width: {}", width);
        println!("x: {}", x);
        for idx in y..y+lines.len() {
            let line = &lines[idx-y];
            let length = line.graphemes(true).count();
            // draw inside the original content
            if idx < self.len() {
                if x+length < self[idx].graphemes(true).count() {
                    println!("line length: {}", length);
                    out.push(format!("{}{}{}", &self[idx].graphemes(true).collect::<Vec<_>>()[..x].join(""), &line, &self[idx].graphemes(true).collect::<Vec<_>>()[x+length..].join("")));
                } else {
                    out.push(format!("{}{}", &self[idx].graphemes(true).collect::<Vec<_>>()[..x].join(""), &line));
                }
                
            }
            // we're drawing past the content (in height)
            else { 
                out.push(format!("{}{}{}", " ".repeat(x), line.clone(), " ".repeat(width.saturating_sub(x).saturating_sub(length))));
            }
        }

        if y + lines.len() < self.len() {
            out.append(&mut self[y+lines.len()..].to_vec())
        }
        out
    }
}

pub trait Render {
    fn render(&self) -> String;
}

impl Render for Vec<String> {
    fn render(&self) -> String {
        self.join("\n")
    }
}