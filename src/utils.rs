use lazy_static::lazy_static;
use regex::Regex;

pub fn time_stamp_converter(text: &str) -> i64 {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(\d+)").unwrap();
    }
    let regex_result = RE.captures(text);

    if regex_result.is_none() {
        return 0;
    }
    let match_values = regex_result.unwrap();

    let time_stamp: i64 = match_values.get(1).map_or("", |m| m.as_str()).parse().unwrap_or(0) / 1000;
    time_stamp
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_time_stamp_converter() {
        let good_test_string = "/Date(1654228500000-0700)/";
        assert_eq!(time_stamp_converter(good_test_string), 1654228500);

        let bad_test_string = "null";
        assert_eq!(time_stamp_converter(bad_test_string), 0);
    }
}
