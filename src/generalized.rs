#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::ffi::{c_char, CStr, CString};

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

fn main() {
    unsafe {
        let mut tm = bitwuzla_term_manager_new();

        let mut options = bitwuzla_options_new();

        let cadical_cstr = CString::new("cadical").unwrap();

        bitwuzla_set_option(options, BitwuzlaOption_BITWUZLA_OPT_PRODUCE_MODELS, 1);
        bitwuzla_set_option_mode(
            options,
            BitwuzlaOption_BITWUZLA_OPT_SAT_SOLVER,
            cadical_cstr.as_ptr() as *const i8,
        );
        let mut bitwuzla = bitwuzla_new(tm, options);

        let x_cstr = CString::new("x").unwrap();

        let mut sortbv4 = bitwuzla_mk_bv_sort(tm, 4);
        let mut x = bitwuzla_mk_const(tm, sortbv4, x_cstr.as_ptr() as *const i8);

        bitwuzla_assert(
            bitwuzla,
            bitwuzla_mk_term2(
                tm,
                BitwuzlaKind_BITWUZLA_KIND_EQUAL,
                x,
                bitwuzla_mk_bv_value_uint64(tm, sortbv4, 2),
            ),
        );
        let mut result = bitwuzla_check_sat(bitwuzla);
        println!("Expect: sat\n");

        let res_str = CStr::from_ptr(bitwuzla_result_to_string(result));
        println!("Bitwuzla: {}\n", res_str.to_str().unwrap());
    }
}