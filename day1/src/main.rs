use std::{fs, str::Lines};

fn main() {
    let input = fs::read_to_string("input.txt").expect("Input file not found");
    let lines = input.lines();

    let most_calories = find_most_calories(lines.clone());
    println!(
        "The elf carrying the most calories carries {}",
        most_calories
    );

    let top_3 = find_top_3(lines);
    println!("The top 3 elves carry {}", top_3);
}

fn find_most_calories(lines: Lines) -> i32 {
    let mut max_calories = 0;
    let mut sum_calories = 0;

    for line in lines {
        if line.is_empty() {
            if sum_calories > max_calories {
                max_calories = sum_calories;
            }
            sum_calories = 0;
        } else {
            let calories = line.parse::<i32>();

            sum_calories += match calories {
                Ok(x) => x,
                _ => 0,
            }
        }
    }

    if sum_calories > max_calories {
        max_calories = sum_calories;
    }

    max_calories
}

fn find_top_3(lines: Lines) -> i32 {
    let mut calories_carried = Vec::new();
    let mut sum_calories = 0;

    for line in lines {
        if line.is_empty() {
            calories_carried.push(sum_calories);
            sum_calories = 0;
        } else {
            let calories = line.parse::<i32>();

            sum_calories += match calories {
                Ok(x) => x,
                _ => 0,
            }
        }
    }

    calories_carried.push(sum_calories);

    calories_carried.sort_unstable();
    calories_carried.reverse();
    calories_carried.iter().take(3).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn most_calories_1() {
        let input = "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000";
        assert_eq!(24000, find_most_calories(input.lines()));
    }

    #[test]
    fn top3_1() {
        let input = "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000";
        assert_eq!(45000, find_top_3(input.lines()));
    }
}
