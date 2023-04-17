pub fn selection_sort<T: Ord + Copy>(arr: &mut [T]) {
    let len = arr.len();
    for i in 0..len {
        let mut temp = i;
        for j in (i + 1)..len {
            if arr[temp] > arr[j] {
                temp = j;
            }
        }
        arr.swap(i, temp);
    }
}