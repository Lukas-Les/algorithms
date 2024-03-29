pub fn bubble_sort(arr: &mut[i32]) {
    let len = arr.len();
    for i in 0..len {
        for j in 0..len - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
    println!("{:?}", arr);
}

pub fn optimised_bubble_sort(arr: &mut[i32]) {
    let len = arr.len();
    for i in 0..len {
        let mut swaped = false;
        for j in 0..len - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
                swaped = true;
            }
        }
        if !swaped {
            break;
        }
    }
    println!("{:?}", arr);
}
