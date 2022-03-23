// O(N^2)だが、ある程度ソートされている配列に対してはほぼO(N)で動作する
fn insertion_sort<T: PartialOrd + Copy>(source: &mut [T]) {
  for i in 1..source.len() {
    let v = source[i];
    let mut  j = i;
    while j > 0 && source[j - 1] > v {
        source[j] = source[j - 1];
        j -= 1;
    }
    source[j] = v;
  }
}

#[cfg(test)]
mod tests {
    use std::vec;
    use crate::sort::{insertion_sort::insertion_sort};

  #[test]
  fn insertion_sort_test() {
    let mut a = vec![2, 4, 5, 3, 1];
    insertion_sort(&mut a);
    assert_eq!(a, vec![1, 2, 3, 4, 5]);
  }
}
