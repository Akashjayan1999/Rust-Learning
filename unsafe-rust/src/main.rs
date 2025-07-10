use std::slice;
unsafe extern "C"{
   
        fn abs(input:i32)->i32;
    
}

static HELLO_WORLD:&str = "Hello World";
static mut Counter:u32=0;


fn add_to_count(inc:u32){
    unsafe{
        Counter+=inc;
    }
}
fn main() {
   let mut num =5;
    
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    // let address = 0x012345usize;
    // let r3= address as *const i32;

    unsafe{
        println!("r1 is :{}",*r1);
        println!("r2 is :{}",*r2);
    }

    unsafe fn dangerous(){

    }

     unsafe{
        dangerous();
     }


     let mut v =vec![1,2,3,4,5];

     let r = &mut v[..];

     let (a,b) = r.split_at_mut(3);

     assert_eq!(a,&[1,2,3]);
     assert_eq!(b,&[4,5]);

     fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
        // let len = slice.len();

        // assert!(mid <= len);

        // (&mut slice[..mid], &mut slice[mid..]) // return two mutable slices throw error

       let len = slice.len();
      let ptr = slice.as_mut_ptr();
     assert!(mid <= len);
     unsafe {
         (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
   }

   unsafe{
    println!("abs(-5) is {}",abs(5));
   }

   add_to_count(5);

   unsafe{
    let counter_value = Counter;
    println!("Counter is {}",counter_value);
   }

}

#[unsafe(no_mangle)]
 pub  extern "C" fn call_from_c(){
    println!("Called from C");
}


unsafe trait Foo{

}

unsafe impl Foo for i32{

}
