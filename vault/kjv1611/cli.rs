// IMPORT

use crate::archive::bible::kjv1611::corpus::get;
use crate::foundation::input::text::capitalize_first_letter;

use std::io::{Write, stdin, stdout};

fn read(input: &mut String) {
    stdout().flush().expect("failed to flush");
    stdin().read_line(input).expect("failed to read");
}

pub fn kjv_1611_interface() {
    loop {
        print!("GPSE> COMMAND> READ> 1611 KJV> ");

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
                println!(
                    "Genesis, Exodus, Leviticus, Numbers, Deuteronomy, Joshua, Judges, Ruth, 1 Samuel, 2 Samuel, 1 Kings, 2 Kings, 1 Chronicles, 2 Chronicles, Ezra, Nehemiah, Esther, Job, Psalms, Proverbs, Ecclesiastes, Song of Solomon, Isaiah, Jeremiah, Lamentations, Ezekiel, Daniel, Hosea, Joel, Amos, Obadiah, Jonah, Micah, Nahum, Habakkuk, Zephaniah, Haggai, Zechariah, Malachi",
                );

                println!(
                    "Matthew, Mark, Luke, John, Acts, Romans, 1 Corinthians, 2 Corinthians, Galatians, Ephesians, Philippians, Colossians, 1 Thessalonians, 2 Thessalonians, 1 Timothy, 2 Timothy, Titus, Philemon, Hebrews, James, 1 Peter, 2 Peter, 1 John, 2 John, 3 John, Jude, Revelations"
                );
            }

            "verse index" => {

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

                let reference: Vec<&str> = bible_request[1].split(':').collect();

                let verse_range: Vec<&str> = reference[1].split('-').collect();

                if verse_range.len() == 1 {
                    println!(
                        "The book of {}, chapter {}, verse {}:",
                        capitalize_first_letter(book),
                        reference[0],
                        reference[1]
                    );
                    println!("-------------------------------------------------");
                    println!();

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

                    let result = get(book, chapter, verse);

                    match result {
                        Some(verse) => println!("{}", verse),
                        None => println!("Verse not found."),
                    }
                } else if verse_range.len() == 2 {
                    print!(
                        "The book of {}, chapter {}, verses {} through {}:\n",
                        capitalize_first_letter(book),
                        reference[0],
                        verse_range[0],
                        verse_range[1]
                    );
                    println!("-------------------------------------------------");
                    println!();

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
                        let result = get(book, chapter, verse);

                        match result {
                            Some(verse_text) => println!("{}", verse_text),
                            None => println!("Verse {} not found.", verse),
                        }
                    }
                } else {
                    println!(
                        "Invalid reference format. Use \"book chapter:verse\" or \"book chapter:start_verse-end_verse\"."
                    );
                }
            } //END OF _ => PARSE
        } //END OF MATCH COMMAND
    } //END OF LOOP
} //END OF BIBLE CLI FUNCTION