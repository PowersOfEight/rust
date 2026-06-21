// Unsafe: this is just wild
/**
* # Safety: don't try this at home kids
*/
pub unsafe fn dangerous() {}

use std::slice;

// In rust, global variables are called `static` variables
//
static HELLO_WORLD: &str = "Hello, World!";

// We do make these with the 'static  lifetime, and so mutating these is inherently unsafe
static mut COUNTER: u32 = 0;

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

// SAFETY: calling this from more than a single thread at a time is undefined
// behavior, so you *must* guarantee you only call it from a single thread at
// a time
unsafe fn add_to_count(increment: u32) {
    unsafe {
        COUNTER += increment;
    }
}

/**
* Call the absolute value function from the Foreign Function Interface in C
*/
unsafe extern "C" {
    fn abs(input: i32) -> i32;
    // could mark this as safe since it has no safety preconditions
    // safe fn abs(input: i32) -> i32
    // if you mark an extern function as safe, you can use it outside of the unsafe block
}

// This is how you externalize rust functions to C
#[unsafe(no_mangle)]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}

unsafe trait Foo {
    // a trait is unsafe when at least one of its methods has some invariant the
    // compileer can't verify
}

unsafe impl Foo for i32 {
    // method implementations go here
}

#[cfg(test)]
mod tests {
    use crate::COUNTER;
    use crate::HELLO_WORLD;
    use crate::abs;
    use crate::add_to_count;
    use crate::dangerous;

    #[test]
    fn unsafe_20_1() {
        // we create a mutable num
        let mut num = 5;

        // we can create a raw pointer with the &raw syntax
        // pointers can be either const or mut
        // Note that we do not need an unsafe block to create the raw pointer
        let _raw_const_pointer = &raw const num;
        let _raw_mut_pointer = &raw mut num;

        // we cannot mutate the address of the raw const pointer;
        // let m = 7;
        // _raw_const_pointer = &raw const m;
        // observe the diagnostics below:
        // Diagnostics:
        // 1. cannot mutate immutable variable `_raw_const_pointer` [E0384]
        // 2. cannot assign twice to immutable variable `_raw_const_pointer`
        //    cannot assign twice to immutable variable [E0384]
        //    lib.rs:12:13: first assignment to `_raw_const_pointer`
        //    lib.rs:12:13: consider making this binding mutable: `mut `

        // now lets see if we can get away with this using the mut pointer
        // let mut n = 7;
        // _raw_mut_pointer = &raw mut m;
        //
        // Diagnostics:
        // 1. cannot assign twice to immutable variable `_raw_mut_pointer`
        //    cannot assign twice to immutable variable [E0384]
        //    lib.rs:13:13: first assignment to `_raw_mut_pointer`
        //    lib.rs:13:13: consider making this binding mutable: `mut `
        // 2. first assignment to `_raw_mut_pointer` [E0384]
        //    lib.rs:28:9: original diagnostic
        //
        // If we want t change the pointer address later we need to mut the left
        // side of the pattern
        let mut _n = 7;
        let mut _raw_mut_mut_pointer = &raw mut _n;

        let mut _m = 11;
        _raw_mut_mut_pointer = &raw mut _m;
        unsafe {
            // dereferencing can only take place in an unsafe block
            // we dereference the pointer the same way we do in C and other languages
            // by prefixing the pointer with the * operator
            assert!(*_raw_mut_pointer == 5);

            // let's try and make use of the const-ness
            assert!(*_raw_mut_mut_pointer == 11);

            // lets do another thing
            *_raw_mut_pointer = 13;
            assert!(*_raw_const_pointer == 13);
        }
    }

    #[test]
    fn unsafe_20_2() {
        // The book says there's usually no good reason to write code where
        // we give it a raw address
        // However, in the microcontroller world, this is precisely what we need
        // to do, so it's important to know how to do it
        let random_address = 0x012345usize;
        let _r = random_address as *const i32;

        assert_eq!(0x012345usize, random_address);
    }

    #[test]
    fn unsafe_20_3() {
        let mut num = 5;

        let r1 = &raw const num;
        let r2 = &raw mut num;

        unsafe {
            println!("r1 is: {}", *r1);
            println!("r2 is: {}", *r2);
        }
    }

    #[test]
    fn unsafe_fn() {
        unsafe {
            dangerous();
        }
    }

    #[test]
    fn split_at_mut_is_safe() {
        todo!();
    }

    #[test]
    fn call_c_abs_extern_works() {
        unsafe {
            println!("Absolute value of -3 according to C: {}", abs(-3));
        }
    }

    #[test]
    fn read_static_variable() {
        println!("value is: {HELLO_WORLD}"); // reading a global variable totally safe
    }

    #[test]
    fn mutate_static_variable_is_unsafe() {
        unsafe {
            add_to_count(3);
            println!("COUNTER: {}", *(&raw const COUNTER));
        }
    }

    #[test]
    fn quiz_question_2() {
        let mut v: Vec<u32> = Vec::with_capacity(4);
        for i in 0..3 {
            v.push(i);
        }
        let n = &raw const v[0];
        v.push(4);
        println!("{}", unsafe { *n });
    }
}
