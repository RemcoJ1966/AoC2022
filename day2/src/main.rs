use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Input file not found");
    let lines = input.lines();
    let moves = lines.map(|l| {
        let mut s = l.split_whitespace();
        (
            s.next().unwrap().chars().next().unwrap(),
            s.next().unwrap().chars().next().unwrap(),
        )
    });

    println!(
        "Total score according to strategy guide 1: {}",
        play_strategy(moves.clone(), play_game_1)
    );

    println!(
        "Total score according to strategy guide 2: {}",
        play_strategy(moves, play_game_2)
    );
}

fn is_win(elf_choice: char, my_choice: char) -> Result<bool, &'static str> {
    match elf_choice {
        'A' => Ok(my_choice == 'Y'),
        'B' => Ok(my_choice == 'Z'),
        'C' => Ok(my_choice == 'X'),
        _ => Err("Unsupported choice by elf"),
    }
}

fn is_draw(elf_choice: char, my_choice: char) -> Result<bool, &'static str> {
    match elf_choice {
        'A' => Ok(my_choice == 'X'),
        'B' => Ok(my_choice == 'Y'),
        'C' => Ok(my_choice == 'Z'),
        _ => Err("Unsupported choice by elf"),
    }
}

fn choice_points(my_choice: char) -> u32 {
    u32::from(my_choice) - 87
}

fn game_points(elf_choice: char, my_choice: char) -> u32 {
    if is_win(elf_choice, my_choice).unwrap() {
        6
    } else if is_draw(elf_choice, my_choice).unwrap() {
        3
    } else {
        0
    }
}

fn must_play(elf_choice: char, my_lot: char) -> Result<char, &'static str> {
    match (elf_choice, my_lot) {
        ('A', 'X') => Ok('Z'), // Rock, Lose => Scissors: Z
        ('A', 'Y') => Ok('X'), // Rock, Draw => Rock: X
        ('A', 'Z') => Ok('Y'), // Rock, Win  => Paper: Y
        ('B', 'X') => Ok('X'), // Paper, Lose => Rock: X
        ('B', 'Y') => Ok('Y'), // Paper, Draw => Paper: Y
        ('B', 'Z') => Ok('Z'), // Paper, Win => Scissors: Z
        ('C', 'X') => Ok('Y'), // Scissors, Lose => Paper: Y
        ('C', 'Y') => Ok('Z'), // Scissors, Draw => Scissors: Z
        ('C', 'Z') => Ok('X'), // Scissors, Win => Rock: X
        _ => Err("Unsupported move"),
    }
}

fn play_game_1(elf_choice: char, my_choice: char) -> u32 {
    game_points(elf_choice, my_choice) + choice_points(my_choice)
}

fn play_game_2(elf_choice: char, my_lot: char) -> u32 {
    let my_choice = must_play(elf_choice, my_lot).unwrap();
    game_points(elf_choice, my_choice) + choice_points(my_choice)
}

fn play_strategy(
    moves: impl Iterator<Item = (char, char)>,
    play_game: fn(char, char) -> u32,
) -> u32 {
    moves.fold(0, |acc, el| acc + play_game(el.0, el.1))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_loss_ax() {
        assert_eq!(Ok(false), is_win('A', 'X'));
    }

    #[test]
    fn test_is_win_ay() {
        assert_eq!(Ok(true), is_win('A', 'Y'));
    }

    #[test]
    fn test_is_loss_az() {
        assert_eq!(Ok(false), is_win('A', 'Z'));
    }

    #[test]
    fn test_is_loss_bx() {
        assert_eq!(Ok(false), is_win('B', 'X'));
    }

    #[test]
    fn test_is_loss_by() {
        assert_eq!(Ok(false), is_win('B', 'Y'));
    }

    #[test]
    fn test_is_win_bz() {
        assert_eq!(Ok(true), is_win('B', 'Z'));
    }

    #[test]
    fn test_is_win_cx() {
        assert_eq!(Ok(true), is_win('C', 'X'));
    }

    #[test]
    fn test_is_loss_cy() {
        assert_eq!(Ok(false), is_win('C', 'Y'));
    }

    #[test]
    fn test_is_loss_cz() {
        assert_eq!(Ok(false), is_win('C', 'Z'));
    }

    #[test]
    fn test_is_draw_ax() {
        assert_eq!(Ok(true), is_draw('A', 'X'));
    }

    #[test]
    fn test_is_nodraw_ay() {
        assert_eq!(Ok(false), is_draw('A', 'Y'));
    }

    #[test]
    fn test_is_nodraw_az() {
        assert_eq!(Ok(false), is_draw('A', 'Z'));
    }

    #[test]
    fn test_is_nodraw_bx() {
        assert_eq!(Ok(false), is_draw('B', 'X'));
    }

    #[test]
    fn test_is_draw_by() {
        assert_eq!(Ok(true), is_draw('B', 'Y'));
    }

    #[test]
    fn test_is_nodraw_bz() {
        assert_eq!(Ok(false), is_draw('B', 'Z'));
    }

    #[test]
    fn test_is_nodraw_cx() {
        assert_eq!(Ok(false), is_draw('C', 'X'));
    }

    #[test]
    fn test_is_nodraw_cy() {
        assert_eq!(Ok(false), is_draw('C', 'Y'));
    }

    #[test]
    fn test_is_draw_cz() {
        assert_eq!(Ok(true), is_draw('C', 'Z'));
    }

    #[test]
    fn test_choice_points_x() {
        assert_eq!(1, choice_points('X'));
    }

    #[test]
    fn test_choice_points_y() {
        assert_eq!(2, choice_points('Y'));
    }

    #[test]
    fn test_choice_points_z() {
        assert_eq!(3, choice_points('Z'));
    }

    #[test]
    fn test_strategy_1() {
        let moves = vec![('A', 'Y'), ('B', 'X'), ('C', 'Z')];
        assert_eq!(15, play_strategy(moves.into_iter(), play_game_1));
    }

    #[test]
    fn test_strategy_2() {
        let moves = vec![('A', 'Y'), ('B', 'X'), ('C', 'Z')];
        assert_eq!(12, play_strategy(moves.into_iter(), play_game_2));
    }
}
