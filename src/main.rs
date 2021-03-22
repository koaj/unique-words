use std::{env::args, io::Read};

fn main() {
    // All arguments list.
    let all_args: Vec<String> = args().collect();

    // Check for file input as the second arg.
    if all_args.len() > 1 {
        // File path to read.
        use std::process;
        let mut file = std::fs::File::open(&all_args[1]).unwrap_or_else(|err| {
            println!("There was a problem, please check: {}", err);
            process::exit(1);
        });

        // File content.
        let mut file_content = String::new();
        file.read_to_string(&mut file_content).unwrap();

        // Spliting all words and turn them in a vector.
        let all_lines = file_content
            .split(
                &[
                    '\n', '!', '(', ')', '{', '}', '/', '.', '=', ';', ',', '\'', ':', '"', ']',
                    '[', '&', '@', '$', '%', '+', '*', '#', '`', '\\', '<', '>', ' ', ' ', '?',
                    '~', '|',
                ][..]
            )
            .collect::<Vec<&str>>();

        let mut words_list: Vec<&str> = Vec::new();

        let index = 0;

        for i in all_lines.iter() {
            for w in i.split(' ').collect::<Vec<_>>() {
                if !w.is_empty() && w.len() > 1 {
                    words_list.insert(index, w);
                }
            }
        }

        // Make a unique list of words.
        let mut unique_list: Vec<&str> = Vec::new();

        for word in words_list.iter() {
            if !unique_list.contains(word) {
                let unique_index = 0;
                unique_list.insert(unique_index, word)
            }
        }

        for word in unique_list {
            println!("{}", word)
        }
    } else {
        eprint!("Enter a file path as input.")
    }
}
