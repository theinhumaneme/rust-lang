use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}

fn main() {
    let rounds: Vec<_> = read_lines("part2.txt");
    dbg!(rounds
        .iter()
        .map(|line| validate(line.chars().collect::<Vec<_>>()))
        .sum::<i32>());
}

fn validate(chars: Vec<char>) -> i32 {
    let mut score: i32 = 0;
    // match chars[2] {
    //     'X' => {
    //         score += 1;
    //         if chars[0] == 'C' {
    //             score += 6
    //         }
    //         else if chars[0] == 'A'{
    //             score+=3
    //         }
    //     }
    //     'Y' => {
    //         score += 2;
    //         if chars[0] == 'A' {
    //             score += 6
    //         }
    //         else if chars[0] == 'B'{
    //             score+=3
    //         }
    //     }
    //     'Z' => {
    //         score += 3;
    //         if chars[0] == 'B' {
    //             score += 6
    //         }
    //         else if chars[0] == 'C'{
    //             score+=3
    //         }
    //     }
    //     _ => {}
    // }

    match chars[2] {
        'Y' => {
            score += 3;
            if chars[0] == 'A' {
                score += 1
            } else if chars[0] == 'B' {
                score += 2
            } else {
                score += 3
            }
        }
        'X' => {
            score += 0;
            if chars[0] == 'A' {
                score += 3
            } else if chars[0] == 'B' {
                score += 1
            } else {
                score += 2
            }
        }
        'Z' => {
            score += 6;
            if chars[0] == 'A' {
                score += 2
            } else if chars[0] == 'B' {
                score += 3
            } else {
                score += 1
            }
        }
        _ => {}
    }

    dbg!(&score);
    score
}
