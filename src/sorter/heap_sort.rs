pub fn heap_sort<T: Ord + Copy>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }

    build_maxheap(arr);

    for end in (1..arr.len()).rev() {
        arr.swap(0, end);
        heapify(&mut arr[..end], 0);
    }
}

fn build_maxheap<T: Ord + Copy>(arr: &mut [T]) {
    let last_parent = (arr.len() - 2) / 2;
    for i in (0..=last_parent).rev() {
        heapify(arr, i);
    }
}

fn heapify<T: Ord + Copy>(arr: &mut [T], mut root: usize) {
    let last = arr.len() - 1;
    loop {
        let left = 2 * root + 1;
        if left > last {
            break;
        }
        let right = left + 1;
        let max = if right <= last && arr[right] > arr[left] {
            right
        } else {
            left
        };

        if arr[max] > arr[root] {
            arr.swap(root, max);
        }
        root = max;
    }
}