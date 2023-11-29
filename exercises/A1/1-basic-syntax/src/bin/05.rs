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
}


fn floored_half(data: i32) ->i32 {
    data / 2
}
