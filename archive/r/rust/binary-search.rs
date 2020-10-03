fn binary_search(search_arr: &Vec<i32>, target: &i32) -> Option<usize> {
  let mut low: i8 = 0;
  let mut high: i8 = search_arr.len() as i8 - 1;

  while low <= high {
    let mid = (((high - low) / 2) + low) as usize;
    let val = &search_arr[mid];

    if val == target {
        return Some(mid);
    }

    // If value is < target then search between mid + 1 and high
    if val < target {
        low = mid as i8 + 1;
    }

    // If value is > target then search between low and mid - 1
    if val > target {
        high = mid as i8 - 1;
    }
  }

  return None;
}


fn main() {
    let arr_1 = vec![0, 3, 6, 8, 9, 10, 12];
    let target_1 = 6;

    if let Some(i) = binary_search(&arr_1, &target_1) {
      println!("Target {} found at index {} in arr {:?}", target_1, i, arr_1);
    }

    let arr_2 = vec![0, 3, 6, 8, 9, 10, 12];
    let target_2 = 11;

    if let None = binary_search(&arr_2, &target_2) {
      println!("Target {} not found in arr {:?}", target_2, arr_2);
    }
}
