use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;
use super::Error;

#[macro_export]
macro_rules! to_slice {
    ($name: expr, $input_type: tt) => {
        unsafe { core::slice::from_raw_parts($name as *const $input_type as *const u8, core::mem::size_of::<$input_type>())}
    };
}

pub trait CoreSeek{
    fn seek_to(&mut self, offset: u64);
}
pub trait CoreRead {
    fn read_bytes(&mut self, buf: &mut [u8]) -> Result<usize, Error>;
    fn read_struct<T: Sized>(&mut self) -> Result<T, Error> {
        let mut buf = vec![0u8; core::mem::size_of::<T>()];
        self.read_bytes(buf.as_mut_slice())?;
        unsafe { Ok((buf.as_ptr() as *const T).read()) }
    }
    fn read_all(&mut self, buf: &mut Vec<u8>) -> Result<usize, Error>;
    fn read_to_string(&mut self, buf: &mut String) -> Result<usize, Error> {
        unsafe { self.read_all(buf.as_mut_vec()) }
    }
}

pub trait CoreWrite {
    fn write_bytes(&mut self, buf: &[u8]) -> Result<usize, Error>;

    fn write_struct<T: Sized>(&mut self, buf: &T) -> Result<usize, Error> {
        let buf = to_slice!(buf, T);
        self.write_bytes(buf)
    }
    fn write_string(&mut self, buf: &String) -> Result<usize, Error> {
        self.write_bytes(buf.as_bytes())
    }
}

impl CoreRead for &[u8] {
    #[inline]
    fn read_bytes(&mut self, buf: &mut [u8]) -> Result<usize, Error> {
        let amt = core::cmp::min(buf.len(), self.len());
        let (a, b) = self.split_at(amt);
        if amt == 1 {
            buf[0] = a[0];
        } else {
            buf[..amt].copy_from_slice(a);
        }

        *self = b;
        Ok(amt)
    }

    fn read_all(&mut self, buf: &mut Vec<u8>) -> Result<usize, Error> {
        buf.extend_from_slice(*self);
        let len = self.len();
        *self = &self[len..];
        Ok(len)
    }
}

impl CoreWrite for &[u8] {
    fn write_bytes(&mut self, buf: &[u8]) -> Result<usize, Error> {
        if buf.len() > self.len() {
            return Err(Error::UnexpectedEof("failed to fill whole buffer".to_string()));
        }
        unsafe {
            core::slice::from_raw_parts_mut(self.as_ptr() as *mut u8, buf.len())
        }.copy_from_slice(buf);
        *self = &self[buf.len()..];
        Ok(buf.len())
    }
}