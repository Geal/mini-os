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
    let s = b"PANIC!\0";
    printk(s as *const u8);
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

type unsigned_long = u64;
extern "C" {
  //fn init_mm_real(start: *mut unsigned_long, max: *mut unsigned_long);
  fn init_page_allocator(min: unsigned_long, mac: unsigned_long);

  static mut alloc_bitmap: *const unsigned_long;
  static mut _text: *const u8;
  fn arch_init_mm(start_pfn_p: *mut unsigned_long, max_pfn_p: *mut unsigned_long);
  fn arch_init_p2m(max_pfn: unsigned_long);
  fn arch_init_demand_mapping_area(cur_pfn: unsigned_long);
  fn printk(fmt: *const u8, ...);
}
#[no_mangle]
pub fn fini_mm() {
  //let prompt = CString::new("HELLO FROM RUST").unwrap();
}

// because unsigned long is 8 bytes on x64
#[no_mangle]
pub fn init_mm() {
  unsafe {
    let txt = b"Hello from Rust, world!\n\0";
    printk(txt as *const u8);

    let mut start_pfn: unsigned_long = 0;
    let mut max_pfn:   unsigned_long = 0;

    arch_init_mm(&mut start_pfn, &mut max_pfn);

  /*  let fmt_s = b"MM: Initialise page allocator for %lx(%lx)-%lx(%lx)\n";
    printk(fmt_s as *const u8,
            to_virt(PFN_PHYS(start_pfn) as u32), PFN_PHYS(start_pfn),
            to_virt(PFN_PHYS(max_pfn) as u32), PFN_PHYS(max_pfn));
    */
    
    let txt2 = b"RustMM: done\n\0";
    printk(txt2 as *const u8);
    let s_ph = start_pfn << L1_PAGETABLE_SHIFT;
    let m_ph = max_pfn   << L1_PAGETABLE_SHIFT;
    
    let fmt_s = b"RustMM: Initialise page allocator for %lx-%lx\n";
    printk(fmt_s as *const u8, start_pfn, max_pfn);
    printk(fmt_s as *const u8,
         s_ph, m_ph);
    init_page_allocator(
        s_ph, m_ph);

    arch_init_p2m(max_pfn);
    arch_init_demand_mapping_area(max_pfn);

    let txt3 = b"Good bye from Rust, world!\n\0";
    printk(txt3 as *const u8);
  }
}

const L1_PAGETABLE_SHIFT: u8 = 12;

fn to_virt(p: u64) -> *const u8 {
  unsafe {
    _text.offset(p as isize)
  }
}


fn PFN_PHYS(p: u64) -> u64 {
  p << L1_PAGETABLE_SHIFT
}

/*
fn init_page_allocator(min: u32, mac: u32) {

}*/

