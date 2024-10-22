// Assignmnet 2
// Student Name: Anubhav Aery
// Student Number: 1005839513
use colored::*;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use walkdir::WalkDir;

// Helper function to perform case-insensitive replacement and highlighting
fn highlight_matches(line: &str, search_string: &str, case_insensitive: bool) -> String {
    // If case-sensitive search, simply replace the search string with the colored version
    if !case_insensitive {
        return line.replace(search_string, &search_string.red().to_string());
    }

    // For case-insensitive search, we need to match regardless of case
    let mut result = String::new();
    let mut remaining = line;
    let search_string_lower = search_string.to_lowercase();
    let match_len = search_string.chars().count();

    // Initialize remaining_lower
    let mut remaining_lower = remaining.to_lowercase();

    while let Some(pos) = remaining_lower.find(&search_string_lower) {
        // Find the byte position in the original line
        let byte_pos = remaining.char_indices().nth(pos).unwrap().0;

        // Append everything before the match to the result
        result.push_str(&remaining[..byte_pos]);

        // Get the matched substring from the original line
        let matched_str: String = remaining[byte_pos..].chars().take(match_len).collect();

        // Append the colored matched substring to the result
        result.push_str(&matched_str.red().to_string());

        // Update the remaining part of the line after the matched substring
        let consume_len = byte_pos + matched_str.len();
        remaining = &remaining[consume_len..];

        // Update the lowercase version of the remaining line
        remaining_lower = remaining_lower[byte_pos + matched_str.len()..].to_string();
    }

    // Append any remaining text after the last match
    result.push_str(remaining);
    result
}

// Function to search for the search_string in a given file
fn search_file(
    search_string: &str,
    file_path: &str,
    case_insensitive: bool,
    print_line_numbers: bool,
    invert_match: bool,
    print_filename: bool,
    color_output: bool,
) -> Result<(), io::Error> {
    // Open the file
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    //Read each line from the file
    for (line_num, line) in reader.lines().enumerate() {
        let line = line?;

        // Prepare the line for matching based on case sensitivity
        let line_to_search = if case_insensitive {
            line.to_lowercase()
        } else {
            line.clone()
        };

        // Prepare the search string based on case sensitivity
        let search_pattern = if case_insensitive {
            search_string.to_lowercase()
        } else {
            search_string.to_string()
        };

        // Determine if the line matches the search criteria
        let is_match = line_to_search.contains(&search_pattern);

        // Apply invert match logic
        if invert_match && !is_match || !invert_match && is_match {
            // Apply colour highlighting if enabled
            let output_line = if color_output {
                highlight_matches(&line, search_string, case_insensitive)
            } else {
                line
            };

            // Construct the output based on the flags
            if print_filename {
                if print_line_numbers {
                    // Print filename, line number, and the line
                    println!("{}:{}: {}", file_path, line_num + 1, output_line);
                } else {
                    // Print filename and the line
                    println!("{}: {}", file_path, output_line);
                }
            } else if print_line_numbers {
                // Print line number and the line
                println!("{}: {}", line_num + 1, output_line);
            } else {
                // Print the line only
                println!("{}", output_line);
            }
        }
    }
    Ok(())
}

fn main() {
    // Get command line arguments
    let args: Vec<String> = env::args().collect();

    // Help Menu check
    if args.contains(&"-h".to_string()) || args.contains(&"--help".to_string()) {
        println!("Usage: grep [OPTIONS] <pattern> <files...>");
        println!("Options:");
        println!("-i                Case-insensitive search");
        println!("-n                Print line numbers");
        println!("-v                Invert match (exclude lines that match the pattern)");
        println!("-r                Recursive directory search");
        println!("-f                Print filenames");
        println!("-c                Enable colored output");
        println!("-h, --help        Show help information");
        return;
    }

    // Initialize option flags
    let mut case_insensitive = false;
    let mut print_line_numbers = false;
    let mut invert_match = false;
    let mut recursive_search = false;
    let mut print_filename = false;
    let mut color_output = false;

    let mut search_string: Option<String> = None;
    let mut files = Vec::new();

    // Parse command line arguments
    let args_iter = args.iter().skip(1); // To skip program name, and focus on actual command line arguments
    for arg in args_iter {
        match arg.as_str() {
            "-i" => case_insensitive = true,
            "-n" => print_line_numbers = true,
            "-v" => invert_match = true,
            "-r" => recursive_search = true,
            "-f" => print_filename = true,
            "-c" => color_output = true,
            _ => {
                // The first non-option argument is the search string
                if search_string.is_none() {
                    search_string = Some(arg.clone());
                } else {
                    // Remaining arguments are file pathssss
                    files.push(arg.clone());
                }
            }
        }
    }

    let search_string = search_string.unwrap();

    // Perform search on files
    if recursive_search {
        // Recursive search through directories
        for file_path in &files {
            for entry in WalkDir::new(file_path).into_iter().filter_map(Result::ok) {
                let path = entry.path();
                if path.is_file() {
                    if let Some(file_path_str) = path.to_str() {
                        // Search in each file
                        search_file(
                            &search_string,
                            file_path_str,
                            case_insensitive,
                            print_line_numbers,
                            invert_match,
                            print_filename,
                            color_output,
                        )
                        .unwrap();
                    }
                }
            }
        }
    } else {
        // Non-recursive search in specified files
        for file_path in &files {
            search_file(
                &search_string,
                file_path,
                case_insensitive,
                print_line_numbers,
                invert_match,
                print_filename,
                color_output,
            )
            .unwrap();
        }
    }
}
