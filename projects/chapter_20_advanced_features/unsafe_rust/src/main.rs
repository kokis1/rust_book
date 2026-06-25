use std::slice;

// making a simple version of the split_at_mut method (implemented here as a function)
fn split_at_mut<T>(values: &mut [T], mid: usize) -> (&mut [T], &mut [T]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid)
        )
    }
}


// creating a simple Foreign Function Interface (FFI) for a C function (this is always unsafe)
unsafe extern "C" {
    // for this specific case, abs is safe to call so the safe keyword can be used to mark this function as safe
    safe fn abs(input: i32) -> i32;
}

// creating a simple FFI for rust code to be called from other languages (in this case C)
#[unsafe(no_mangle)]
pub extern "C" fn call_from_c() {
    println!("Called a Rust function from C!!");
}

// unsafe function that can only be called from inside an unsafe block
unsafe fn dangerous() {}

// a static variable that can be mutated in an unsafe block
static mut COUNTER: u32 = 0;

// unsafe function that updates the counter
unsafe fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}


// unsafe trait: where at least one of its methods is unsafe
unsafe trait Foo {
    // methods in here
}

unsafe impl Foo for i32 {
    // implementations go in here
}

fn main() {
    // raw pointers:
    // raw pointers can be created in safe rust, just not used
    // there is both a mutable and immutable reference here, not normally allowed

    let mut num = 5;
    let r1 = &raw const num;
    let r2 = &raw mut num;

    // creating a raw pointer that isn't guaranteed to point to valid memory
    let address = 0x012345usize;
    let r = address as *const i32;

    // an unsafe block is needed to dereference raw pointers
    unsafe {
        println!("the pointer r1 is pointing to {}", *r1);
        println!("the pointer r2 is pointing to {}", *r2);
        // println!("address is pointing to: 0x012345usize -> {}", *r); <-- bad address DON'T try and compile
    }

    // unsafe functions can only be called inside unsafe blocks
    unsafe {
        dangerous();
    }

    println!("Absolute value of -4, according to C, is {}", abs(-4));


    // incrementing a mutable static with an unsafe block
    unsafe {
        println!("The counter is at {}", *(&raw const COUNTER));
        add_to_count(5);
        println!("The counter is at {}", *(&raw const COUNTER));
    }


}
