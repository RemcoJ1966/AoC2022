use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Input file not found");
    let lines = input.lines();

    println!(
        "The sum of the priorities of common items is {}",
        common_item_priorities(lines.clone())
    );

    println!(
        "The sum of the priorities of badges is {}",
        badge_priorities(lines)
    );
}

fn split_items(line: &str) -> (&str, &str) {
    let split = line.len() / 2;
    (&line[..split], &line[split..])
}

fn common_item(items: (&str, &str)) -> char {
    let common: Vec<char> = items.0.chars().filter(|c| items.1.contains(*c)).collect();
    common[0]
}

fn common_items(items_1: &str, items_2: &str) -> String {
    items_1
        .chars()
        .filter(|c| items_2.contains(*c))
        .collect::<String>()
}

fn badge_type<'a>(lines: impl Iterator<Item = &'a str>) -> char {
    let l: Vec<&str> = lines.collect();
    let common_1 = common_items(l[0], l[1]);
    let common_2 = common_items(&common_1, l[2]);

    let badge: Vec<char> = common_2.chars().collect();
    badge[0]
}

fn item_priority(item: char) -> u32 {
    if item.is_uppercase() {
        u32::from(item) - 38
    } else {
        u32::from(item) - 96
    }
}

fn common_item_priorities<'a>(lines: impl Iterator<Item = &'a str>) -> u32 {
    lines.fold(0, |acc, el| {
        acc + item_priority(common_item(split_items(el)))
    })
}

fn badge_priorities<'a>(lines: impl Iterator<Item = &'a str>) -> u32 {
    const GROUP_SIZE: usize = 3;
    let l: Vec<&str> = lines.collect();
    let mut priorities = 0;

    for i in (0..l.len()).step_by(GROUP_SIZE) {
        priorities += item_priority(badge_type(&mut l[i..i + GROUP_SIZE].iter().cloned()));
    }

    priorities
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prio_lower_a() {
        assert_eq!(1, item_priority('a'))
    }

    #[test]
    fn test_prio_lower_z() {
        assert_eq!(26, item_priority('z'))
    }

    #[test]
    fn test_prio_upper_a() {
        assert_eq!(27, item_priority('A'))
    }

    #[test]
    fn test_prio_upper_z() {
        assert_eq!(52, item_priority('Z'))
    }

    #[test]
    fn test_split_items_1() {
        let contents = "vJrwpWtwJgWrhcsFMMfFFhFp";
        let items = split_items(contents);
        assert_eq!("vJrwpWtwJgWr", items.0);
        assert_eq!("hcsFMMfFFhFp", items.1);
    }

    #[test]
    fn test_common_item_1() {
        let items = ("vJrwpWtwJgWr", "hcsFMMfFFhFp");
        assert_eq!('p', common_item(items));
    }

    #[test]
    fn test_common_items_prios() {
        let contents =
            "vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw";

        assert_eq!(157, common_item_priorities(contents.lines()));
    }

    #[test]
    fn test_badge_type_1() {
        let contents = vec![
            "vJrwpWtwJgWrhcsFMMfFFhFp",
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
            "PmmdzqPrVvPwwTWBwg",
        ];

        assert_eq!('r', badge_type(&mut contents.into_iter()));
    }

    #[test]
    fn test_badge_type_2() {
        let contents = vec![
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
            "ttgJtRGJQctTZtZT",
            "CrZsJsPPZsGzwwsLwLmpwMDw",
        ];

        assert_eq!('Z', badge_type(&mut contents.into_iter()));
    }

    #[test]
    fn test_badge_priorities() {
        let contents =
            "vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw";

        assert_eq!(70, badge_priorities(contents.lines()));
    }
}
