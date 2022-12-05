use std::fs;

type MoveFnT = fn(Vec<Vec<char>>, usize, usize, usize) -> Vec<Vec<char>>;

fn main() {
    let stacks = vec![
        vec!['N', 'S', 'D', 'C', 'V', 'Q', 'T'],
        vec!['V', 'F', 'M'],
        vec!['F', 'Q', 'W', 'D', 'P', 'N', 'H', 'M'],
        vec!['D', 'Q', 'R', 'T', 'F'],
        vec!['R', 'F', 'M', 'N', 'Q', 'H', 'V', 'B'],
        vec!['C', 'F', 'G', 'N', 'P', 'W', 'Q'],
        vec!['W', 'F', 'R', 'L', 'C', 'T'],
        vec!['T', 'Z', 'N', 'S'],
        vec!['M', 'S', 'D', 'J', 'R', 'Q', 'H', 'N'],
    ];

    let input = fs::read_to_string("input.txt").expect("Input file not found");
    let lines = input.lines().skip_while(|l| !l.starts_with("move"));

    let message9000 = top_of_rearranged_crates(lines.clone(), stacks.clone(), move_crates);
    println!("The message with crane 9000 is {message9000}");

    let message9001 = top_of_rearranged_crates(lines, stacks, move_crates_at_once);
    println!("The message with crane 9001 is {message9001}");
}

fn top_of_rearranged_crates<'a>(
    instructions: impl Iterator<Item = &'a str>,
    mut stacks: Vec<Vec<char>>,
    move_fn: MoveFnT,
) -> String {
    for instr in instructions {
        let segments = parse_instruction(instr);
        stacks = move_fn(stacks, segments.0, segments.1, segments.2);
    }

    top_of_stacks(stacks)
}

fn parse_instruction(instruction: &str) -> (usize, usize, usize) {
    let segments: Vec<&str> = instruction.split(' ').into_iter().collect();
    (
        segments[1].parse::<usize>().unwrap(),
        segments[3].parse::<usize>().unwrap(),
        segments[5].parse::<usize>().unwrap(),
    )
}

fn top_of_stacks(stacks: Vec<Vec<char>>) -> String {
    stacks.iter().fold(String::new(), |mut acc, stack| {
        acc.push(*stack.last().unwrap());
        acc
    })
}

fn move_crate(mut stacks: Vec<Vec<char>>, from: usize, to: usize) -> Vec<Vec<char>> {
    let moved_crate = stacks[from - 1].pop().unwrap();
    stacks[to - 1].push(moved_crate);
    stacks
}

fn move_crates(
    mut stacks: Vec<Vec<char>>,
    number: usize,
    from: usize,
    to: usize,
) -> Vec<Vec<char>> {
    for _ in 0..number {
        stacks = move_crate(stacks, from, to);
    }
    stacks
}

fn move_crates_at_once(
    mut stacks: Vec<Vec<char>>,
    number: usize,
    from: usize,
    to: usize,
) -> Vec<Vec<char>> {
    let at = stacks[from - 1].len() - number;
    let mut moved_crates = stacks[from - 1].split_off(at);
    stacks[to - 1].append(&mut moved_crates);
    stacks
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_top_of_stacks() {
        let stacks = vec![vec!['C'], vec!['M'], vec!['P', 'D', 'N', 'Z']];
        assert_eq!("CMZ", top_of_stacks(stacks));
    }

    #[test]
    fn test_move_crate() {
        let stacks = vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']];
        assert_eq!(
            vec![vec!['Z', 'N', 'D'], vec!['M', 'C'], vec!['P']],
            move_crate(stacks, 2, 1)
        );
    }

    #[test]
    fn test_move_crates() {
        let stacks = vec![vec!['Z', 'N', 'D'], vec!['M', 'C'], vec!['P']];
        assert_eq!(
            vec![vec![], vec!['M', 'C'], vec!['P', 'D', 'N', 'Z']],
            move_crates(stacks, 3, 1, 3)
        );
    }

    #[test]
    fn test_move_crates_at_once() {
        let stacks = vec![vec!['Z', 'N', 'D'], vec!['M', 'C'], vec!['P']];
        assert_eq!(
            vec![vec![], vec!['M', 'C'], vec!['P', 'Z', 'N', 'D']],
            move_crates_at_once(stacks, 3, 1, 3)
        );
    }
}
