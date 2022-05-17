fn reverse(mut num: u32) -> u32 {
    let mut rev: u32 = 0;
    while num != 0 {
        rev = rev * 10 + num % 10;
        num = num / 10;
    }
    rev
}

fn check_if_palindrome(num: u32) -> bool {
    let reversed = reverse(num);
    if num == reversed {
        true
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn reverse_test1() {
        let num: u32 = 1234;
        assert_eq!(reverse(num), 4321);
    }
    #[test]
    fn reverse_test2() {
        let num: u32 = 123321;
        assert_eq!(reverse(num), 123321);
    }
    #[test]
    fn reverse_test3() {
        let num: u32 = 12334245;
        assert_eq!(reverse(num), 54243321);
    }


    #[test]
    fn check_if_palindrome_test1() {
        let num = 1234321;
        assert_eq!(check_if_palindrome(num), true);
    }
    #[test]
    fn check_if_palindrome_test2() {
        let num = 123321;
        assert_eq!(check_if_palindrome(num), true);
    }
    #[test]
    fn check_if_palindrome_test3() {
        let num = 123421;
        assert_eq!(check_if_palindrome(num), false);
    }
    #[test]
    fn check_if_palindrome_test4() {
        let num = 1;
        assert_eq!(check_if_palindrome(num), true);
    }
}
