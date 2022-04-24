pub(crate) fn quick_sort<T: Ord>(arr: &mut[T]) {
    let len = arr.len();
    _quick_sort(arr, 0, (len-1) as isize);
}

pub fn _quick_sort<T: Ord>(arr: &mut [T], low: isize, high: isize) {
    if low >= high {
        return;
    }
    let pivot = _partition(arr, low, high);
    _quick_sort(arr, low, pivot-1);
    _quick_sort(arr, pivot+1, high);
}

fn _partition<T: Ord>(arr: &mut [T], low: isize, high: isize) -> isize {
    let pivot = high as usize;

    let mut store_index = low -1;
    let mut last_index = high;

    loop {
        store_index += 1;
        while arr[store_index as usize] < arr[pivot] {
            store_index += 1;
        }
        last_index -= 1;
        while last_index > 0 && arr[last_index as usize] > arr[pivot] {
            last_index -=1;
        }
        if store_index >= last_index {
            break;
        }
        arr.swap(store_index as usize, last_index as usize);
    }
    arr.swap(store_index as usize, pivot as usize);
    store_index
}

#[cfg(test)]
mod tests {
    use crate::arrays;

    #[test]
    fn quick_sort() {
        let mut factors= [3, 5] ;
        let expected = [3, 5];
        arrays::quick_sort(&mut factors);
        assert_eq!(expected, factors);
        let mut factors= [5, 3] ;
        arrays::quick_sort(&mut factors);
        assert_eq!(expected, factors);
        let mut factors= [5, 3, 5] ;
        let expected = [3, 5, 5];
        arrays::quick_sort(&mut factors);
        assert_eq!(expected, factors);
        let mut factors= [11, 3, 7, 5, 9] ;
        let expected = [3, 5, 7, 9, 11];
        arrays::quick_sort(&mut factors);
        assert_eq!(expected, factors);
   }
}