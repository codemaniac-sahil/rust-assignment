fn find_first_occurrence(arr: &[i32], target: i32) -> Option<usize> {
    let mut left = 0;
    let mut right = arr.len() - 1;
    let mut result = None;

    while left <= right {
        let mid = left + (right - left) / 2;

        if arr[mid] == target {
            result = Some(mid);
            right = mid - 1;
        } else if arr[mid] < target {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }

    result
}

fn main() {
    let arr = vec![1, 2, 3, 3, 3, 4, 5, 6, 7, 8, 9];
    let target = 3;

    match find_first_occurrence(&arr, target) {
        Some(index) => println!("The first occurrence of {} is at index {}", target, index),
        None => println!("{} is not found in the array", target),
    }
}
