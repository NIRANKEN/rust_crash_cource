pub fn run() {
    let mut hello = String::from("Hello ");
    hello.push('W');
    hello.push_str("orld!");
    println!("Capacity: {}", hello.capacity());
    println!("Is Empty: {}", hello.is_empty());
    println!("{}", hello);

    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());
    println!("{}", s);

    let person: (&str, &str, i8) = ("NiranKen", "Japan", 29);
    println!("{} is from {} and is {}", person.0, person.1, person.2);

    // mutはmutable宣言で、途中で変更できまっせ宣言
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", numbers);

    numbers[2] = 20;
    println!("{:?}", numbers);
    println!("Single Value: {}", numbers[0]);
    println!("Array occupies bytes: {}", std::mem::size_of_val(&numbers));
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice);

    let mut vec_numbers: Vec<i32> = vec![1, 2, 3, 4];

    vec_numbers[2] = 20;
    vec_numbers.push(5);
    vec_numbers.push(6);
    vec_numbers.pop();
    println!("{:?}", vec_numbers);
    println!("Single Value: {}", vec_numbers[0]);
    println!(
        "Array occupies bytes: {}",
        std::mem::size_of_val(&vec_numbers)
    );
    let slice: &[i32] = &vec_numbers[0..2];
    println!("Slice: {:?}", slice);

    for x in vec_numbers.iter() {
        println!("Number: {}", x);
    }

    for x in vec_numbers.iter_mut() {
        *x *= 2;
    }
    println!("{:?}", vec_numbers);

    let age = 29;
    // let is_of_age = age >= 21 ? true : false ; って書けないんな
    let is_of_age = if age >= 21 { true } else { false };
    println!("Is Of Age: {}", is_of_age);
}
