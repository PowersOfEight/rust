
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iterator_demonstration() {
        let v1 = vec![1,2,3];

        let mut v1_iter = v1.iter();

        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);
    }

    #[test]
    fn iterator_sum() {
        let v1 = vec![1,2,3];

        let v1_iter = v1.iter();

        let total: i32 = v1_iter.sum();

        assert_eq!(total, 6);
    }

    #[test]
    fn iterator_adapter() {
        let v1 = vec![1,2,3,4,5,6];

        let viter = v1.iter();

        let v1_plus_one: Vec<i32> = viter.map(|x| x + 1).collect();

        for (i, x) in v1.iter().enumerate() {
            assert_eq!(x + 1, v1_plus_one[i]);
        }
    }
}