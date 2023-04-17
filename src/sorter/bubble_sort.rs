pub fn bubble_sort<T: Ord + Copy>(arr: &mut [T]) {
    let mut new_len: usize = arr.len();
    let mut len: usize = arr.len();

    while new_len != 0 {
        new_len = 0;
        for i in 1..len {
            if arr[i - 1] > arr[i] {
                arr.swap(i - 1, i);
                new_len = i;
            }
        }
        len = new_len;
    }
}