use rand::RngExt;

use std::io::{Write, stdin, stdout};

fn read(input: &mut String) {
    stdout().flush().expect("failed to flush");
    stdin().read_line(input).expect("failed to read");
}

pub fn marvel_rivals_random_hero() {
    loop {
        println!("MARVEL RIVAL RANDOM HERO");
        println!();
        println!("Enter # to choose role: ");
        println!("  1. Vanguard");
        println!("  2. Duelist");
        println!("  3. Strategist");
        println!("  4. All Heros");
        println!("  5. Vanguard/Duelist");
        println!("  6. Vanguard/Strategist");
        println!("  7. Duelist/Strategist");
        println!();
        print!("selection> ");

        let mut input = String::new();

        read(&mut input);

        let pool_selector = input.trim();

        let vanguard: Vec<&str> = vec![
            "Angela",
            "Bruce Banner",
            "Captain America",
            "Vanguard-Deadpool",
            "Devil Dinosaur",
            "Doctor Strange",
            "Emma Frost",
            "Groot",
            "Magneto",
            "Peni Parker",
            "Rouge",
            "The Hood",
            "The Thing",
            "Thor",
            "Venom",
        ];
        let duelist: Vec<&str> = vec![
            "Black Cat",
            "Black Panther",
            "Black Widow",
            "Blade",
            "Cyclops",
            "Daredevil",
            "Duelist-Deadpool",
            "Elsa",
            "Gorr",
            "Hawkeye",
            "Hela",
            "Human Torch",
            "Iron Fist",
            "Iron Man",
            "Magik",
            "Mister Fantastic",
            "Moon Knight",
            "Namor",
            "Phoenix",
            "Psylocke",
            "Scarlet Witch",
            "Spider-Man",
            "Squirrel Girl",
            "Star-Lord",
            "Storm",
            "The Punisher",
            "Winter Soldier",
            "Wolverine",
        ];
        let strategist: Vec<&str> = vec![
            "Adam Warlock",
            "Cloak & Dagger",
            "Strategist-Deadpool",
            "Gambit",
            "Invisible Woman",
            "Jeff The Land Shark",
            "Jubilee",
            "Loki",
            "Luna Snow",
            "Mantis",
            "Rocket Raccoon",
            "Ultron",
            "White Fox",
        ];

        let mut all_heroes: Vec<&str> = Vec::new();
        let mut vanguard_duelist: Vec<&str> = Vec::new();
        let mut vanguard_strategist: Vec<&str> = Vec::new();
        let mut duelist_strategist: Vec<&str> = Vec::new();

        for hero in vanguard.iter() {
            all_heroes.push(hero);
            vanguard_duelist.push(hero);
            vanguard_strategist.push(hero);
        }

        for hero in duelist.iter() {
            all_heroes.push(hero);
            vanguard_duelist.push(hero);
            duelist_strategist.push(hero);
        }

        for hero in strategist.iter() {
            all_heroes.push(hero);
            vanguard_strategist.push(hero);
            duelist_strategist.push(hero);
        }

        //let heroes: usize = all_heroes.len();

        let hero_pool: Vec<Vec<&str>> = vec![
            vanguard,
            duelist,
            strategist,
            all_heroes,
            vanguard_duelist,
            vanguard_strategist,
            duelist_strategist,
        ];

        let pool_index: usize = match pool_selector {
            "1" => 0,
            "2" => 1,
            "3" => 2,
            "4" => 3,
            "5" => 4,
            "6" => 5,
            "7" => 6,

            "exit" => break,

            _ => {
                println!("Invalid selection.");
                continue;
            }
        };

        let selected_pool = &hero_pool[pool_index];

        let mut rng = rand::rng();

        let value: usize = rng.random_range(0..selected_pool.len());

        let hero = selected_pool[value];

        println!("Random Hero: {hero}");
    }
}
