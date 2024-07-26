use std::fs;

fn read_from_file(file_path: &str) -> String {
    let contents = fs::read_to_string(file_path).expect("Something went wrong reading the file");
    return contents;
}

fn main() {
    // Getting the input from the file

    let input_file_path = "/home/ekansh/Desktop/Advent-Of-Code/2023/day2/my_input.txt";
    let input = read_from_file(input_file_path);

    // Defining some structs

    struct Pull {
        red_cubes: i32,
        green_cubes: i32,
        blue_cubes: i32,
    }

    impl Pull {
        fn new(pull_string: String) -> Self {
            let pull_string = pull_string.replace(",", "");

            let mut number_of_cubes_of_an_unknown_color = 0;
            let mut color_for_number_of_unknown_cubes;

            let mut red_cubes = 0;
            let mut green_cubes = 0;
            let mut blue_cubes = 0;

            for part in pull_string.split_whitespace() {
                // Iterating over the parts of the string
                if part.parse::<i32>().is_ok() {
                    // If the part is a number
                    number_of_cubes_of_an_unknown_color = part.parse::<i32>().unwrap();
                    // Converting the part to an integer
                    continue;
                }

                color_for_number_of_unknown_cubes = part.to_string();

                // If the part is a color, then add the number to the respective colors variable

                if color_for_number_of_unknown_cubes == "red" {
                    red_cubes = number_of_cubes_of_an_unknown_color;
                } else if color_for_number_of_unknown_cubes == "green" {
                    green_cubes = number_of_cubes_of_an_unknown_color;
                } else if color_for_number_of_unknown_cubes == "blue" {
                    blue_cubes = number_of_cubes_of_an_unknown_color;
                }
            }

            // Returns a new Pull object
            Self {
                red_cubes,
                green_cubes,
                blue_cubes,
            }
        }
    }

    // and creates a sum variable that stores the answer to this problem
    let mut sum = 0;

    // Iterating over the lines of the input
    for line in input.lines() {
        let mut colon_index: usize = 0;
        for (i, char) in line.chars().enumerate() {
            if char == ':' {
                colon_index = i;
            }
        }

        // Extracting the different pulls from the line

        let line = &line[(colon_index + 2)..]; // To remove the game id and the colon from the line
                                               // so that we just have the data

        let mut pulls: Vec<&str> = vec![]; // This vector contains all the different like pulls of cubes which
                                           // where separated by a semi colon ";"
        let mut last_pull_index = 0;

        for (i, char) in line.chars().enumerate() {
            if char == ';' {
                pulls.push(&line[last_pull_index..(i)]);
                last_pull_index = i + 2; // To remove the semi colon and the space
            }
            if i == line.len() - 1 {
                pulls.push(&line[last_pull_index..(i + 1)]); // To include the last pull correctly
                last_pull_index = i + 2;
            }
        }

        // Converting the pull strings into pull objects which are easier to work with

        let mut pull_objects: Vec<Pull> = vec![];

        for pull in pulls {
            let pull_object = Pull::new(pull.to_string());
            pull_objects.push(pull_object);
        }

        // Now we have a pull_objects vector which contains all the pull objects

        // Find the highest number of each color cubes pulled in the line

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

        let lowest_number_of_red_cubes_in_bag = highest_number_of_red_cubes_pulled;
        let lowest_number_of_green_cubes_in_bag = highest_number_of_green_cubes_pulled;
        let lowest_number_of_blue_cubes_in_bag = highest_number_of_blue_cubes_pulled;

        let power_of_cube_set = lowest_number_of_red_cubes_in_bag
            * lowest_number_of_green_cubes_in_bag
            * lowest_number_of_blue_cubes_in_bag;

        sum += power_of_cube_set;
    }
    println!("{sum}");
}
