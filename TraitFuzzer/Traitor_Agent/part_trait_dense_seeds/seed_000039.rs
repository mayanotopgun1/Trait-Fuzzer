#![feature(rustc_private)]
#![no_main]

extern crate libc;

trait SigAction {
    unsafe fn get_action(sig: libc::c_int) -> libc::sigaction;
}

impl SigAction for () {
    unsafe fn get_action(sig: libc::c_int) -> libc::sigaction {
        let mut actual: libc::sigaction = core::mem::zeroed();
        libc::sigaction(sig, core::ptr::null(), &mut actual);
        actual
    }
}

#[no_mangle]
extern "C" fn main(argc: core::ffi::c_int, argv: *const *const u8) -> core::ffi::c_int {
    assert_eq!(argc, 2, "Must pass SIG_IGN or SIG_DFL as first arg");
    let arg1 = unsafe { core::ffi::CStr::from_ptr(*argv.offset(1) as *const libc::c_char) }
        .to_str()
        .unwrap();

    let expected = match arg1 {
        "SIG_IGN" => libc::SIG_IGN,
        "SIG_DFL" => libc::SIG_DFL,
        arg => panic!("Must pass SIG_IGN or SIG_DFL as first arg. Got: {}", arg),
    };

    let actual = unsafe { <()>::get_action(libc::SIGPIPE) }.sa_sigaction;

    assert_eq!(actual, expected, "actual and expected SIGPIPE disposition in child differs");

    0
}