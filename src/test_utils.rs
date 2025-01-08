use std::fs;
use std::path::Path;
use std::io;

/// Test utilities for cleaning up generated files
pub struct TestUtils;

impl TestUtils {
    /// Clean up generated files in the gen/ directory
    pub fn cleanup_generated_files() -> io::Result<()> {
        let gen_path = Path::new("gen");
        
        if gen_path.exists() {
            fs::remove_dir_all(gen_path)?;
        }
        
        Ok(())
    }

    /// Clean up specific temporary files
    pub fn cleanup_temp_files(file_paths: &[&str]) -> io::Result<()> {
        for path in file_paths {
            let path = Path::new(path);
            if path.exists() {
                fs::remove_file(path)?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;

    #[test]
    fn test_cleanup_generated_files() -> io::Result<()> {
        // Create test directory
        fs::create_dir_all("gen/test")?;
        File::create("gen/test_file.txt")?;

        // Clean up
        TestUtils::cleanup_generated_files()?;

        // Verify cleanup
        assert!(!Path::new("gen").exists());
        Ok(())
    }

    #[test]
    fn test_cleanup_temp_files() -> io::Result<()> {
        // Create test files
        File::create("temp1.txt")?;
        File::create("temp2.txt")?;

        // Clean up
        TestUtils::cleanup_temp_files(&["temp1.txt", "temp2.txt"])?;

        // Verify cleanup
        assert!(!Path::new("temp1.txt").exists());
        assert!(!Path::new("temp2.txt").exists());
        Ok(())
    }
}
