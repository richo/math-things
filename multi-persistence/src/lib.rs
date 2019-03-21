/// Figure out the multiplacitve
pub fn persistence(mut n: usize) -> usize {
    let mut its = 0;
    while n > 10 {
        let mut accum: usize = 1;
        for i in digits(n) {
            accum *= i;
        }
        n = accum;
        its += 1;
    }
    its
}

fn digits(mut n: usize) -> Vec<usize> {
    let mut out = Vec::with_capacity(20);
    while n > 10 {
        out.push(n % 10);
        n /= 10;
    }
    out.push(n);
    out.reverse();
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_digits() {
        assert_eq!(digits(666),
                   vec![6, 6, 6]);
        assert_eq!(digits(1234509876),
                   vec![1, 2, 3, 4, 5, 0, 9, 8, 7, 6]);
    }

    #[test]
    fn test_persistence() {
        assert_eq!(persistence(5428), 2);
        assert_eq!(persistence(327), 2);
        assert_eq!(persistence(277777788888899), 11);

    }
    #[test]
    fn test_lol() {
        assert_eq!(persistence(9999999999999999999), 2);

    }
}
