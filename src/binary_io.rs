use std::io;

pub fn read_be_u16(slice: &[u8]) -> u16 {
    let mut bytes = [0u8; 2];
    bytes.copy_from_slice(slice);
    u16::from_be_bytes(bytes)
}

pub fn read_be_u32(slice: &[u8]) -> u32 {
    let mut bytes = [0u8; 4];
    bytes.copy_from_slice(slice);
    u32::from_be_bytes(bytes)
}

pub fn read_be_u64(slice: &[u8]) -> u64 {
    let mut bytes = [0u8; 8];
    bytes.copy_from_slice(slice);
    u64::from_be_bytes(bytes)
}

pub fn read_be_i64(slice: &[u8]) -> i64 {
    let mut bytes = [0u8; 8];
    bytes.copy_from_slice(slice);
    i64::from_be_bytes(bytes)
}

pub fn read_be_f64(slice: &[u8]) -> f64 {
    let mut bytes = [0u8; 8];
    bytes.copy_from_slice(slice);
    f64::from_be_bytes(bytes)
}

pub trait WriteExt {
    fn write_u8(&mut self, value: u8) -> io::Result<()>;
    fn write_u16(&mut self, value: u16) -> io::Result<()>;
    fn write_u32(&mut self, value: u32) -> io::Result<()>;
    fn write_u64(&mut self, value: u64) -> io::Result<()>;
    fn write_i64(&mut self, value: i64) -> io::Result<()>;
    fn write_f64(&mut self, value: f64) -> io::Result<()>;
    fn write_string_len16(&mut self, value: &str) -> io::Result<()>;
    fn write_string_len32(&mut self, value: &str) -> io::Result<()>;
}

impl<W: io::Write> WriteExt for W {
    fn write_u8(&mut self, value: u8) -> io::Result<()> {
        self.write_all(&[value])
    }

    fn write_u16(&mut self, value: u16) -> io::Result<()> {
        self.write_all(&value.to_be_bytes())
    }

    fn write_u32(&mut self, value: u32) -> io::Result<()> {
        self.write_all(&value.to_be_bytes())
    }

    fn write_u64(&mut self, value: u64) -> io::Result<()> {
        self.write_all(&value.to_be_bytes())
    }

    fn write_i64(&mut self, value: i64) -> io::Result<()> {
        self.write_all(&value.to_be_bytes())
    }

    fn write_f64(&mut self, value: f64) -> io::Result<()> {
        self.write_all(&value.to_be_bytes())
    }

    fn write_string_len16(&mut self, value: &str) -> io::Result<()> {
        let bytes = value.as_bytes();
        if bytes.len() > u16::MAX as usize {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "String too long"));
        }
        self.write_u16(bytes.len() as u16)?;
        self.write_all(bytes)
    }

    fn write_string_len32(&mut self, value: &str) -> io::Result<()> {
        let bytes = value.as_bytes();
        if bytes.len() > u32::MAX as usize {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "String too long"));
        }
        self.write_u32(bytes.len() as u32)?;
        self.write_all(bytes)
    }
}

pub trait ReadExt {
    fn read_u8(&mut self) -> io::Result<u8>;
    fn read_u16(&mut self) -> io::Result<u16>;
    fn read_u32(&mut self) -> io::Result<u32>;
    fn read_u64(&mut self) -> io::Result<u64>;
    fn read_i64(&mut self) -> io::Result<i64>;
    fn read_f64(&mut self) -> io::Result<f64>;
    fn read_string_len16(&mut self) -> io::Result<String>;
    fn read_string_len32(&mut self) -> io::Result<String>;
}

impl<R: io::Read> ReadExt for R {
    fn read_u8(&mut self) -> io::Result<u8> {
        let mut buffer = [0u8; 1];
        self.read_exact(&mut buffer)?;
        Ok(buffer[0])
    }

    fn read_u16(&mut self) -> io::Result<u16> {
        let mut buffer = [0u8; 2];
        self.read_exact(&mut buffer)?;
        Ok(u16::from_be_bytes(buffer))
    }

    fn read_u32(&mut self) -> io::Result<u32> {
        let mut buffer = [0u8; 4];
        self.read_exact(&mut buffer)?;
        Ok(u32::from_be_bytes(buffer))
    }

    fn read_u64(&mut self) -> io::Result<u64> {
        let mut buffer = [0u8; 8];
        self.read_exact(&mut buffer)?;
        Ok(u64::from_be_bytes(buffer))
    }

    fn read_i64(&mut self) -> io::Result<i64> {
        let mut buffer = [0u8; 8];
        self.read_exact(&mut buffer)?;
        Ok(i64::from_be_bytes(buffer))
    }

    fn read_f64(&mut self) -> io::Result<f64> {
        let mut buffer = [0u8; 8];
        self.read_exact(&mut buffer)?;
        Ok(f64::from_be_bytes(buffer))
    }

    fn read_string_len16(&mut self) -> io::Result<String> {
        let len = self.read_u16()? as usize;
        let mut buffer = vec![0u8; len];
        self.read_exact(&mut buffer)?;
        String::from_utf8(buffer).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    }

    fn read_string_len32(&mut self) -> io::Result<String> {
        let len = self.read_u32()? as usize;
        let mut buffer = vec![0u8; len];
        self.read_exact(&mut buffer)?;
        String::from_utf8(buffer).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    }
}