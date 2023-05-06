fn merge_sort(arr: &mut [i32]) {
    let mid = arr.len() / 2;
    if mid == 0 {
        return;
    }

    merge_sort(&mut arr[..mid]);
    merge_sort(&mut arr[mid..]);

    let mut ret = arr.to_vec();
    merge(&arr[..mid], &arr[mid..], &mut ret[..]);
    arr.copy_from_slice(&ret);
}

fn merge(left: &[i32], right: &[i32], merged: &mut [i32]) {
    let (mut left_idx, mut right_idx, mut merged_idx) = (0, 0, 0);

    while left_idx < left.len() && right_idx < right.len() {
        if left[left_idx] <= right[right_idx] {
            merged[merged_idx] = left[left_idx];
            left_idx += 1;
        } else {
            merged[merged_idx] = right[right_idx];
            right_idx += 1;
        }
        merged_idx += 1;
    }

    if left_idx < left.len() {
        merged[merged_idx..].copy_from_slice(&left[left_idx..]);
    }
    if right_idx < right.len() {
        merged[merged_idx..].copy_from_slice(&right[right_idx..]);
    }
}
