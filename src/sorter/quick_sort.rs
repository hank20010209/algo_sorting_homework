pub fn quick_sort<T: Ord + Copy>(arr: &mut [T]) {
    let right: isize = arr.len() as isize - 1;
    quick_sort_helper(arr, 0, right);
    
}

pub fn quick_sort_helper<T: Ord + Copy>(arr: &mut [T], left: isize, right: isize) {
    if left <= right {
        let pivot: isize = partition(arr, left, right);
        quick_sort_helper(arr, left, pivot - 1);
        quick_sort_helper(arr, pivot + 1, right);
    }
}

pub fn partition<T: Ord + Copy>(arr: &mut [T], left: isize, right: isize) -> isize{
    let pivot = arr[right as usize];
    let mut i = left;

    for j in i..right {
        if arr[j as usize] < pivot {
            arr.swap(i as usize, j as usize);
            i += 1;
        }
    }
    arr.swap(i as usize, right as usize);
    i
}
