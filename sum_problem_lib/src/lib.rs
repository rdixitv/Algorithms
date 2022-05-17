#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn large_list() {
        let vect = vec![1, 2, 3, 4, 5, 5];
        let result = check(vect, 10);
        assert_eq!(result, vec![5, 5]);
    }

    #[test]
    fn small_list() {
        let vect = vec![100, 3];
        let result = check(vect, 103);
        assert_eq!(result, vec![100, 3]);
    }

    #[test]
    fn does_not_add_up() {
        let vect = vec![1, 2, 2, 0, 10];
        let result = check(vect, 5);
        assert_eq!(result, vec![1]);
    }
}

fn check(vect: Vec<i32>, sum: i32) -> Vec<i32> {
    for i in vect.iter() {
        for j in vect.iter() {
            if i + j == sum {
                return vec![*i, *j];
            }
        }
    }
    vec![1]
}
