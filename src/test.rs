
fn _array_flipping<T>(array: Vec<T>) -> Vec<T> {
    array.into_iter().rev().collect::<Vec<T>>()
}

#[test] 
fn test_flip() {
    let array1 = vec![1,2,3];
    assert_eq!(_array_flipping(array1), vec![3,2,1]);

    let array2 = vec!["H", "a", "l", "l", "o"];
    assert_eq!(_array_flipping(array2), vec!["o", "l", "l", "a", "H"]);
}

