// This a unfinished implementation of the well-known merge sort algorithm
//
// 1. Fix the language problems in the function merge
//
// 2. Finish the implementation of the function merge_sort
//
// 3. EXTRA: try changing the type from i32 into String everywhere; does your program still compile? What changes are necessary?

/// Merge two array slices (that have to be sorted) into a vector
fn merge(a: &[String], b: &[String])-> Vec<String> {
    let mut dest = Vec::new();

    let mut a_idx = 0; //change to mut index
    let mut b_idx = 0; //change to mut index

    while a_idx < a.len() && b_idx < b.len() {
        if a[a_idx] <= b[b_idx] {
            dest.push(a[a_idx].clone());
            a_idx += 1
        } else {
            dest.push(b[b_idx].clone());
            b_idx += 1
        }
    }

    for elem in &a[a_idx..] { //fix reference
        dest.push((*elem).clone())
    }
    for elem in &b[b_idx..] {
        dest.push((*elem).clone())
    }

    dest
}

/// Take an array slice, and sort into a freshly constructed vector using the above function
fn merge_sort(data: &[String]) -> Vec<String> {
    if data.len() > 1 {
        // implement this
        let middle = data.len() / 2;
        let left = &data[0..middle];
        let right = &data[middle..];
        let sorted_left = merge_sort(left);
        let sorted_right = merge_sort(right);
        merge(&sorted_left, &sorted_right)
    } else {
        data.to_vec()
    }
}

/// Read a bunch of numbers from standard input into a Vec<i32>.
fn read_numbers() -> Vec<String> {
    use std::io;
    let mut result = Vec::new();
    for line in io::stdin().lines().flatten() {
        for word in line.split_whitespace() {
            result.push(word.parse().unwrap())
        }
    }

    result
}

fn main() {
    //let input = read_numbers();
    let input = vec!["Hello".to_string(), "world".to_string(),"Great".to_string()];

    println!("Data to be sorted:");
    println!("{input:?}");

    let sorted_input = merge_sort(&input);
    println!("Sorted data:");
    println!("{sorted_input:?}");
}

// you can run these automatic tests by typing 'cargo test'
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sort() {
//	assert_eq!(merge_sort(&[]), vec![]);
	assert_eq!(merge_sort(&["test".to_owned()]), vec!["test".to_owned()]);
	assert_eq!(merge_sort(&["test".to_owned(),"TEST".to_owned(),"123".to_owned()]), vec!["123".to_owned(),"test".to_owned(),"TEST".to_owned()]);
    }
}
