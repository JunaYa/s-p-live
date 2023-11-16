// 作业 3

fn main() {
    println!("Hello, world!");
    let mut ve1 = vec![5, 0, 3, 1, 1, 2, -1];
    bubble_sort(&mut ve1);
    println!("排序后：{:?}", ve1);
}

fn bubble_sort<T: Ord>(arr: &mut [T]) {
    for i in 0..arr.len() {
        for j in 0..arr.len() - 1 - i {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1)
            }
        }
    }
}
