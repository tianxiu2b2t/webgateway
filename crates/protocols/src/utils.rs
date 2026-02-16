#[inline]
pub fn find_index_pos<T: PartialEq>(arr: &[T], target: &[T]) -> Option<usize> {
    let arr_len = arr.len();
    let target_len = target.len();

    if arr_len < target_len {
        return None;
    }

    (0..=arr_len - target_len).find(|&i| &(arr[i..i + target_len]) == target)
}
