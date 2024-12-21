use lazy_static::lazy_static;

lazy_static! {
  static ref EXAMPLE: u8 = 42;
}

fn main() {
  println!("Hello, world!");
}

// Expanded code:
//
// struct EXAMPLE {
//     __private_field: (),
// }
// #[doc(hidden)]
// #[allow(non_upper_case_globals)]
// static EXAMPLE: EXAMPLE = EXAMPLE { __private_field: () };
// impl ::lazy_static::__Deref for EXAMPLE {
//     type Target = u8;
//     fn deref(&self) -> &u8 {
//         #[inline(always)]
//         fn __static_ref_initialize() -> u8 {
//             42
//         }
//         #[inline(always)]
//         fn __stability() -> &'static u8 {
//             static LAZY: ::lazy_static::lazy::Lazy<u8> = ::lazy_static::lazy::Lazy::INIT;
//             LAZY.get(__static_ref_initialize)
//         }
//         __stability()
//     }
// }
// impl ::lazy_static::LazyStatic for EXAMPLE {
//     fn initialize(lazy: &Self) {
//         let _ = &**lazy;
//     }
// }
