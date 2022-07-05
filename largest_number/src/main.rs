// The parameter `list` represents any concrete slice of `i32` values.
fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];

    // Pattern matching and destructuring each `&i32` so that the `for` loop
    // gets so that `item` will be an `i32` inside the loop body.
    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = largest(&number_list);
    println!("The largest number is {}", result);
}

