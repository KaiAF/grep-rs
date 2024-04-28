use regex::Regex;
use std::{collections::HashMap, env, fs, process::exit};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!(
            "Usage: grep [OPTION]... PATTERN [FILE]...\nTry 'grep --help' for more information"
        );

        exit(1);
    }

    let options_bool = parse_options_bool(&args);
    let options_int = parse_options_int(&args);
    let ignore_case = options_bool.get("ignore_case").expect("");
    let line_number = options_bool.get("line_number").expect("");
    let is_help = options_bool.get("help").expect("");
    let is_version = options_bool.get("version").expect("");
    let should_show_colour = options_bool.get("should_show_colour").expect("");
    let max_num = options_int.get("max_num").expect("").to_owned();

    if *is_help {
        println!("Usage: grep [OPTION]... PATTERN [FILE]...\nSearch for PATTERN in each FILE or standard input.\nPATTERN is, by default, a basic regular expression (BRE).\nExample: grep -i 'hello world' menu.h main.c\n");
        println!("Regexp selection and interpretation:");
        println!("\t-i, --ignore-case     ignore case distinctions");
        println!("Miscellaneous:");
        println!("\t-V, --version         display version information and exit");
        println!("\t    --help            display this help text and exit");
        println!("\nOutput control:");
        println!("\t-m                    stop after NUM matches");
        println!("\t-n, --line-number     print line number with output lines");
        println!("Control control:");
        println!("\t-c, --colour          displays a colour on any words that were found");
        println!("");
        println!("Report bugs to: https://github.com/KaiAF/grep-rs/issues\ngrep rust home page: https://github.com/KaiAF/grep-rs");

        exit(0);
    }

    if *is_version {
        println!("grep (grep-rs) 1.0.1");
        println!("License MIT <https://github.com/KaiAF/grep-rs/blob/master/LICENSE>");
        println!("");
        println!("Written by Iris Zol");

        exit(0);
    }

    let file = &args[args.len() - 1];
    match fs::read_to_string(file) {
        Ok(file_content) => {
            let content_array = file_content.split("\n");

            let mut results: Vec<String> = vec![];
            for (i, str) in content_array.into_iter().enumerate() {
                if max_num > 0 && results.len() > max_num.try_into().unwrap() {
                    break;
                }

                let mut line = String::new();
                if *line_number {
                    line = format!("{}:", i + 1);
                }

                let formatted_str = format!("{}{}", line, str);
                let mut flags = "";
                if *ignore_case {
                    flags = "(?i)";
                }

                let regex = Regex::new(
                    format!(r"{}{}", flags, &args[args.len() - 2])
                        .as_str()
                        .to_owned()
                        .as_str(),
                )
                .unwrap();

                if regex.is_match(str) {
                    let mut replaced = formatted_str.clone();
                    if *should_show_colour {
                        replaced = regex
                            .replace(&formatted_str, format!("\u{1b}[31m{}\u{1b}[0m", "$0"))
                            .to_string();
                    }

                    results.push(replaced.to_string());
                }
            }

            println!("{}", results.join("\n"))
        }
        Err(_err) => {
            println!("grep: {}: No such file or directory", file);
            exit(1);
        }
    }
}

fn parse_options_bool(args: &Vec<String>) -> HashMap<&str, bool> {
    let options = &args[1..];
    let mut mapped_options: HashMap<&str, bool> = HashMap::new();
    mapped_options.insert("ignore_case", false);
    mapped_options.insert("line_number", false);
    mapped_options.insert("help", false);
    mapped_options.insert("version", false);
    mapped_options.insert("should_show_colour", false);

    if options.len() > 0 {
        for option in options {
            if option.eq("-i") || option.eq("--ignore-case") {
                mapped_options.insert("ignore_case", true);
            }

            if option.eq("-n") || option.eq("--line-number") {
                mapped_options.insert("line_number", true);
            }

            if option.eq("--help") {
                mapped_options.insert("help", true);
            }

            if option.eq("-V") || option.eq("--version") {
                mapped_options.insert("version", true);
            }

            if option.eq("-c") || option.eq("--colour") {
                mapped_options.insert("should_show_colour", true);
            }
        }
    }

    return mapped_options;
}

fn parse_options_int(args: &Vec<String>) -> HashMap<&str, i32> {
    let options = &args[1..];
    let mut mapped_options: HashMap<&str, i32> = HashMap::new();
    mapped_options.insert("max_num", 0);

    if options.len() > 0 {
        for (i, option) in options.iter().enumerate() {
            if option.eq("-m") {
                mapped_options.insert("max_num", options[i + 1].parse().unwrap());
            }
        }
    }

    return mapped_options;
}
