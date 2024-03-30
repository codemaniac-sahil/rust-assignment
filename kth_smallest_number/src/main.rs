fn partition(arr: &mut [i32], left: usize, right: usize) -> usize {
    let pivot_index = right;
    let pivot_value = arr[pivot_index];
    let mut i = left;

    for j in left..right {
        if arr[j] < pivot_value {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, pivot_index);
    i
}

fn kth_smallest_element(arr: &[i32], left: usize, right: usize, k: usize) -> i32 {
    let mut arr = arr.to_vec(); // Convert to mutable vec for partitioning
    let index = partition(&mut arr, left, right);

    if index == k {
        arr[index]
    } else if index < k {
        kth_smallest_element(&arr, index + 1, right, k)
    } else {
        kth_smallest_element(&arr, left, index - 1, k)
    }
}

fn main() {
    let arr = [3, 2, 1, 5, 4];
    let k = 2; // Find the 2nd smallest element

    let result = kth_smallest_element(&arr, 0, arr.len() - 1, k - 1);

    println!("The {}th smallest element is: {}", k, result);
}
