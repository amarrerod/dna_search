

fn partition<T>(data: &mut[T], left: isize, right: isize, pivot: T) -> isize where T:  std::cmp::PartialOrd, T: Copy {
    let mut left_idx = left;
    let mut right_idx = right - 1;
    loop {
        while data[left_idx as usize ] < pivot {
            left_idx += 1;
        }
        while right_idx > 0 && data[right_idx as usize] > pivot {
            right_idx -= 1;
        }
        if left_idx >= right_idx {
            break;
        }
        else {
            data.swap(left_idx as usize, right_idx as usize);
            
        }
    }
    data.swap(left_idx as usize, right as usize);
    left_idx

}

pub fn quicksort<T>(data: &mut[T], left: isize, right: isize) where T:  std::cmp::PartialOrd, T: Copy {
    if right - left <= 0 {
        return;
    }
    else {
        let pivot = data[right as usize];
        let part_idx = partition(data, left, right, pivot);
        quicksort(data, left, part_idx - 1);
        quicksort(data, part_idx +1, right);
    }
}