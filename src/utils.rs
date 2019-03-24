pub fn insertion_sort<T: PartialOrd>(list: &mut [T]) {
    let n = list.len();
    for i in 0..n {
        let mut j = i;
        while j > 0 && list[j - 1] > list[j] {
            list.swap(j - 1, j);
            j -= 1;
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_insertion_sort_1() {
        let mut l = vec![3, 2, 5, 1];
        insertion_sort(&mut l);
        assert_eq!(l[0], 1);
        assert_eq!(l[1], 2);
        assert_eq!(l[2], 3);
        assert_eq!(l[3], 5);
    }

    #[test]
    fn test_insertion_sort_2() {
        let mut l = vec![1];
        insertion_sort(&mut l);
        assert_eq!(l[0], 1);

        let mut l = vec![1.2, 100.3, 20.2];
        insertion_sort(&mut l);
        assert_eq!(l[0], 1.2);
        assert_eq!(l[1], 20.2);
        assert_eq!(l[2], 100.3);
    }
}