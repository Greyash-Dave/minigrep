use std::error::Error;
use std::fs;
use std::env;

// ANSI color codes
const RESET: &str = "\x1b[0m";
const RED: &str = "\x1b[31m";
const GREEN: &str = "\x1b[32m";
const YELLOW: &str = "\x1b[33m";
const BLUE: &str = "\x1b[34m";
const MAGENTA: &str = "\x1b[35m";
const CYAN: &str = "\x1b[36m";

#[derive(Debug, PartialEq)]
pub enum ColorMode {
    Always,
    Auto,
    Never,
}

impl ColorMode {
    fn from_str(s: &str) -> Result<Self, &'static str> {
        match s.to_lowercase().as_str() {
            "always" => Ok(ColorMode::Always),
            "auto" => Ok(ColorMode::Auto),
            "never" => Ok(ColorMode::Never),
            _ => Err("invalid color mode, use 'always', 'auto', or 'never'"),
        }
    }
}

#[derive(Debug)]
pub enum HighlightColor {
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
}

impl HighlightColor {
    fn from_str(s: &str) -> Result<Self, &'static str> {
        match s.to_lowercase().as_str() {
            "red" => Ok(HighlightColor::Red),
            "green" => Ok(HighlightColor::Green),
            "yellow" => Ok(HighlightColor::Yellow),
            "blue" => Ok(HighlightColor::Blue),
            "magenta" => Ok(HighlightColor::Magenta),
            "cyan" => Ok(HighlightColor::Cyan),
            _ => Err("invalid highlight color"),
        }
    }

    fn code(&self) -> &'static str {
        match self {
            HighlightColor::Red => RED,
            HighlightColor::Green => GREEN,
            HighlightColor::Yellow => YELLOW,
            HighlightColor::Blue => BLUE,
            HighlightColor::Magenta => MAGENTA,
            HighlightColor::Cyan => CYAN,
        }
    }
}

pub struct Config {
    pub case_sensitive: bool,
    pub invert: bool,
    pub count_only: bool,
    pub query: String,
    pub filename: String,
    pub color_mode: ColorMode,
    pub highlight_color: HighlightColor,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        // Default values
        let mut case_sensitive = true; // Default is case-sensitive
        let mut invert = false; // Default is normal search
        let mut count_only = false; // Default is to show lines
        let mut color_mode = ColorMode::Auto; // Default is auto
        let mut highlight_color = HighlightColor::Red; // Default is red
        
        let mut query_index = 1;
        
        // Check for environment variable CASE_INSENSITIVE
        if env::var("CASE_INSENSITIVE").is_ok() {
            case_sensitive = false;
        }
        
        // Parse flags if present
        if args[1].starts_with('-') {
            let mut i = 1;
            while i < args.len() && args[i].starts_with('-') {
                if args[i] == "-i" {
                    case_sensitive = false;
                } else if args[i] == "-v" {
                    invert = true;
                } else if args[i] == "-n" {
                    count_only = true;
                } else if args[i].starts_with("--color=") {
                    let color_value = args[i].trim_start_matches("--color=");
                    color_mode = ColorMode::from_str(color_value)?;
                } else if args[i].starts_with("--highlight=") {
                    let highlight_value = args[i].trim_start_matches("--highlight=");
                    highlight_color = HighlightColor::from_str(highlight_value)?;
                } else {
                    return Err("invalid flag argument");
                }
                i += 1;
            }
            
            // Set query_index to the first non-flag argument
            query_index = i;
            
            // If we didn't find a non-flag argument, return error
            if query_index >= args.len() {
                return Err("missing query argument");
            }
        }
        
        if query_index + 1 >= args.len() {
            return Err("missing filename argument");
        }
        
        let query = args[query_index].clone();
        let filename = args[query_index + 1].clone();

        Ok(Config { 
            case_sensitive, 
            invert, 
            count_only, 
            query, 
            filename, 
            color_mode,
            highlight_color
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("Searching for '{}' in file '{}'", config.query, config.filename);

    let contents = fs::read_to_string(&config.filename)?;

    // Determine if we should use colors
    let use_color = match config.color_mode {
        ColorMode::Always => true,
        ColorMode::Never => false,
        ColorMode::Auto => atty::is(atty::Stream::Stdout),
    };

    if config.count_only {
        // Count mode
        let count = search_count(
            &config.query, 
            &contents, 
            !config.case_sensitive,
            config.invert
        );
        if use_color {
            println!("Found {}{}{} occurrences of '{}'", 
                config.highlight_color.code(),  // Start color
                count, 
                RESET,  // Reset color
                config.query
            );
        } else {
            println!("Found {} occurrences of '{}'", count, config.query);
        }
    } else {
        // Line output mode
        let results = if config.invert {
            // Invert search
            search_invert(&config.query, &contents, !config.case_sensitive)
        } else if !config.case_sensitive {
            // Case-insensitive search
            search_case_insensitive(&config.query, &contents)
        } else {
            // Default case-sensitive search
            search(&config.query, &contents)
        };
        
        // Print results with or without color
        for (line_num, line, matches) in results {
            if use_color {
                println!("{}{}{}: {}", 
                config.highlight_color.code(),  // Apply color
                line_num,
                RESET,  // Reset color
                highlight_matches(line, matches, config.highlight_color.code())
            );
            } else {
                println!("{}: {}", line_num, line);
            }
        }
    }

    Ok(())
}

// Highlight matches within a line
fn highlight_matches(line: &str, matches: Vec<(usize, usize)>, color_code: &str) -> String {
    let mut result = String::new();
    let mut last_end = 0;
    
    for (start, end) in matches {
        // Add text before this match
        result.push_str(&line[last_end..start]);
        // Add colored match
        result.push_str(color_code);
        result.push_str(&line[start..end]);
        result.push_str(RESET);
        
        last_end = end;
    }
    
    // Add remaining text after last match
    result.push_str(&line[last_end..]);
    
    result
}

// Returns count of matching lines
pub fn search_count(query: &str, contents: &str, case_insensitive: bool, invert: bool) -> i32 {
    let mut count = 0;
    
    let query = if case_insensitive { query.to_lowercase() } else { query.to_string() };
    
    for line in contents.lines() {
        let line_to_check = if case_insensitive { line.to_lowercase() } else { line.to_string() };
        let contains_match = line_to_check.contains(&query);
        
        if (contains_match && !invert) || (!contains_match && invert) {
            count += 1;
        }
    }

    count
}

// Return line numbers, lines, and match positions for highlighting
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<(usize, &'a str, Vec<(usize, usize)>)> {
    let mut results = Vec::new();
    let mut line_num = 1;

    for line in contents.lines() {
        // Find all occurrences of query in this line
        let mut matches = Vec::new();
        let mut start = 0;
        
        while let Some(pos) = line[start..].find(query) {
            let absolute_start = start + pos;
            let absolute_end = absolute_start + query.len();
            matches.push((absolute_start, absolute_end));
            start = absolute_end;
        }
        
        if !matches.is_empty() {
            results.push((line_num, line, matches));
        }
        
        line_num += 1;
    }

    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<(usize, &'a str, Vec<(usize, usize)>)> {
    let query = query.to_lowercase();
    let mut results = Vec::new();
    let mut line_num = 1;

    for line in contents.lines() {
        // Find all occurrences of query in this line, case insensitive
        let mut matches = Vec::new();
        let lowercase_line = line.to_lowercase();
        let mut start = 0;
        
        while let Some(pos) = lowercase_line[start..].find(&query) {
            let absolute_start = start + pos;
            let absolute_end = absolute_start + query.len();
            matches.push((absolute_start, absolute_end));
            start = absolute_end;
        }
        
        if !matches.is_empty() {
            results.push((line_num, line, matches));
        }
        
        line_num += 1;
    }

    results
}

pub fn search_invert<'a>(query: &str, contents: &'a str, case_insensitive: bool) -> Vec<(usize, &'a str, Vec<(usize, usize)>)> {
    let mut results = Vec::new();
    let mut line_num = 1;

    let query = if case_insensitive { query.to_lowercase() } else { query.to_string() };

    for line in contents.lines() {
        let line_to_check = if case_insensitive { line.to_lowercase() } else { line.to_string() };
        
        if !line_to_check.contains(&query) {
            // For inverted search, no highlighting needed, so empty vec
            results.push((line_num, line, Vec::new()));
        }
        
        line_num += 1;
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        let results = search(query, contents);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].0, 2); // Line number
        assert_eq!(results[0].1, "safe, fast, productive."); // Line content
        assert_eq!(results[0].2, vec![(10, 14)]); // Match positions
    }

    #[test]
    fn test_search_case_insensitive() {
        let query = "RuSt";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        let results = search_case_insensitive(query, contents);
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].1, "Rust:"); // First matching line
        assert_eq!(results[1].1, "Trust me."); // Second matching line
    }

    #[test]
    fn test_search_invert() {
        let query = "rust";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        let results = search_invert(query, contents, true);
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].1, "safe, fast, productive."); // First non-matching line
        assert_eq!(results[1].1, "Pick three."); // Second non-matching line
    }

    #[test]
    fn test_color_mode_from_str() {
        assert_eq!(ColorMode::from_str("always").unwrap(), ColorMode::Always);
        assert_eq!(ColorMode::from_str("auto").unwrap(), ColorMode::Auto);
        assert_eq!(ColorMode::from_str("never").unwrap(), ColorMode::Never);
        assert!(ColorMode::from_str("invalid").is_err());
    }

    #[test]
    fn test_highlight_matches() {
        let line = "The quick brown fox jumps over the lazy dog";
        let matches = vec![(4, 9), (16, 19)]; // "quick" and "fox"
        
        let highlighted = highlight_matches(line, matches, RED);
        assert_eq!(
            highlighted, 
            format!("The {}quick{} brown {}fox{} jumps over the lazy dog", RED, RESET, RED, RESET)
        );
    }
}