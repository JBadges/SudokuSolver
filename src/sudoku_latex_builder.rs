use std::fs::File;
use std::io::{Write, Result};
use std::path::Path;
use std::process::Command;

pub struct SudokuLatexBuilder {
    content: String,
}

impl SudokuLatexBuilder {
    pub fn new() -> Self {
        let mut builder = SudokuLatexBuilder {
            content: String::new(),
        };
        builder.init();
        builder
    }

    fn init(&mut self) {
        self.content.push_str("\\documentclass{standalone}\n");
        self.content.push_str("\\usepackage{tikz}\n");
        self.content.push_str("\\begin{document}\n");
        self.content.push_str("\\begin{tikzpicture}[scale=.5]\n");
    }

    pub fn add_grid(&mut self) {
        for x in 0..=9 {
            self.content.push_str(&format!("\\draw[very thick] ({},0) -- ({},9);\n", x, x));
            self.content.push_str(&format!("\\draw[very thick] (0,{}) -- (9,{});\n", x, x));
        }
    }

    pub fn color_candidate(&mut self, row: usize, col: usize, digit: usize, color: &str) {
        let x = col as f32 + ((digit - 1) % 3) as f32 * 0.33 + 0.11;
        let y = 8.5 - row as f32 - ((digit - 1) / 3) as f32 * 0.33 - 0.11;
        self.content.push_str(&format!("\\node[font=\\tiny, text={}] at ({},{}) {{{}}};\n", color, x, y, digit));
    }

    pub fn add_chain(&mut self, start: (usize, usize, usize), end: (usize, usize, usize), color: &str) {
        let start_x = start.1 as f32 + ((start.2 - 1) as f32 % 3.0) * 0.33 + 0.11;
        let start_y = 8.5 - start.0 as f32 - ((start.2 - 1) as f32 / 3.0) * 0.33 - 0.11;
        let end_x = end.1 as f32 + ((end.2 - 1) as f32 % 3.0) * 0.33 + 0.11;
        let end_y = 8.5 - end.0 as f32 - ((end.2 - 1) as f32 / 3.0) * 0.33 - 0.11;
        self.content.push_str(&format!("\\draw[->, {}, thick] ({},{}) -- ({},{});\n", color, start_x, start_y, end_x, end_y));
    }

    pub fn build(&mut self, path: &str) -> Result<()> {
        self.content.push_str("\\end{tikzpicture}\n");
        self.content.push_str("\\end{document}\n");
    
        let path_obj = Path::new(path);
        let directory = path_obj.parent().unwrap_or_else(|| Path::new("."));
        let filename = path_obj.file_name().unwrap();
    
        let mut file = File::create(path)?;
        file.write_all(self.content.as_bytes())?;
    
        let output = Command::new("pdflatex")
            .arg(filename)
            .current_dir(directory)  // Set the working directory
            .output()
            .expect("pdflatex is required to run this code in visualizer mode.");
    
        println!("Status: {}", output.status);
        println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
        println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    
        Ok(())
    }
}