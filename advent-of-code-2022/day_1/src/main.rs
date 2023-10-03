use std::fs::read_to_string;
use std::fs::File;
use std::path::Path;

fn main() {
    let input = Path::new("input.txt");
    let path = input.display();
    let mut elf_calories: Vec<i32> = vec![];
    let mut file = match File::open(&input) {
        Err(why) => panic!("couldn't open the file {}", path),
        Ok(file) => file,
    };
    let elf: Vec<_> = read_lines("input.txt");
    let calories: Vec<_> = elf.split(|s| s == "").collect();
    for elf in calories {
        let total = elf
            .iter()
            .map(|val| val.to_owned().parse::<i32>().unwrap())
            .sum::<i32>();
        elf_calories.push(total);
    }
    elf_calories.sort();
    elf_calories.reverse();
    print!("{}", elf_calories[0] + elf_calories[1] + elf_calories[2]);
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}
