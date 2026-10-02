use crate::archive::bible::kjv1611::kjv_1611::lookup;

use std::io::{Write, stdin, stdout};

fn read(input: &mut String) {
    stdout().flush().expect("failed to flush");
    stdin().read_line(input).expect("failed to read");
}

pub fn kjv_1611_cli_funk() {
    loop {
        print!("read> kjv1611> ");

        let mut command = String::new();

        read(&mut command);

        let command = command.trim().to_lowercase();

        match command.as_str() {
            "help" => {
                println!("Bible commands.");
                println!("type BOOK CHAPTER:VERSE");
            }

            "index" => {
                println!("Bible index.");
                println!("Genesis, Exodus, Leviticus, Numbers, Deuteronomy, Joshua, Judges, Ruth, 1 Samuel, 2 Samuel, 1 Kings, 2 Kings, 1 Chronicles, 2 Chronicles, Ezra, Nehemiah, Esther, Job, Psalms, Proverbs, Ecclesiastes, Song of Solomon, Isaiah, Jeremiah, Lamentations, Ezekiel, Daniel, Hosea, Joel, Amos, Obadiah, Jonah, Micah, Nahum, Habakkuk, Zephaniah, Haggai, Zechariah, Malachi",);
            }

            "exit" => {
                println!("Jesus loves you.");
                break;
            }

            _ => {
                let bible_request: Vec<&str> = command.split_whitespace().collect();

                if bible_request.len() != 2 {
                    println!("Invalid reference.");
                }

                let book = bible_request[0];

                //i

                let reference: Vec<&str> = bible_request[1].split(':').collect();

                let verse_range: Vec<&str> = reference[1].split('-').collect();

                //println!("Book: {}, Chapter: {}, Verse: {}", book, reference[0], reference[1]);

                if verse_range.len() == 1 {
                    let chapter: u8 = match reference[0].parse() {
                        Ok(chapter) => chapter,

                        Err(_) => {
                            println!("Invalid chapter.");
                            continue;
                        }
                    };

                    let verse: u8 = match reference[1].parse() {
                        Ok(verse) => verse,

                        Err(_) => {
                            println!("Invalid verse.");
                            continue;
                        }
                    };

                    let result = lookup(book, chapter, verse);

                    match result {
                        Some(verse) => println!("{}", verse),
                        None => println!("Verse not found."),
                    }
                } else if verse_range.len() == 2 {
                    let chapter: u8 = match reference[0].parse() {
                        Ok(chapter) => chapter,

                        Err(_) => {
                            println!("Invalid chapter.");
                            continue;
                        }
                    };

                    let start_verse: u8 = match verse_range[0].parse() {
                        Ok(start_verse) => start_verse,

                        Err(_) => {
                            println!("Invalid start verse.");
                            continue;
                        }
                    };

                    let end_verse: u8 = match verse_range[1].parse() {
                        Ok(end_verse) => end_verse,

                        Err(_) => {
                            println!("Invalid end verse.");
                            continue;
                        }
                    };

                    if start_verse > end_verse {
                        println!("Start verse cannot be greater than end verse.");
                        continue;
                    }

                    for verse in start_verse..=end_verse {
                        let result = lookup(book, chapter, verse);

                        match result {
                            Some(verse_text) => println!("{}", verse_text),
                            None => println!("Verse {} not found.", verse),
                        }
                    }
                } else {
                    println!("Invalid reference format. Use BOOK CHAPTER:VERSE or BOOK CHAPTER:START_VERSE-END_VERSE.");
                }

                /* 

                if reference.len() != 2 {
                    println!("Invalid reference. Use chapter:verse.");
                    continue;
                }

                if reference[0].parse::<u8>().is_err() || reference[1].parse::<u8>().is_err() {
                    println!("Invalid chapter or verse.");
                    continue;
                }

                if reference[0].parse::<u8>().unwrap() == 0 || reference[1].parse::<u8>().unwrap() == 0 {
                    println!("Chapter and verse must be greater than 0.");
                    continue;
                }

                if reference[0].parse::<u8>().unwrap() > 150 {
                    println!("Chapter out of range. Maximum chapter is 150.");
                    continue;
                }

                if reference[1].parse::<u8>().unwrap() > 176 {
                    println!("Verse out of range. Maximum verse is 176.");
                    continue;
                }

                if reference.len() == 2 && reference[0].parse::<u8>().is_ok() && reference[1].parse::<u8>().is_ok() {
                    let chapter: u8 = reference[0].parse().unwrap();
                    let verse: u8 = reference[1].parse().unwrap();

                    let result = lookup(book, chapter, verse);

                    match result {
                        Some(verse) => println!("{}", verse),
                        None => println!("Verse not found."),
                    }
                } else {
                    println!("Invalid reference format. Use BOOK CHAPTER:VERSE.");
                }

                */
                

                /*

                let chapter: u8 = match reference[0].parse() {
                    Ok(chapter) => chapter,
                    
                    Err(_) => {
                        println!("Invalid chapter.");
                        continue;
                    }
                };

                let verse: u8 = match reference[1].parse() {
                    Ok(verse) => verse,

                    Err(_) => {
                        println!("Invalid verse.");
                        continue;
                    }
                };

                let result = lookup(book, chapter, verse);

                match result {
                    Some(verse) => println!("{}", verse),
                    None => println!("Verse not found."),
                }

                */
            } //END OF _ => PARSE
        } //END OF MATCH COMMAND
    } //END OF LOOP
} //END OF BIBLE CLI FUNCTION
