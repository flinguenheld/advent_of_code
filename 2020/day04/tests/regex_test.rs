#[cfg(test)]
mod tests {

    use day04::constants::*;
    use regex::Regex;

    #[test]
    fn test_byr() {
        let byr = Regex::new(BYR).unwrap();
        for date in -100..3000 {
            match (1920..=2002).contains(&date) {
                true => assert!(byr.is_match(&date.to_string())),
                false => assert!(!byr.is_match(&date.to_string())),
            }
        }
        assert!(!byr.is_match("2000a"));
        assert!(!byr.is_match("20a00"));
        assert!(!byr.is_match("a2000"));
    }
    #[test]
    fn test_iyr() {
        let iyr = Regex::new(IYR).unwrap();
        for date in -100..3000 {
            match (2010..=2020).contains(&date) {
                true => assert!(iyr.is_match(&date.to_string())),
                false => assert!(!iyr.is_match(&date.to_string())),
            }
        }
        assert!(!iyr.is_match("2000a"));
        assert!(!iyr.is_match("20a00"));
        assert!(!iyr.is_match("a2000"));
    }
    #[test]
    fn test_eyr() {
        let eyr = Regex::new(EYR).unwrap();
        for date in -100..3000 {
            match (2020..=2030).contains(&date) {
                true => assert!(eyr.is_match(&date.to_string())),
                false => assert!(!eyr.is_match(&date.to_string())),
            }
        }
        assert!(!eyr.is_match("2021a"));
        assert!(!eyr.is_match("20a21"));
        assert!(!eyr.is_match("a2021"));
    }
    #[test]
    fn test_hgt() {
        let hgt = Regex::new(HGT).unwrap();
        for value in -100..2000 {
            match (150..=193).contains(&value) {
                true => assert!(hgt.is_match(&format!("{}cm", value))),
                false => assert!(!hgt.is_match(&format!("{}cm", value))),
            }
        }
        for value in 0..2000 {
            match (59..=76).contains(&value) {
                true => assert!(hgt.is_match(&format!("{}in", value))),
                false => assert!(!hgt.is_match(&format!("{}in", value))),
            }
        }
        assert!(!hgt.is_match("180cma"));
        assert!(!hgt.is_match("a180cm"));
        assert!(!hgt.is_match("59ina"));
        assert!(!hgt.is_match("a60in"));
    }
    #[test]
    fn test_hcl() {
        let hgt = Regex::new(HCL).unwrap();
        assert!(hgt.is_match("#000000"));
        assert!(!hgt.is_match("#0000000"));
        assert!(!hgt.is_match("#00000"));
        assert!(hgt.is_match("#aaaaaa"));
        assert!(!hgt.is_match("#aaaaaaa"));
        assert!(!hgt.is_match("#aaaaa"));
        assert!(hgt.is_match("#AAAAAA"));
        assert!(!hgt.is_match("#AAAAAAA"));
        assert!(!hgt.is_match("#AAAAA"));

        assert!(hgt.is_match("#a0a5a6"));
        assert!(!hgt.is_match("#Z0a5a6"));
        assert!(!hgt.is_match("#a0a5ag"));
    }
    #[test]
    fn test_ecl() {
        let ecl = Regex::new(ECL).unwrap();
        assert!(ecl.is_match("amb"));
        assert!(ecl.is_match("blu"));
        assert!(ecl.is_match("brn"));
        assert!(ecl.is_match("gry"));
        assert!(ecl.is_match("grn"));
        assert!(ecl.is_match("hzl"));
        assert!(ecl.is_match("oth"));
        assert!(!ecl.is_match("aamba"));
        assert!(!ecl.is_match("a"));
    }

    #[test]
    fn test_pid() {
        let pid = Regex::new(PID).unwrap();
        assert!(pid.is_match("000000000"));
        assert!(pid.is_match("012345678"));
        assert!(pid.is_match("000000001"));
        assert!(!pid.is_match("0123"));
        assert!(!pid.is_match("01235654665465"));
    }
}
