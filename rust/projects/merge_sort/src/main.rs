fn merge_sort(a: &[i32]) -> Vec<i32> {
    if a.len() > 1 {
      let middle = a.len() / 2;
      let left = merge_sort(& a[..middle]);
      let right = merge_sort(& a[middle..]);
      return merge(& left, & right);
    } else {
      return a.to_vec();
    }
  }
  
  
  fn merge<'a>(left: &'a [i32], right: &'a [i32]) -> Vec<i32> {
    let l_len: usize = left.len();
    let r_len: usize = right.len();
    let mut a: Vec<i32> = Vec::with_capacity(l_len + r_len);
    let mut i = 0; // counter for left slice
    let mut j = 0; // counter for right slice
    for _k in 0..(l_len+r_len) {
      if i < left.len() && (j >= right.len() || left[i] <= right[j]) {
        a.push(left[i]);
        i += 1;
      } else if j < right.len() {
        a.push(right[j]);
        j += 1;
      }
    }
    return a;
  }
  
  fn main() {
    let a = vec![5, 2, 7, 3, 4, 7];
    let b = merge_sort(& a);
    println!("Sorted array: {:?}", b);
  }
  