#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Direction {
    North,
    East,
    West,
    South,
}

fn dir_reduc(arr: &[Direction]) -> Vec<Direction> {
    let mut res = vec![];
    arr.iter().for_each(|&x| {
        match res.last() {
            Some(&v) if (v as u8 + x as u8) == 3 => { res.pop(); },
            _ => res.push(x),
        }
    });
    res
}

#[cfg(test)]
mod tests {
    use super::{dir_reduc, Direction::{self, *}};

    #[test]
    fn basic() {
        let a = [North, South, South, East, West, North, West];
        assert_eq!(dir_reduc(&a), [West]);

        let a = [North, West, South, East];
        assert_eq!(dir_reduc(&a), [North, West, South, East]);
    }
}