fn selection_sort<T: PartialOrd + Copy>(source: &mut [T]) {
  for i in 0..source.len() {
    // i以降の未整列部分のうち、最小のインデックスを特定する
    let mut minj = i;
    for j in i..source.len() {
      if source[j] < source[minj] {
        minj = j;
      }
    }
    // 最小のものとiの要素を交換して、整列部分を更新する
    source.swap(i, minj);
  }
}

#[cfg(test)]
mod tests {
    use std::vec;
    use crate::sort::selection_sort::selection_sort;

  #[test]
  fn selection_sort_test() {
    let mut a = vec![2, 4, 5, 3, 1];
    selection_sort(&mut a);
    assert_eq!(a, vec![1, 2, 3, 4, 5]);
  }
}
