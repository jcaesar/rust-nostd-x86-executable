#![no_std]
#![no_main]

fn print_str(s: &str) {
    let b = s.as_bytes();
    unsafe {
        libc::write(1, b.as_ptr() as *const libc::c_void, b.len());
    }
}

#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    let b = "##PANIC##\n".as_bytes();
    unsafe {
        libc::write(2, b.as_ptr() as *const libc::c_void, b.len());
        libc::abort();
    }
}

#[no_mangle]
fn main() {
    baz(&A {});
    baz(&B {});
}

trait Hoge {
    fn say(&self);
}
struct A {}
struct B {}

impl Hoge for A {
    fn say(&self) {
        print_str("It's a me, A!\n")
    }
}
impl Hoge for B {
    fn say(&self) {
        print_str("It's a me, B!\n");
    }
}

fn baz(hoge: &dyn Hoge) {
    hoge.say();
}
