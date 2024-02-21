use std::thread;

fn even_odd_sort(arr: &mut [i32]) {
    let mut sorted = false;
    while !sorted {
        sorted = true;
        let mut handles = vec![];

        // Sorting even indices
        let arr_clone_even = arr.to_vec();
        let handle_even = thread::spawn(move || {
            let mut arr = arr_clone_even;
            let mut sort_flag = true;
            for i in (0..arr.len() - 2).step_by(2) {
                if arr[i] > arr[i + 2] {
                    arr.swap(i, i + 2);
                    sort_flag = false;
                }
            }
            (arr, sort_flag, true)
        });
        handles.push(handle_even);

        // Sorting odd indices
        let arr_clone_odd = arr.to_vec();
        let handle_odd = thread::spawn(move || {
            let mut arr = arr_clone_odd;
            let mut sort_flag = true;
            for i in (1..arr.len() - 2).step_by(2) {
                if arr[i] > arr[i + 2] {
                    arr.swap(i, i + 2);
                    sort_flag = false;
                }
            }
            (arr, sort_flag, false)
        });
        handles.push(handle_odd);

        // Join threads and update array
        let mut even_sorted = vec![];
        let mut odd_sorted = vec![];
        for handle in handles {
            let (arr_result, sorted_local, is_even) = handle.join().unwrap();
            if is_even {
                even_sorted = arr_result;
            } else {
                odd_sorted = arr_result;
            }
            sorted = sorted && sorted_local;
        }

        // println!("{:?}", even_sorted);
        // println!("{:?} {:?}", odd_sorted, sorted);

        // merge the two
        for i in 0..even_sorted.len()/2 + 1 {
            arr[i * 2] = even_sorted[i * 2];
            if i*2+1 < even_sorted.len() - 1 {
                arr[i * 2 + 1] = odd_sorted[i * 2 + 1];
            }
        }

        // offset
        for i in (1..arr.len() - 1).step_by(2) {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
                sorted = false;
            }
        }
        // println!("{:?}", arr);
        for i in (0..arr.len() - 1).step_by(2) {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
                sorted = false;
            }
        }
    }
}

fn main() {
    let mut numbers = vec![9, 3, 6, 2, 8, 5, 1, 4, 7];
    println!("Unsorted: {:?}", numbers);
    even_odd_sort(&mut numbers);
    println!("Sorted: {:?}", numbers);
}
