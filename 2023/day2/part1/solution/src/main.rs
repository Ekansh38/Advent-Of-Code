use std::{fs, str::FromStr};

fn main() {
    let input = read_from_file("/home/ekansh/Desktop/Advent-Of-Code/2023/day2/my_input.txt");

    struct Bag {
        red_cubes: i32,
        green_cubes: i32,
        blue_cubes: i32,
    }

    impl Bag {
        fn new(red_cubes: i32, green_cubes: i32, blue_cubes: i32) -> Self {
            Self {
                red_cubes,
                green_cubes,
                blue_cubes,
            }
        }
    }

    let bag = Bag::new(12, 13, 14);

    for line in input.lines() {
        // Extracting the game id from the line
        // For loop to find the colon in the line
        let mut colon_ending_index: usize = 0;
        for (index, char) in line.chars().enumerate() {
            if char == ':' {
                colon_ending_index = index;
            }
        }

        let game_id = &line[5..colon_ending_index];

        let line = &line[(colon_ending_index + 2)..];

        let mut pulls: Vec<&str> = vec![]; // This vector contains all the different like pulls of cubes which
                                           // are separated by a semi colon ";"
        let mut last_pull_index = 0;

        for (i, char) in line.chars().enumerate() {
            if char == ';' {
                pulls.push(&line[last_pull_index..(i)]);
                last_pull_index = i + 2;
            }
        }

        for pull in pulls {
            println!("{}", pull);
        }
    }
}

fn read_from_file(file_path: &str) -> String {
    let contents = fs::read_to_string(file_path).expect("Something went wrong reading the file");
    return contents;
}
