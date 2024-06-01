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

    struct Pull {
        red_cubes: i32,
        green_cubes: i32,
        blue_cubes: i32,
    }

    impl Pull {
        fn new(pull_string_input: String) -> Self {
            let pull_string = pull_string_input.replace(",", "");
            let mut num = 0;
            let mut color = String::new();
            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;
            for part in pull_string.split_whitespace() {
                if part.parse::<i32>().is_ok() {
                    num = part.parse::<i32>().unwrap();
                } else {
                    color = part.to_string();
                    if color == "red" {
                        red = num;
                    } else if color == "green" {
                        green = num;
                    } else if color == "blue" {
                        blue = num;
                    }
                }
            }
            Self {
                red_cubes: red,
                green_cubes: green,
                blue_cubes: blue,
            }
        }
    }

    let bag = Bag::new(12, 13, 14);
    let mut sum = 0;

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
        let game_id = i32::from_str(game_id).unwrap();

        // Extracting the pulls from the line

        let line = &line[(colon_ending_index + 2)..];

        let mut pulls: Vec<&str> = vec![]; // This vector contains all the different like pulls of cubes which
                                           // are separated by a semi colon ";"
        let mut last_pull_index = 0;

        for (i, char) in line.chars().enumerate() {
            if char == ';' {
                pulls.push(&line[last_pull_index..(i)]);
                last_pull_index = i + 2;
            }
            if i == line.len() - 1 {
                pulls.push(&line[last_pull_index..(i + 1)]);
                last_pull_index = i + 2;
            }
        }

        let mut pull_objects: Vec<Pull> = vec![];

        for pull in pulls {
            let pull_object = Pull::new(pull.to_string());
            pull_objects.push(pull_object);
        }

        // Now we have a pull_objects vector which contains all the pull objects

        // Find the highest number of cubes pulled in the line

        let mut highest_number_of_red_cubes_pulled = 0;
        let mut highest_number_of_green_cubes_pulled = 0;
        let mut highest_number_of_blue_cubes_pulled = 0;

        for pull in pull_objects {
            if pull.red_cubes > highest_number_of_red_cubes_pulled {
                highest_number_of_red_cubes_pulled = pull.red_cubes;
            }

            if pull.green_cubes > highest_number_of_green_cubes_pulled {
                highest_number_of_green_cubes_pulled = pull.green_cubes;
            }

            if pull.blue_cubes > highest_number_of_blue_cubes_pulled {
                highest_number_of_blue_cubes_pulled = pull.blue_cubes;
            }
        }

        let mut add_to_sum = true;

        if highest_number_of_red_cubes_pulled > bag.red_cubes {
            add_to_sum = false;
        } else if highest_number_of_green_cubes_pulled > bag.green_cubes {
            add_to_sum = false;
        } else if highest_number_of_blue_cubes_pulled > bag.blue_cubes {
            add_to_sum = false;
        }

        if add_to_sum {
            sum += game_id;
        }
    }
    println!("{sum}");
}

fn read_from_file(file_path: &str) -> String {
    let contents = fs::read_to_string(file_path).expect("Something went wrong reading the file");
    return contents;
}
