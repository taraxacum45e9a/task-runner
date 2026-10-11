use std::fs::{self, File};
use std::io::{self, Read, Seek, SeekFrom};
use std::path::Path;

/// The whole file, or its last `limit` bytes starting at a line boundary.
/// Invalid UTF-8 is replaced.
pub fn tail(file: &Path, limit: u64) -> io::Result<String> {
    let size = fs::metadata(file)?.len();
    if size <= limit {
        return Ok(String::from_utf8_lossy(&fs::read(file)?).into_owned());
    }
    let mut f = File::open(file)?;
    f.seek(SeekFrom::Start(size - limit))?;
    let mut buf = Vec::new();
    f.read_to_end(&mut buf)?;
    let text = String::from_utf8_lossy(&buf);
    let start = text.find('\n').map_or(0, |i| i + 1);
    Ok(text[start..].to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tmp(name: &str, content: &str) -> std::path::PathBuf {
        let p = std::env::temp_dir().join(format!("task-runner-{}-{name}", std::process::id()));
        fs::write(&p, content).unwrap();
        p
    }

    #[test]
    fn small_file_is_returned_whole() {
        let p = tmp("small", "a\nb\n");
        assert_eq!(tail(&p, 100).unwrap(), "a\nb\n");
    }

    #[test]
    fn large_file_is_cut_at_line_boundary() {
        let p = tmp("large", "aaaa\nbbbb\ncccc\n");
        // last 7 bytes = "bb\ncccc\n"[..] -> starts mid-line, drops partial line
        assert_eq!(tail(&p, 7).unwrap(), "cccc\n");
    }

    #[test]
    fn large_file_without_newline_is_kept() {
        let p = tmp("nonl", "abcdefghij");
        assert_eq!(tail(&p, 4).unwrap(), "ghij");
    }
}
