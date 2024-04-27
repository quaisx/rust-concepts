pub struct Fibonacci {
    current: u32,
    next: u32,
}

impl Iterator for Fibonacci {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;
        self.current = self.next;
        self.next = current + self.next;
        Some(current)
    }
}

pub fn fibonacci() -> Fibonacci {
    Fibonacci {
        current: 0,
        next: 1,
    }
}

#[cfg(test)]
mod tests {
    use crate::iter::fibonacci::fibonacci;

    #[test]
    fn test_fibonacci_init() {
        let mut fib = fibonacci();
        assert_eq!(fib.next(), Some(0));
    }

    #[test]
    fn test_fibonacci_4() {
        let fib = fibonacci();
        let f: u32 = fib.take(4).map(|x| x).sum();
        assert_eq!(f, 4);
    }

    #[test]
    fn test_fibonacci_skip_4() {
        let fib = fibonacci();
        let f: u32 = fib.skip(4).take(4).map(|x| x).sum();
        // 0, 1, 1, 2, 3, 5, 8, 13
        assert_eq!(f, 29);
    }
}
