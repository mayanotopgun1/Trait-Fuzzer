#![feature(specialization)]

#[cfg(windows)]
mod imp {
    type LPVOID = *mut u8;
    type DWORD = u32;
    type LPWSTR = *mut u16;

    extern "system" {
        fn FormatMessageW(
            flags: DWORD,
            lpSrc: LPVOID,
            msgId: DWORD,
            langId: DWORD,
            buf: LPWSTR,
            nsize: DWORD,
            args: *const u8,
        ) -> DWORD;
    }

    trait FormatMessage {
        unsafe fn format_message(&self, flags: DWORD, msgId: DWORD, langId: DWORD, buf: &mut [u16], args: *const u8) -> DWORD;
    }

    default impl<T> FormatMessage for T {
        unsafe fn format_message(&self, _flags: DWORD, _msgId: DWORD, _langId: DWORD, _buf: &mut [u16], _args: *const u8) -> DWORD {
            0 // Default implementation returns 0
        }
    }

    impl FormatMessage for () {
        unsafe fn format_message(&self, flags: DWORD, lpSrc: LPVOID, msgId: DWORD, langId: DWORD, buf: &mut [u16], args: *const u8) -> DWORD {
            FormatMessageW(flags, lpSrc, msgId, langId, buf.as_mut_ptr(), buf.len() as u32, args)
        }
    }

    pub fn test() {
        let mut buf: [u16; 50] = [0; 50];
        let ret = unsafe {
            let fmt_msg = FormatMessage::format_message(&(), 0x1000, core::ptr::null_mut(), 1, 0x400, &mut buf, core::ptr::null());
            assert!(ret != 0);
        };
    }
}

#[cfg(not(windows))]
mod imp {
    pub fn test() {}
}

fn main() {
    imp::test()
}