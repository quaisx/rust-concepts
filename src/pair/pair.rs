pub struct Pair<T>(T, T);

trait Contains {
    type A;

    fn contains(&self, _: &Self::A, _: &Self::A) -> bool;
    fn first(&self) -> Self::A;
    fn second(&self) -> Self::A;
}

impl<T> Contains for Pair<T>
where
    T: PartialEq + Clone + Copy,
{
    type A = T;

    fn contains(&self, x: &T, y: &T) -> bool {
        (&self.0 == x) && (&self.1 == y)
    }

    fn first(&self) -> T {
        self.0
    }

    fn second(&self) -> T {
        self.1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        let p = Pair(1, 2);
        assert_eq!(p.first(), 1);
        assert_eq!(p.second(), 2);
    }

    #[test]
    fn test_contains_pair() {
        let p = Pair(1, 2);
        assert!(p.contains(&1, &2));
    }
}
