#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reverse_s() {
        let s = "Hello, World!";
        let result = reverse_string(&s);
        assert_eq!(result, "!dlroW ,olleH");
    }

    #[test]
    fn reverse_n() {
        let num = 1233414;
        let result = reverse_int(num);
        assert_eq!(result, 4143321)
    }

    #[test]
    fn palindrome_str() {
        let s = "test";
        let result = check_if_palindrome_str(&s);
        assert_eq!(result, false);
    }
    #[test]
    fn palindrome_str2() {
        let s = "racecar";
        let result = check_if_palindrome_str(&s);
        assert_eq!(result, true);
    }
    #[test]
    fn palindrome_int() {
        let n = 1837;
        let result = check_if_palindrome_int(n);
        assert_eq!(result, false);
    }
    #[test]
    fn palindrome_int2() {
        let n = 1;
        let result = check_if_palindrome_int(n);
        assert_eq!(result, true);
    }
    #[test]
    fn palindrome_int3() {
        let n = 123454321;
        let result = check_if_palindrome_int(n);
        assert_eq!(result, true);
    }
    #[test]
    fn palindrome_int4() {
        let n = 12344321;
        let result = check_if_palindrome_int(n);
        assert_eq!(result, true);
    }
}

pub fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}

pub fn check_if_palindrome_str(s: &str) -> bool {
    if s == reverse_string(&s) {
        return true;
    }
    false
}

pub fn reverse_int(mut n: u32) -> u32 {
    let mut rev = 0;
    while n != 0 {
        rev = rev * 10 + n % 10;
        n /= 10;
    }
    rev
}

pub fn check_if_palindrome_int(n: u32) -> bool {
    if n == reverse_int(n) {
        return true;
    }
    false
}


