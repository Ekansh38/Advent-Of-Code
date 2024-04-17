fn main() {
    let input: &str = r#""#;

    let mut sum: i64 = 0;
    let mut nums: Vec<char> = Vec::new();

    for char in input.chars() {
        if char == '\n' {
            let mut val: String = String::new();
            let start = nums[0];
            let end = nums[nums.len() - 1];
            val.push(start);
            val.push(end);
            nums.clear();
            println!("{}", val);
            let val: i64 = val.parse().unwrap();
            sum += val;
        }
        if char.is_digit(10) {
            nums.push(char)
        }
    }
    println!("{}", sum);
}
