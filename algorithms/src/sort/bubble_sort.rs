// O(N^2)
fn bubble_sort<T: PartialOrd + Copy>(source: &mut [T]) {
  let mut flag = true;
  while flag {
    flag = false;
    for i in 1..source.len() {
      if source[i] < source[i - 1] {
        source.swap(i - 1, i);
        flag = true;
      }
    }
  }
}

#[cfg(test)]
mod tests {
    use super::bubble_sort;

  #[test]
  fn bubble_sort_test() {
    let mut a = vec![2, 4, 5, 3, 1];
    bubble_sort(&mut a);
    assert_eq!(a, vec![1, 2, 3, 4, 5]);
  }
}
