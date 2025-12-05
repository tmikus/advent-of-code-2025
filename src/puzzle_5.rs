#[derive(Clone, Debug, PartialEq)]
struct Range {
    from: i64,
    to: i64,
}

impl Range {
    fn new(from: i64, to: i64) -> Self {
        Self { from, to }
    }

    fn contains(&self, item: i64) -> bool {
        item >= self.from && item <= self.to
    }

    fn len(&self) -> i64 {
        self.to - self.from + 1
    }

    fn is_valid(&self) -> bool {
        self.len() >= 1
    }

    fn trim_range_to_not_overlap(self, other: &Self) -> Vec<Self> {
        if self.from < other.from && self.to < other.from {
           return vec![self];
        }
        if self.from > other.to && self.to > other.to {
            return vec![self];
        }
        let mut result = vec![];
        if self.from < other.from {
            result.push(Range::new(self.from, other.from - 1));
        }
        if self.to > other.to {
            result.push(Range::new(other.to + 1, self.to))
        }
        result
    }
}

fn get_input_blocks(input: &str) -> (Vec<&str>, Vec<&str>) {
    let lines = input.lines().collect::<Vec<&str>>();
    let split_position = lines.iter().position(|l| l.trim() == "").unwrap();
    (lines[..split_position].to_vec(), lines[split_position + 1..].to_vec())
}

fn get_number_pairs(input: Vec<&str>) -> Vec<Range> {
    input
        .iter()
        .map(|id_pair| {
            let id_pair = id_pair.trim().split_once("-").unwrap();
            let (id1, id2) = id_pair;
            Range::new(id1.parse::<i64>().unwrap(), id2.parse::<i64>().unwrap())
        })
        .collect()
}

fn get_numbers(input: Vec<&str>) -> Vec<i64> {
    input.iter().map(|id_pair| id_pair.parse::<i64>().unwrap()).collect()
}

fn is_in_any_range(item: i64, ranges: &[Range]) -> bool {
    ranges.iter().any(|range| range.contains(item))
}

pub fn solve_puzzle(input: &str) {
    let (ranges, ingredients) = get_input_blocks(input);
    let ranges = get_number_pairs(ranges);
    let ingredients = get_numbers(ingredients);

    let mut count = 0;
    for ingredient in ingredients {
        if is_in_any_range(ingredient, &ranges) {
            count += 1;
        }
    }
    println!("Part 1: {}", count);

    let mut trimmed_ranges = vec![ranges[0].clone()];

    for range_to_trim in ranges.into_iter().skip(1) {
        let mut intermediate_ranges = vec![range_to_trim];
        for range in trimmed_ranges.iter() {
            intermediate_ranges = intermediate_ranges
                .into_iter()
                .flat_map(|r| r.trim_range_to_not_overlap(range))
                .collect();
        }
        trimmed_ranges.extend(intermediate_ranges);
    }

    let total_len = trimmed_ranges.iter().fold(0, |acc, range| acc + range.len());
    println!("Part 2: {}", total_len);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range_trim_range_to_not_overlap() {
        assert_eq!(
            Range::new(1, 2).trim_range_to_not_overlap(&Range::new(2, 3)),
            vec![Range::new(1, 1)],
        );
        assert_eq!(
            Range::new(1, 5).trim_range_to_not_overlap(&Range::new(2, 3)),
            vec![Range::new(1, 1), Range::new(4, 5)],
        );
    }
}
