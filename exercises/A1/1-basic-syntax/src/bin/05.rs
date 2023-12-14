fn print_bytes<T: AsRef<[u8]>>(slice: T) {
    let bytes: &[u8] = slice.as_ref();
    for byte in bytes {
      print!("{:02X}", byte);
    }
    println!();
  }

  #[derive(Debug, Clone)]
  struct HasDrop;

impl Drop for HasDrop {
    fn drop(&mut self) {
        println!("Dropping HasDrop!");
    }
}


use std::fmt::Debug; // Trait to bound with.

#[derive(Debug)]
struct Ref<'a, T: 'a>(&'a T);
// `Ref` contains a reference to a generic type `T` that has
// an unknown lifetime `'a`. `T` is bounded such that any
// *references* in `T` must outlive `'a`. Additionally, the lifetime
// of `Ref` may not exceed `'a`.

// A generic function which prints using the `Debug` trait.
fn print<T>(t: T) where
    T: Debug {
    println!("`print`: t is {:?}", t);
}

// Here a reference to `T` is taken where `T` implements
// `Debug` and all *references* in `T` outlive `'a`. In
// addition, `'a` must outlive the function.
fn print_ref<T>(t:& T) where  
    T: Debug  {
    println!("`print_ref`: t is {:?}", t);
}


use std::ops::Index;

use std::slice::SliceIndex;
use std::thread;

struct A<'a, T> {
    slice: &'a [T],
}

impl<'a, T, Idx> Index<Idx> for A<'a, T>
where
    Idx: SliceIndex<[T]>,
{
    type Output = <[T] as Index<Idx>>::Output;

    #[inline(always)]
    fn index(&self, index: Idx) -> &Self::Output {
        self.slice.index(index)
    }
}




  
fn main() {
    let mut data = [22, 12, 13, 17, 18];
    for i in 0..5 {
        data[i] = floored_half(data[i]);
    }
    let owned_bytes: Vec<u8> = vec![0xDE, 0xAD, 0xBE, 0xEF];
  print_bytes(owned_bytes);
  
 // let mut d = HasDrop {};
 // HasDrop::drop(&mut d); not allow explict call destructor
  
  fn get_str<'a>(s1: &'a str) -> &'a str {
    let x = "Hello, world!";
    s1
    
  }

  let byte_slice: [u8; 4] = [0xFE, 0xED, 0xC0, 0xDE];
  print_bytes(byte_slice); //slice is copied over
  print_bytes(byte_slice); //slice is copied over
  get_str("Hello, world!");

  let x = 7;
  let ref_x = Ref(&x);

  print_ref(&ref_x);
  print(ref_x);


  let  slice = &mut [1, 2, 3];

// Then, we iterate over it and increment each element value:
for element in slice.iter_mut() {
    *element += 1;
}


let aa: Vec<u64> = vec![0; 10];
let coefficient_iterable = A { slice: &aa };
assert_eq!(coefficient_iterable[1..3],[0,0] ); // 1..3 => 1 to 2 inclusive 
assert_eq!(coefficient_iterable[1],0 );


let numbers = Vec::from_iter(0..=1000);

let t = thread::spawn(move || {
    let len = numbers.len();
    let sum = numbers.iter().sum::<usize>();
    sum / len
});

let average = t.join().unwrap();

println!("average: {average}");

let numbers = Vec::from_iter(0..=1000); // immutable - reference without moving the ownership into closure

let average = thread::scope(|spawner| {
    spawner.spawn(|| {
        let len = numbers.len();
        let sum = numbers.iter().sum::<usize>();
        sum / len
    }).join().unwrap()
});

println!("average: {average:?}");
println!{"{:?}",numbers}

let p: Vec<u32> = vec![0, 1, 2, 3, 4];

let p2: Vec<u32> = p.iter().map(|x| x * 2).collect();
let sum = p.iter().try_fold(0, |acc: u32, x: &u32| acc.checked_add(*x));


println!("sum: {:?}", sum);

let seed = vec![1, 5, -3];
let res = {
    use Result::{Err as Break, Ok as Next};
    seed.iter()
        .cycle()
        .try_fold(0, |accum, value| match accum + value {
            res if res > 10 => Break(res),
            next => Next(next),
        })
        .unwrap_err()
};

//let res = iter.try_fold(...).unwrap_or_else(|res| res);

println!("{}", res);

}


fn floored_half(data: i32) ->i32 {
    data / 2
}
