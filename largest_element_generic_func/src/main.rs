use std::cmp::PartialOrd;


fn find_largest_element<T: PartialOrd>(list: &[T]) -> Option<&T> {
    if list.is_empty() {
        return None
    }

    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }

    Some(largest)
}


fn main() {

    let numbers = vec![1, 4, 6, 7, 787, 1000];
    let str = vec!["abc", "apple", "tree", "ji"];
    let string = vec![
        String::from("global"),
        String::from("west"),
        String::from("test"),
    ];
    let float = vec![3.14, 1.444, 9.45];

    let empty: Vec<i32> = vec![];

    let number_result = find_largest_element(&numbers);
    let str_result = find_largest_element(&str);
    let string_result = find_largest_element(&string);
    let float_result = find_largest_element(&float);
    let empty_result = find_largest_element(&empty);


    println!("Number result = {:?}", number_result.unwrap_or(&-1));
    println!("str result = {:?}", str_result.unwrap_or(&"Not found"));
    println!("string result = {:?}", string_result.unwrap_or(&String::from("Not found")));
    println!("Float result = {:?}", float_result.unwrap_or(&0.0));
    println!("Empty result = {:?}", empty_result.unwrap_or(&-1));
}
