use std::env;
use std::fs;
use std::io::{self, Write};

fn print_help() {
    println!("Rust File Analyzer");

    println!("Usage: cargo run <FILE_PATH> [OPTION]");

    println!("\nOptions: ");

    println!(" -s, --size : Display file size in bytes.");

    println!(" -b, --bytes : Perform and display byte frequency analysis.");

    println!(" -p, --pattern <PATTERN>: Search for a specific byte pattern in the file.");

    println!(" -h, --help : Display this help message.");

    println!("\nExamples:");

    println!(" cargo run data.txt --size");

    println!(" cargo run image.png --bytes");

    println!(" cargo run my_binary.exe --pattern 4D5A (for MZ header)");

    println!(" cargo run my_document.pdf");
}

// read file content
fn read_file_contents(file_path: &str) -> Result<Vec<u8>, io::Error> {
    println!("\nAttempting to read file: '{}'", file_path);

    fs::read(file_path)
}

fn get_file_size(file_contents: &Vec<u8>) -> usize {
    file_contents.len()
}
fn analyze_byte_frequency(file_contents: &Vec<u8>, counts: &mut [usize; 256]) {
    println!("Performing byte frequency analysis...");

    for &byte in file_contents.iter() {
        counts[byte as usize] += 1;
    }
    println!("Analysis complete. Displaying top 10 most frequent bytes:");

    let mut byte_stats: Vec<(u8, usize)> = Vec::new();

    for i in 0..256 {
        if counts[i] > 0 {
            byte_stats.push((i as u8, counts[i]));
        }
    }

    byte_stats.sort_by(|a, b| b.1.cmp(&a.1));

    for (i, (byte_val, count)) in byte_stats.iter().take(10).enumerate() {
        println!(
            "{}. Byte: 0x{:02x} ('{}'), Count: {}",
            i + 1,
            byte_val,
            *byte_val as char,
            count
        );
    }
}

fn search_pattern(file_contents: &[u8], pattern: &[u8]) {
    println!("Searching for pattern: {:?}", pattern);

    let mut found_count = 0;

    for (i, window) in file_contents.windows(pattern.len()).enumerate() {
        if window == pattern {
            println!(" Found pattern at offset: {}", i);

            found_count += 1;
        }
    }
    if found_count == 0 {
        println!(" Pattern not found.");
    } else {
        println!(" Found pattern {} time(s).", found_count);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_help();

        return;
    }

    let file_path = &args[1];

    let file_contents_result = read_file_contents(file_path);

    let file_contents = match file_contents_result {
        Ok(contents) => {
            println!(
                "Successfully read {} bytes from '{}'",
                contents.len(),
                file_path
            );
            contents
        }
        Err(error) => {
            eprintln!("Error reading file '{}': {}", file_path, error);

            eprintln!("Please check the file path and permissions.");

            return;
        }
    };

    let mut has_option = false;
    for i in 2..args.len() {
        match args[i].as_str() {
            "-s" | "--size" => {
                has_option = true;

                let size = get_file_size(&file_contents);

                println!("File size: {} bytes", size)
            }
            "-b" | "--bytes" => {
                has_option = true;
                let mut byte_counts: [usize; 256] = [0; 256];

                analyze_byte_frequency(&file_contents, &mut byte_counts);
            }
            "-p" | "--patters" => {
                has_option = true;
                if let Some(pattern_str) = args.get(i + 1) {
                    let pattern_bytes: Vec<u8> = (0..pattern_str.len())
                        .step_by(2)
                        .map(|i| u8::from_str_radix(&pattern_str[i..i + 2], 16).unwrap_or(0))
                        .collect();

                    search_pattern(&file_contents, &pattern_bytes);
                } else {
                    eprintln!("Error: --pattern requires a hexadecimal pattern argument.");

                    print_help();
                }
            }
            "-h" | "--help" => {
                print_help();

                has_option = true;
            }
            _ => {
                // If it's an unrecognized option, and not the pattern argument we just consumed

                if args[i - 1] != "-p" && args[i - 1] != "--pattern" {
                    // Avoid re-processing pattern arg

                    eprintln!("Warning: Unrecognized option '{}'. Ignoring.", args[i]);
                }
            }
        }
    }
    if !has_option {
        println!("No specific options provided. File successfully read.");
    }

    println!("\nFile Analyzer finished.");
}

// use sha2::{Digest, Sha256};
// use std::env;
// use std::fs;
// use std::io::{self, Write};

// /// Known magic numbers for common file types
// const MAGIC_NUMBERS: &[(&str, &[u8])] = &[
//     ("PE/EXE", &[0x4D, 0x5A]),          // MZ for Windows PE files
//     ("ELF", &[0x7F, 0x45, 0x4C, 0x46]), // 0x7F E L F for Linux ELF files
//     ("PDF", &[0x25, 0x50, 0x44, 0x46]), // %PDF for PDF files
//     ("ZIP", &[0x50, 0x4B, 0x03, 0x04]), // PK.. for ZIP/Office files
//     ("PNG", &[0x89, 0x50, 0x4E, 0x47]), // PNG header
// ];

// fn print_help() {
//     println!("Rust Cyber File Analyzer");
//     println!("Usage: cargo run <FILE_PATH> [OPTIONS]");
//     println!("\nOptions:");
//     println!("  -b, --bytes               Perform byte frequency analysis");
//     println!("  -p, --pattern <PATTERN>   Search for a specific byte pattern in hex (e.g., 4D5A)");
//     println!("  -h, --help                Show this help message");
// }

// fn read_file_contents(file_path: &str) -> Result<Vec<u8>, io::Error> {
//     fs::read(file_path)
// }

// fn get_file_size(file_contents: &[u8]) -> usize {
//     file_contents.len()
// }

// fn calculate_sha256(file_contents: &[u8]) -> String {
//     let mut hasher = Sha256::new();
//     hasher.update(file_contents);
//     let result = hasher.finalize();
//     result.iter().map(|b| format!("{:02x}", b)).collect()
// }

// fn detect_magic_headers(file_contents: &[u8]) {
//     println!("Magic Header Detection:");
//     for (file_type, magic) in MAGIC_NUMBERS {
//         if file_contents.starts_with(magic) {
//             println!(" -> Detected: {} (Header: {:X?})", file_type, magic);
//         } else {
//             for (i, window) in file_contents.windows(magic.len()).enumerate() {
//                 if window == *magic && i != 0 {
//                     println!(" !! Suspicious: {} header found at offset {}", file_type, i);
//                 }
//             }
//         }
//     }
// }

// fn analyze_byte_frequency(file_contents: &[u8], counts: &mut [usize; 256]) {
//     println!("Performing byte frequency analysis...");
//     for &byte in file_contents.iter() {
//         counts[byte as usize] += 1;
//     }
//     let mut byte_stats: Vec<(u8, usize)> = counts
//         .iter()
//         .enumerate()
//         .filter(|&(_, &count)| count > 0)
//         .map(|(val, &count)| (val as u8, count))
//         .collect();

//     byte_stats.sort_by(|a, b| b.1.cmp(&a.1));

//     println!("Top 10 Most Frequent Bytes:");
//     for (i, (byte_val, count)) in byte_stats.iter().take(10).enumerate() {
//         println!(
//             "{}. Byte: 0x{:02x} ('{}') - Count: {}",
//             i + 1,
//             byte_val,
//             *byte_val as char,
//             count
//         );
//     }
// }

// fn search_pattern(file_contents: &[u8], pattern: &[u8]) {
//     println!("Searching for pattern: {:X?}", pattern);
//     let mut found_count = 0;
//     for (i, window) in file_contents.windows(pattern.len()).enumerate() {
//         if window == pattern {
//             println!(" -> Found at offset: {}", i);
//             found_count += 1;
//         }
//     }
//     if found_count == 0 {
//         println!("Pattern not found.");
//     } else {
//         println!("Total Occurrences: {}", found_count);
//     }
// }

// fn main() {
//     let args: Vec<String> = env::args().collect();
//     if args.len() < 2 {
//         print_help();
//         return;
//     }

//     let file_path = &args[1];
//     let file_contents = match read_file_contents(file_path) {
//         Ok(contents) => contents,
//         Err(e) => {
//             eprintln!("Error reading file: {}", e);
//             return;
//         }
//     };

//     println!("=== File Analysis: {} ===", file_path);
//     println!("File Size: {} bytes", get_file_size(&file_contents));
//     println!("SHA256: {}", calculate_sha256(&file_contents));
//     println!();

//     detect_magic_headers(&file_contents);
//     println!();

//     let mut has_option = false;

//     for i in 2..args.len() {
//         match args[i].as_str() {
//             "-b" | "--bytes" => {
//                 has_option = true;
//                 let mut byte_counts = [0usize; 256];
//                 analyze_byte_frequency(&file_contents, &mut byte_counts);
//             }
//             "-p" | "--pattern" => {
//                 has_option = true;
//                 if let Some(pattern_str) = args.get(i + 1) {
//                     let pattern_bytes: Vec<u8> = (0..pattern_str.len())
//                         .step_by(2)
//                         .map(|i| u8::from_str_radix(&pattern_str[i..i + 2], 16).unwrap_or(0))
//                         .collect();
//                     search_pattern(&file_contents, &pattern_bytes);
//                 } else {
//                     eprintln!("Error: --pattern requires hex string argument.");
//                 }
//             }
//             "-h" | "--help" => {
//                 print_help();
//                 has_option = true;
//             }
//             _ => {}
//         }
//     }

//     if !has_option {
//         println!("No specific analysis option provided. Displayed basic info.");
//     }
// }
