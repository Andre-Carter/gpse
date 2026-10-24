use crate::archive::bible::kjv1611::kjv_1611::lookup_kjv1611;
use crate::systems::command::core::command_core::captialize_first_letter;

use std::io::{Write, stdin, stdout};

fn read(input: &mut String) {
    stdout().flush().expect("failed to flush");
    stdin().read_line(input).expect("failed to read");
}

pub fn bible_verse_index() {
    println!(
        "Genesis 1-50: 1:1-31, 2:1-25, 3:1-24, 4:1-26, 5:1-32, 6:1-22, 7:1-24, 8:1-22, 9:1-29, 10:1-32, 11:1-32, 12:1-20, 13:1-18, 14:1-24, 15:1-21, 16:1-16, 17:1-27, 18:1-33, 19:1-38, 20:1-18, 21:1-34, 22:1-24, 23:1-20, 24:1-67, 25:1-34, 26:1-35, 27:1-46, 28:1-22, 29:1-35, 30:1-43, 31:1-55, 32:1-32, 33:1-20, 34:1-31, 35:1-29, 36:1-43, 37:1-36, 38:1-30, 39:1-23, 40:1-23, 41:1-57, 42:1-38, 43:1-34, 44:1-34, 45:1-28, 46:1-34, 47:1-31, 48:1-22, 49:1-33, 50:1-26."
    );

    println!(
        ""
    );

    println!(
        ""
    );

    println!(
        ""
    );

    println!(
        ""
    );

    println!(
        ""
    );

    println!(
        ""
    );

    println!(
        ""
    );

    println!(
        ""
    );

    println!(
        ""
    );

    println!(
        ""
    );

    println!(
        ""
    );

    println!(
        ""
    );

    println!(
        ""
    );

    println!(
        ""
    );

    println!(
        ""
    );

    println!(
        ""
    );

    println!(
        ""
    );

    println!(
        ""
    );

    println!(
        ""
    );

    println!(
        ""
    );

    println!(
        ""
    );

    println!(
        ""
    );

    println!(
        ""
    );

    println!(
        ""
    );

    println!(
        ""
    );

    println!(
        ""
    );

    println!(
        ""
    );

    println!(
        ""
    );

    println!(
        ""
    );

    println!(
        ""
    );

    println!(
        ""
    );

    println!(
        ""
    );

    println!(
        ""
    );

    println!(
        ""
    );

    println!(
        ""
    );

    println!(
        ""
    );

    println!(
        ""
    );

    println!(
        ""
    );

    println!(
        ""
    );

    println!(
        ""
    );

    println!(
        ""
    );

    println!(
        ""
    );

    println!(
        ""
    );

    println!(
        ""
    );

    println!(
        ""
    );

    println!(
        ""
    );

    println!(
        ""
    );

    println!(
        ""
    );

    println!(
        ""
    );

    println!(
        ""
    );

    println!(
        ""
    );

    println!(
        ""
    );

    println!(
        ""
    );

    println!(
        ""
    );

    println!(
        ""
    );

    println!(
        ""
    );

    println!(
        ""
    );

    println!(
        ""
    );

    println!(
        ""
    );

    println!(
        ""
    );

    println!(
        ""
    );

    println!(
        ""
    );

    println!(
        ""
    );

    println!(
        ""
    );

    println!(
        ""
    );

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
                println!(
                    "Genesis, Exodus, Leviticus, Numbers, Deuteronomy, Joshua, Judges, Ruth, 1 Samuel, 2 Samuel, 1 Kings, 2 Kings, 1 Chronicles, 2 Chronicles, Ezra, Nehemiah, Esther, Job, Psalms, Proverbs, Ecclesiastes, Song of Solomon, Isaiah, Jeremiah, Lamentations, Ezekiel, Daniel, Hosea, Joel, Amos, Obadiah, Jonah, Micah, Nahum, Habakkuk, Zephaniah, Haggai, Zechariah, Malachi",
                );

                println!(
                    "Matthew, Mark, Luke, John, Acts, Romans, 1 Corinthians, 2 Corinthians, Galatians, Ephesians, Philippians, Colossians, 1 Thessalonians, 2 Thessalonians, 1 Timothy, 2 Timothy, Titus, Philemon, Hebrews, James, 1 Peter, 2 Peter, 1 John, 2 John, 3 John, Jude, Revelations"
                );
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
                        captialize_first_letter(book),
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

                    let result = lookup_kjv1611(book, chapter, verse);

                    match result {
                        Some(verse) => println!("{}", verse),
                        None => println!("Verse not found."),
                    }
                } else if verse_range.len() == 2 {
                    print!(
                        "The book of {}, chapter {}, verses {} through {}:\n",
                        captialize_first_letter(book),
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
                        let result = lookup_kjv1611(book, chapter, verse);

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
