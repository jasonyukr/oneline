use std::io::{self, BufRead};
use std::env;

fn print_usage() {
    println!("{}", "Usage : oneline {-s}");
    println!("   -s   -> use single quote(') character. the default is double quote");
}

fn main() {
    let mut single_mode = false;
    for arg in env::args() {
        if arg == "-h" || arg == "--help" {
            print_usage();
            return;
        } else if arg == "-s" {
            single_mode = true;
        }
    }

    let mut is_first = true;
    let mut out = String::from("");
    let stdin = io::stdin();
    for ln in stdin.lock().lines() {
        if let Ok(line) = ln {
            let line_trim = line.trim();
            if let Some(_) = line_trim.find(' ') {
                if !is_first {
                    out.push_str(" ");
                }
                is_first = false;

                let mut is_quoted = false;
                if line_trim.len() > 2 {
                    if line_trim.starts_with("'") && line_trim.ends_with("'") {
                        is_quoted = true;
                    }
                    if line_trim.starts_with("\"") && line_trim.ends_with("\"") {
                        is_quoted = true;
                    }
                }
                if !is_quoted {
                    if single_mode {
                        out.push_str("'");
                    } else {
                        out.push_str("\"");
                    }
                }

                out.push_str(&line_trim);

                if !is_quoted {
                    if single_mode {
                        out.push_str("'");
                    } else {
                        out.push_str("\"");
                    }
                }
            } else {
                if !is_first {
                    out.push_str(" ");
                }
                is_first = false;

                out.push_str(&line_trim);
            }
        }
    }
    println!("{}", out);
}
