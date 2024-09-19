#![no_std]
#![no_main]

use ckb_std;
use core::ffi::CStr;

ckb_std::default_alloc!();
ckb_std::entry!(program_entry);

fn program_entry() -> i8 {
    match main() {
        Ok(_) => 0,
        Err(err) => err as i8,
    }
}

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
}

impl From<ckb_std::error::SysError> for Error {
    fn from(err: ckb_std::error::SysError) -> Self {
        match err {
            ckb_std::error::SysError::IndexOutOfBound => Self::IndexOutOfBound,
            ckb_std::error::SysError::ItemMissing => Self::ItemMissing,
            ckb_std::error::SysError::LengthNotEnough(_) => Self::LengthNotEnough,
            ckb_std::error::SysError::Encoding => Self::Encoding,
            ckb_std::error::SysError::WaitFailure => Self::WaitFailure,
            ckb_std::error::SysError::InvalidFd => Self::InvalidFd,
            ckb_std::error::SysError::OtherEndClosed => Self::OtherEndClosed,
            ckb_std::error::SysError::MaxVmsSpawned => Self::MaxVmsSpawned,
            ckb_std::error::SysError::MaxFdsCreated => Self::MaxFdsCreated,
            ckb_std::error::SysError::Unknown(err_code) => {
                panic!("unexpected sys error {}", err_code)
            }
        }
    }
}

pub fn main() -> Result<(), Error> {
    let argc: u64 = 2;
    let argv = [
        CStr::from_bytes_with_nul(b"hello\0").unwrap().as_ptr(),
        CStr::from_bytes_with_nul(b"world\0").unwrap().as_ptr(),
    ];
    let mut std_fds: [u64; 2] = [0, 0];
    let mut son_fds: [u64; 3] = [0, 0, 0];
    let (r0, w0) = ckb_std::syscalls::pipe()?;
    std_fds[0] = r0;
    son_fds[1] = w0;
    let (r1, w1) = ckb_std::syscalls::pipe()?;
    std_fds[1] = w1;
    son_fds[0] = r1;
    let mut pid: u64 = 0;
    let mut spgs = ckb_std::syscalls::SpawnArgs {
        argc: argc,
        argv: argv.as_ptr(),
        process_id: &mut pid as *mut u64,
        inherited_fds: son_fds.as_ptr(),
    };
    ckb_std::syscalls::spawn(1, ckb_std::ckb_constants::Source::CellDep, 0, 0, &mut spgs)?;
    let mut buf: [u8; 256] = [0; 256];
    let len = ckb_std::syscalls::read(std_fds[0], &mut buf)?;
    assert_eq!(len, 10);
    buf[len] = 0;
    assert_eq!(
        CStr::from_bytes_until_nul(&buf).unwrap().to_str().unwrap(),
        "helloworld"
    );
    Ok(())
}