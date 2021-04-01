#![no_std]
#![no_main]
#![feature(unsize)]
#![feature(coerce_unsized)]

#[no_mangle]
pub extern "C" fn __libc_csu_init() -> () {}

#[no_mangle]
pub extern "C" fn __libc_csu_fini() -> () {}

#[no_mangle]
pub extern "C" fn __libc_start_main() -> () {
    main();
    exit(0);
}

#[no_mangle]
pub extern "C" fn memset(mut dst: *mut u8, val: i32, len: usize) {
    unsafe {
        let end = dst.offset(len as isize);
        while dst != end {
            *dst.as_mut().unwrap() = val as u8;
            dst = dst.offset(1);
        }
    }
}

#[no_mangle]
pub extern "C" fn memcpy(dst: *mut u8, src: *const u8, len: usize) {
    let mut offset: usize = 0;
    while offset < len {
        unsafe {
            *dst.offset(offset as isize).as_mut().unwrap() = *src.offset(offset as isize).as_ref().unwrap()
        }
        offset += 1;
    }
}

extern "C" {
    fn sys_write(fd: usize, bytes: *const u8, len: usize) -> isize;
    fn sys_exit(code: usize) -> !;
}

fn print_str(s: &str) {
    let b = s.as_bytes();
    unsafe {
        sys_write(1, b.as_ptr(), b.len());
    }
}

fn exit(code: i8) -> ! {
    unsafe {
        sys_exit(code as usize);
    }
}

#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    print_str("##PANIC##\n");
    exit(-1);
}

#[no_mangle]
fn main() {
    let pool = Pool {
        buffer: core::cell::UnsafeCell::new([0; 1000]),
        used_to: atomic::AtomicUsize::new(0),
    };
    loop {
        baz(pool.hold_my(A {}));
    }
}

trait Hoge {
    fn say(&self);
}
struct A {}
impl Drop for A {
    fn drop(&mut self) {
        print_str("It was a me, Aaaaa...\n");
    }
}

impl Hoge for A {
    fn say(&self) {
        print_str("It's a me, A!\n")
    }
}

impl<T: Hoge + ?Sized> Hoge for Owned<T> {
    fn say(&self) {
        (&*self as &T).say();
    }
}

fn baz(hoge: Owned<dyn Hoge>) {
    hoge.say();
}

use core::sync::atomic;

struct Pool<const OOL: usize> {
    buffer: core::cell::UnsafeCell<[u8; OOL]>,
    used_to: atomic::AtomicUsize,
}

impl<const OOL: usize> Pool<OOL> {
    fn hold_my<T>(&self, t: T) -> Owned<T> {
        let align = core::mem::align_of::<T>();
        let size = core::mem::size_of::<T>() + align;
        let ptr = self.used_to.fetch_add(size, atomic::Ordering::SeqCst);
        if ptr + size > OOL {
            panic!("OOM");
        }
        let start = self.buffer.get() as *mut u8;
        let ptr = ptr + (align - (start as usize + ptr - 1) % align);
        let ptr = unsafe {
            let ptr = start.offset(ptr as isize) as *mut T;
            ptr.write(t);
            ptr
        };
        Owned {
            inner: ptr,
            p: core::marker::PhantomData,
        }
    }
}

struct Owned<T: ?Sized> {
    inner: *mut T,
    p: core::marker::PhantomData<T>, // something about drop and cycle checks... see Box/Unique for an explanation
}

impl<T: ?Sized> core::ops::Deref for Owned<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.inner.as_ref().unwrap() }
    }
}

impl<T: ?Sized> Drop for Owned<T> {
    fn drop(&mut self) {
        unsafe {
            core::mem::drop(self.inner.as_mut().unwrap());
        }
    }
}

impl<T, U> core::ops::CoerceUnsized<Owned<U>> for Owned<T> where T: core::marker::Unsize<U>, U: ?Sized {
}
