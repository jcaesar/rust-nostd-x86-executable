#![no_std]
#![no_main]

#[no_mangle]
pub extern "C" fn __libc_csu_init() -> () { }

#[no_mangle]
pub extern "C" fn __libc_csu_fini() -> () { }

#[no_mangle]
pub extern "C" fn __libc_start_main() -> () {
    main();
    exit(0);
}

extern "C" {
    fn asm_print(bytes: *const u8, len: usize) -> ();
    fn asm_exit(code: usize) -> !;
}

fn print_str(s: &str) {
    let b = s.as_bytes();
    unsafe {
        asm_print(b.as_ptr(), b.len());
    }
}

fn exit(code: i8) -> ! {
    unsafe {
        asm_exit(code as usize);
    }
}

#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    print_str("##PANIC##\n");
    exit(-1);
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

fn baz(hoge: &dyn Hoge) { hoge.say(); }
