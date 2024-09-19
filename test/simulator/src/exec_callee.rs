extern crate alloc;

mod entry {
    use core::result::Result;
    use crate::error::Error;

    pub fn main() -> Result<(), Error> {
        let argv = ckb_std::env::argv();
        ckb_std::debug!("argv: {:?}", argv);
        assert_eq!(argv.len(), 2);
        assert_eq!(argv[0].to_bytes(), b"Hello World");
        assert_eq!(argv[1].to_bytes(), "你好".as_bytes());
        Ok(())
    }
}

pub mod error {
    use ckb_std::error::SysError;
    /// Error
    #[repr(i8)]
    pub enum Error {
        IndexOutOfBound = 1,
        ItemMissing,
        LengthNotEnough,
        Encoding,
        WaitFailure,
        InvalidFd,
        OtherEndClosed,
        MaxVmsSpawned,
        MaxFdsCreated,
        // Add customized errors here...
    }

    impl From<SysError> for Error {
        fn from(err: SysError) -> Self {
            use SysError::*;
            match err {
                IndexOutOfBound => Self::IndexOutOfBound,
                ItemMissing => Self::ItemMissing,
                LengthNotEnough(_) => Self::LengthNotEnough,
                Encoding => Self::Encoding,
                WaitFailure => Self::WaitFailure,
                InvalidFd => Self::InvalidFd,
                OtherEndClosed => Self::OtherEndClosed,
                MaxVmsSpawned => Self::MaxVmsSpawned,
                MaxFdsCreated => Self::MaxFdsCreated,
                Unknown(err_code) => panic!("unexpected sys error {}", err_code),
            }
        }
    }
}

use std::env;
use std::ffi::CString;
use std::os::unix::ffi::OsStringExt;

fn main() {
    println!("START simulator callee");
    let args = env::args_os()
        .into_iter()
        .map(|arg| CString::new(arg.into_vec()).unwrap())
        .collect::<Vec<_>>()
        .leak();
    let argv = args
        .iter()
        .map(|cstring| (&**cstring).into())
        .collect::<Vec<_>>()
        .leak();
    println!("START simulator callee entry");
    unsafe { ckb_std::env::set_argv(argv) };
    let code = entry::main()
        .map(|()| 0i32)
        .unwrap_or_else(|err| err as i32);
    if code != 0 {
        println!("exit with {}", code);
    } else {
        println!("simulator callee run success");
    }
    std::process::exit(code);
}
