pub fn test_for() {
    let s = String::from("abcde");
    for (i, item) in s.chars().into_iter().enumerate() {
        println!("index:{}, item:{}", i, item);
    }
}
