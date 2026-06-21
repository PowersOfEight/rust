#[cfg(test)]
mod tests {

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
}
