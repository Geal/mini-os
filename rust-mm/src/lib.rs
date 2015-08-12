#![feature(no_std,lang_items,core,collections)]
#![no_std]

extern crate core;
extern crate collections;

mod std {
#[macro_use]
  pub use core::{fmt, iter, option, ops, slice, mem};
  pub use collections::{boxed, vec, string};
  pub mod prelude {
    pub use core::prelude as v1;
  }
}

#[lang = "panic_fmt"] #[inline(never)] #[cold]
pub extern fn panic_impl(msg: ::core::fmt::Arguments,
    file: &'static str,
    line: usize) -> !
{
  unsafe {
    printk(file.as_bytes().as_ptr());
    //use io::Write;
    //let _ = write!(terminal::get_terminal(), "{}:{} {}", file, line, msg);
    ::core::intrinsics::abort();
  }
}

#[lang = "stack_exhausted"]
extern fn stack_exhausted() { /* ... */ }

#[lang = "eh_personality"]
extern fn eh_personality() { /* ... */ }

/*
#[lang="sized"]
trait Sized {}

#[lang="copy"]
trait Copy {}
*/

extern {
  fn printk(fmt: *const u8, ...);
  fn init_mm_real();
}

#[no_mangle]
pub fn fini_mm() {
  //let prompt = CString::new("HELLO FROM RUST").unwrap();
}

#[no_mangle]
pub fn init_mm() {
  unsafe {
    let txt = b"Hello from Rust, world!\n\0";
    printk(txt as *const u8);

    init_mm_real();

    let txt2 = b"Good bye from Rust, world!\n\0";
    printk(txt2 as *const u8);
  }
}
