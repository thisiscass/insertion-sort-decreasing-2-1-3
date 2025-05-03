fn main() {
    println!("Hello, world!");
    let mut arr = [5, 2, 4, 6, 1, 3];
    insertion_sort_decreasing(&mut arr);
    println!("{:?}", arr);

}

fn insertion_sort_decreasing(arr: &mut [usize]) {
    let n = arr.len();
    for i in 1..n {
        let key = arr[i];
        
        let mut j = i;
        while j > 0 && arr[j - 1] < key {
            arr[j] = arr[j - 1];
            j = j - 1;
        }

        arr[j] = key;
    }
}

// [5, 2, 4, 6, 1, 3]

//i = 1; j = 0; key = 2
//  j      i
// [2, 5, 4, 6, 1, 3]

//---

//i = 2; j = 1; key = 4
//     j  i
// [2, 4, 5, 6, 1, 3]

//---

//i = 3; j = 3; key = 6
//           ji
// [2, 4, 5, 6, 1, 3]

//---

//i = 3; j = 2; key = 6
//           ji
// [2, 4, 5, 6, 1, 3]

//---

//i = 4; j = 0; key = 1
//  j           i
// [1, 2, 4, 5, 6, 3]

//---

//i = 5; j = 2; key = 3
//        j        i
// [1, 2, 3, 4, 5, 6]