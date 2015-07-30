#![feature(no_std,lang_items,core,collections)]
#![no_std]

#[lang="sized"]
trait Sized {}

#[lang = "stack_exhausted"]
extern fn stack_exhausted() { /* ... */ }

#[lang = "eh_personality"]
extern fn eh_personality() { /* ... */ }

#[lang="copy"]
trait Copy {}

extern {
  fn printk(fmt: *const u8, ...);
  fn init_mm_real();
}

#[no_mangle]
pub fn fini_mm() {
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
