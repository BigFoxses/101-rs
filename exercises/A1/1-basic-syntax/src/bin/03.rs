fn largest(i: &[u8]) -> u8{

    let mut l=i[0];
    for j in 0..i.len() {
      if i[j] > l {
        l=i[j]
      }
    }
    l
}
fn smallest(i: &[u8]) -> u8 {
    let mut s: u8 = i[0];
    for j in 0..i.len() {
        if i[j] < s {
            s=i[j]

        }   
    
    }
    s
}


fn main() {
    let input = [23, 82, 16, 45, 21, 94, 12, 34];

    //get the largest element from the input array

    println!("{} is largest and {} is smallest", largest(&input),smallest(&input));
}
