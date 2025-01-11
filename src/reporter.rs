use std::collections::HashMap;
use std::path::PathBuf;

pub struct Reporter {
    generated_files: HashMap<PathBuf, usize>,
}

impl Reporter {
    pub fn new() -> Self {
        Self {
            generated_files: HashMap::new(),
        }
    }

    pub fn record_file(&mut self, path: PathBuf, line_count: usize) {
        self.generated_files.insert(path, line_count);
    }

    pub fn print_report(&self) -> String {
        let mut report = String::new();
        report.push_str("Generated Files Report:\n");
        report.push_str("======================\n");
        
        for (path, lines) in &self.generated_files {
            report.push_str(&format!("- {} ({} lines)\n", path.display(), lines));
        }
        
        report.push_str(&format!("\nTotal files generated: {}", self.generated_files.len()));
        report
    }
}
