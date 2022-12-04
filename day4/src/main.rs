use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Input file not found");
    let lines = input.lines();

    let contained_count = lines
        .clone()
        .filter(|l| fully_contained(parse_sections(l)))
        .count();
    println!("There are {contained_count} fully overlapping sections");

    let overlap_count = lines.filter(|l| overlap(parse_sections(l))).count();
    println!("There are {overlap_count} overlapping sections");
}

fn parse_sections(sections: &str) -> ((u32, u32), (u32, u32)) {
    let v = sections.split(',').fold(Vec::new(), |mut acc, el| {
        let v: Vec<u32> = el.split('-').map(|s| s.parse::<u32>().unwrap()).collect();
        acc.push((v[0], v[1]));
        acc
    });

    ((v[0].0, v[0].1), (v[1].0, v[1].1))
}

fn fully_contained(sections: ((u32, u32), (u32, u32))) -> bool {
    let (a, b) = sections.0;
    let (c, d) = sections.1;

    (a <= c && b >= d) || (c <= a && d >= b)
}

fn overlap(sections: ((u32, u32), (u32, u32))) -> bool {
    let (a, b) = sections.0;
    let (c, d) = sections.1;

    !((b < c || a > d) || (d < a || c > b))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fully_contained_1() {
        let sections = ((2, 4), (6, 8));
        assert!(!fully_contained(sections));
    }

    #[test]
    fn test_fully_contained_2() {
        let sections = ((2, 3), (4, 5));
        assert!(!fully_contained(sections));
    }

    #[test]
    fn test_fully_contained_3() {
        let sections = ((5, 7), (7, 9));
        assert!(!fully_contained(sections));
    }

    #[test]
    fn test_fully_contained_4() {
        let sections = ((2, 8), (3, 7));
        assert!(fully_contained(sections));
    }

    #[test]
    fn test_fully_contained_5() {
        let sections = ((6, 6), (4, 6));
        assert!(fully_contained(sections));
    }

    #[test]
    fn test_fully_contained_6() {
        let sections = ((2, 6), (4, 8));
        assert!(!fully_contained(sections));
    }
}
