// 09实现一个函数，为u32类型的整数集合求和，参数类型为 &[u32]，返回类型为Option，溢出时返回None
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    match sum_u32_collection(&numbers) {
        Some(result) => println!("Sum: {}", result),
        None => println!("Overflow occurred!"),
    }
}

fn sum_u32_collection(numbers: &[u32]) -> Option<u32> {
    numbers.iter().fold(Some(0), |acc, &num| {
        acc.and_then(|sum| sum.checked_add(num))
    })
}
