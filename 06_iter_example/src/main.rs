use std::collections::HashMap;

fn main() {
    let arr = [0, 1, 2, 3];
    for x in &arr {
        println!("{}", x);
    }

    for x in arr.iter().filter(|x| *x % 2 == 0) {
        println!("{}", x);
    }

    for x in arr.iter().map(|x| *x * 42) {
        println!("{}", x);
    }

    let arr2 = [3, 2, 1, 0];
    let iter2 = arr2.iter().cloned();

    let sum: u32 = arr
        .iter()
        .filter(|&x| x % 2 == 0)
        .cloned()
        .zip(iter2)
        .map(|(a, b)| a * b)
        .sum();

    println!("Sum: {}", sum);

    let joined = arr.iter().chain(&arr2).collect::<Vec<_>>();
    println!("{:?}", joined);

    let map = arr.iter().cloned().zip(arr2.iter().cloned()).collect::<HashMap<_, _>>();

    println!("{:?}", map);

    ultra_generic_iter_str(&["hello", "world"]);
    ultra_generic_iter_str(&["number".to_owned(), "go".to_owned(), "up".to_owned()]);
    ultra_generic_iter_u32(&arr);
    ultra_generic_iter_u32(vec![42, 43, 44, 45, 36, 47]);
}

fn ultra_generic_iter_str<I>(iter: I) where I: IntoIterator, <I as IntoIterator>::Item: AsRef<str> {
    for (i, item) in iter.into_iter().enumerate() {
        println!("Item #{}: {}", i, item.as_ref())
    }
}

fn ultra_generic_iter_u32<I>(iter: I) where I: IntoIterator, <I as IntoIterator>::Item: std::borrow::Borrow<u32> {
    use std::borrow::Borrow;

    let mut sum = 0;
    for (i, item) in iter.into_iter().enumerate() {
        sum += *item.borrow();
        println!("Item #{}: {}; sum: {}", i, item.borrow(), sum);
    }
}
