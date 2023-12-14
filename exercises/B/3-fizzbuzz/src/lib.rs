/// Very naive implementation of FizzBuzz
pub fn fizz_buzz(i: u32) -> String {
    if i % 3 == 0 {
        if i % 5 == 0 {
            "FizzBuzz".to_owned()
        } else {
            "Fizz".to_owned()
        }
    } else if i % 5 == 0 {
        "Buzz".to_owned()
    } else {
        format!("{i}")
    }
}

// TODO Write a unit test, using the contents of `fizzbuzz.out` file
// to compare.
// You can use the `include_str!()` macro to include file
// contents as `&str` in your artifact.
#[test]
fn test_fizz_buzz() {
    let me= include_str!("/Users/raymond/Downloads/Rust/101-rs/exercises/B/3-fizzbuzz/fizzbuzz.out");
    let len = me.lines().count(); // convert to line interators and count number of iterators -> number of lines
    let mut i=0;
    while i< len{
        println!("i= {} {}", i, me.lines().nth(i).unwrap());
        assert_eq!(fizz_buzz((i+1) as u32), me.lines().nth(i).unwrap()); // extract me counted from 0  while fizz_buzz starts from 1        i += 1;

        i += 1;
    }
    



}

