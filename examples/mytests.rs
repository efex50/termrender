

#[cfg(test)]
mod t{
    use std::{alloc::GlobalAlloc, mem};

trait Speak {
    fn speak(&self);
}

#[derive(Debug, Default)]
struct Dog {
    a: usize,
}

impl Speak for Dog {
    fn speak(&self) {
        println!("woof! a = {}", self.a);
    }
}

// oha amk vtable
// teşekkürler chatgbt
#[test]
fn dump_and_call_vtable() {
    let dog = Dog { a: 2020 };
    let dyn_ref: &dyn Speak = &dog;

    // Split fat pointer
    let (data, vtable): (*const Dog, *const *const ()) =
        unsafe { mem::transmute(dyn_ref) };

    println!("data_ptr   = {data:p}");
    println!("vtable_ptr = {vtable:p}\n");

    unsafe {
        // Iterate over first 8 entries of the vtable
        for i in 0..8 {
            let entry = *vtable.add(i);
            println!("vtable[{i}] = {entry:p}");

            // Try to call any entry beyond index 2 (since 0=drop, 1=size, 2=align)
            if i == 3 {
                println!("→ attempting call on slot {i}");
                let func: fn(&Dog) = mem::transmute(entry);
                func(&*data);
                func(&Dog { a: 1 });
                func(&Dog { a: 2 });
                func(&Dog { a: 3 });
            }
        }
    }
}


}