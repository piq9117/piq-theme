use std::fs;
use std::process;

use clap::Parser;

const DEV_THEME_PATH: &str = "/home/piq9117/dotfiles/dev_theme";

#[derive(Parser)]
#[command(name = "piq-theme", about = "Set the terminal/nvim theme mode (light/dark)")]
struct Cli {
    #[arg(value_parser = clap::builder::PossibleValuesParser::new(["light", "dark"]))]
    mode: String,
}

fn main() {
    let cli = Cli::parse();

    let content = match fs::read_to_string(DEV_THEME_PATH) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error: could not read {}: {e}", DEV_THEME_PATH);
            process::exit(1);
        }
    };

    let trimmed = content.trim();
    let parts: Vec<&str> = trimmed.split(',').collect();

    if parts.len() != 2 || parts[0].is_empty() {
        eprintln!("Error: dev_theme file has invalid format (expected '<theme>,<light|dark>')");
        process::exit(1);
    }

    let theme_name = parts[0];
    let new_content = format!("{},{}\n", theme_name, cli.mode);

    if let Err(e) = fs::write(DEV_THEME_PATH, &new_content) {
        eprintln!("Error: could not write {}: {e}", DEV_THEME_PATH);
        process::exit(1);
    }

    println!("Theme set to {}", new_content.trim());
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;

    fn run_tool(path: &Path, mode: &str) -> (String, Option<i32>) {
        let content = fs::read_to_string(path).unwrap();
        let trimmed = content.trim();
        let parts: Vec<&str> = trimmed.split(',').collect();
        if parts.len() != 2 || parts[0].is_empty() {
            return (String::new(), Some(1));
        }
        let theme_name = parts[0];
        let new_content = format!("{},{}\n", theme_name, mode);
        fs::write(path, &new_content).unwrap();
        (new_content.trim().to_string(), None)
    }

    #[test]
    fn test_valid_light() {
        let dir = tempfile::TempDir::new().unwrap();
        let path = dir.path().join("dev_theme");
        fs::write(&path, "onedark,dark\n").unwrap();
        let (output, code) = run_tool(&path, "light");
        assert_eq!(code, None);
        assert_eq!(output, "onedark,light");
        assert_eq!(fs::read_to_string(&path).unwrap(), "onedark,light\n");
    }

    #[test]
    fn test_valid_dark() {
        let dir = tempfile::TempDir::new().unwrap();
        let path = dir.path().join("dev_theme");
        fs::write(&path, "onedark,light\n").unwrap();
        let (output, code) = run_tool(&path, "dark");
        assert_eq!(code, None);
        assert_eq!(output, "onedark,dark");
    }

    #[test]
    fn test_malformed_no_comma() {
        let dir = tempfile::TempDir::new().unwrap();
        let path = dir.path().join("dev_theme");
        fs::write(&path, "onedark\n").unwrap();
        let (_, code) = run_tool(&path, "light");
        assert_eq!(code, Some(1));
    }

    #[test]
    fn test_malformed_empty_theme() {
        let dir = tempfile::TempDir::new().unwrap();
        let path = dir.path().join("dev_theme");
        fs::write(&path, ",dark\n").unwrap();
        let (_, code) = run_tool(&path, "light");
        assert_eq!(code, Some(1));
    }
}
