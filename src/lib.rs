#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn foo() {
        unsafe {
            bd_lvm_check_deps();
        }
    }

    #[test]
    fn list_pvs() {
        unsafe {
            let dep_check = bd_lvm_check_deps();
            assert_ne!(dep_check, 0);
            let init_check = bd_lvm_init();
            assert_ne!(init_check, 0);

            bd_lvm_pvs(std::ptr::null_mut());

            bd_lvm_close();
        }
    }
}
