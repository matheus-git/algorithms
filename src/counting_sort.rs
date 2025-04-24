pub fn counting_sort(arr: &mut [usize]) {
    if arr.is_empty() {
        return;
    }

    let max = *arr.iter().max().unwrap();
    let mut count = vec![0; max + 1];

    for &num in arr.iter() {
        count[num] += 1;
    }

    let mut index = 0;
    for (num, &freq) in count.iter().enumerate() {
        for _ in 0..freq {
            arr[index] = num;
            index += 1;
        }
    }
}
