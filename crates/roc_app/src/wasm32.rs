// ⚠️ GENERATED CODE ⚠️ - this entire file was generated by the `roc glue` CLI command

#![allow(unused_unsafe)]
#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(clippy::undocumented_unsafe_blocks)]
#![allow(clippy::redundant_static_lifetimes)]
#![allow(clippy::unused_unit)]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::let_and_return)]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::needless_borrow)]
#![allow(clippy::clone_on_copy)]
#![allow(clippy::non_canonical_partial_ord_impl)]




pub fn mainForHost() -> roc_std::RocStr {
    extern "C" {
        fn roc__mainForHost_1_exposed_generic(_: *mut roc_std::RocStr);
    }

    let mut ret = core::mem::MaybeUninit::uninit();

    unsafe {
        roc__mainForHost_1_exposed_generic(ret.as_mut_ptr(), );

        ret.assume_init()
    }
}