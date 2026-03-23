use std::ops::{Range, RangeInclusive};

fn parse_range_string(range_as_string: &str) -> (u64, u64) {
    let r: Vec<&str> = range_as_string.split("-").collect();
    let start = r[0]
        .to_string()
        .parse::<u64>()
        .expect("Should be an integer");
    let end = r[1]
        .to_string()
        .parse::<u64>()
        .expect("Should be an integer");
    (start, end)
}

pub fn parse_range(range_as_string: &str) -> Range<u64> {
    let range = parse_range_string(range_as_string);

    range.0..range.1
}
pub fn parse_range_inclusive(range_as_string: &str) -> RangeInclusive<u64> {
    let range = parse_range_string(range_as_string);

    range.0..=range.1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn range_is_parsed_correctly() {
        assert_eq!(11..22, parse_range("11-22"));
        assert_eq!(9393918461..9393960770, parse_range("9393918461-9393960770"));
    }

    #[test]
    fn range_inclusive_is_parsed_correctly() {
        assert_eq!(11..=22, parse_range_inclusive("11-22"));
        assert_eq!(
            9393918461..=9393960770,
            parse_range_inclusive("9393918461-=9393960770")
        );
    }
}
