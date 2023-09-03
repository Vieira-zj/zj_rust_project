use std::time::Duration;

pub fn is_number(s: &str) -> bool {
    s.parse::<i64>().is_ok()
}

pub fn parse_secs_duration(arg: &str) -> Result<Duration, std::num::ParseIntError> {
    if is_number(arg) {
        return Ok(Duration::from_secs(arg.parse()?));
    }

    let mut input = arg;
    if input.ends_with("s") {
        input = &arg[..arg.len() - 1];
    }
    Ok(Duration::from_secs(input.parse()?))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_number() {
        assert!(is_number("101"));
        assert!(!is_number("101s"));
    }

    #[test]
    fn test_parse_secs_duration() {
        assert!(parse_secs_duration("101").is_ok());
        assert!(parse_secs_duration("101s").is_ok());
        assert!(parse_secs_duration("101x").is_err());
    }
}
