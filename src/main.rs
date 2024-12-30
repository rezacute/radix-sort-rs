fn get_max(arr: &[i32]) -> i32 {
    *arr.iter().max().unwrap()
}

fn counting_sort(arr: &mut Vec<i32>, exp: i32) {
    let n = arr.len();
    let mut output = vec![0; n];
    let mut count = vec![0; 10];

    // Count the occurrences of each digit
    for &num in arr.iter() {
        let index = (num / exp) % 10;
        count[index as usize] += 1;
    }

    // Update count to store actual position
    for i in 1..10 {
        count[i] += count[i - 1];
    }

    // Build the output array
    for i in (0..n).rev() {
        let index = (arr[i] / exp) % 10;
        output[count[index as usize] as usize - 1] = arr[i];
        count[index as usize] -= 1;
    }

    // Copy output array to arr
    arr.clone_from_slice(&output);
}

fn radix_sort(arr: &mut Vec<i32>) {
    let max_val = get_max(arr);
    let mut exp = 1;

    while max_val / exp > 0 {
        counting_sort(arr, exp);
        exp *= 10;
    }
}

fn main() {
    let mut arr = vec![170, 45, 75, 90, 802, 24, 2, 66];
    println!("Original array: {:?}", arr);

    radix_sort(&mut arr);

    println!("Sorted array: {:?}", arr);
}
