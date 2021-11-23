use crc32fast::Hasher;
use fxhash::FxHashMap;
use std::str;

const STOP_BIT: u8 = 128;

// trait ByteInput {
//     fn read_byte(&mut self) -> u8;
//     fn read_bytes(&mut self, len: usize) -> &[u8];
// }

pub struct ChecksumByteInput<'a> {
    data: &'a [u8],
    hasher: Hasher,
}

impl<'a> ChecksumByteInput<'a> {
    pub fn new(data: &'a [u8]) -> ChecksumByteInput<'a> {
        ChecksumByteInput {
            data,
            hasher: Hasher::new(),
        }
    }
    // }
    // impl ByteInput for ChecksumByteInput<'_> {
    fn read_byte(&mut self) -> u8 {
        let result = self.data[0];
        //println!("read_byte: {}", result);
        self.data = &self.data[1..];
        result
    }
    fn read_bytes(&mut self, len: usize) -> &[u8] {
        let bytes = &self.data[..len];
        self.data = &self.data[len..];
        bytes
    }
}

pub struct DataInput<'a> {
    input: ChecksumByteInput<'a>,
}

impl<'a> DataInput<'a> {
    pub fn new(input: ChecksumByteInput) -> DataInput {
        DataInput { input }
    }

    pub fn read_long(&mut self) -> u64 {
        ((self.read_int() as u64) << 32) | (self.read_int() as u64 & 0xFFFFFFFF)
    }

    pub fn read_string_set(&mut self) -> FxHashMap<String, ()> {
        let size = self.read_vint() as usize;
        match size {
            0 => FxHashMap::<String, ()>::with_capacity_and_hasher(0, Default::default()),
            _ => {
                let mut map =
                    FxHashMap::<String, ()>::with_capacity_and_hasher(size, Default::default());
                for _ in 0..size {
                    map.insert(self.read_string(), ());
                }
                map
            }
        }
    }

    pub fn read_string_map(&mut self) -> FxHashMap<String, String> {
        let count = self.read_vint() as usize;
        match count {
            0 => FxHashMap::<String, String>::with_capacity_and_hasher(0, Default::default()),
            _ => {
                let mut map = FxHashMap::<String, String>::with_capacity_and_hasher(
                    count,
                    Default::default(),
                );
                for _ in 0..count {
                    map.insert(self.read_string(), self.read_string());
                }
                map
            }
        }
    }

    pub fn read_short_string(&mut self) -> String {
        let len = self.input.read_byte() as usize & 0xFF;
        let result = String::from(str::from_utf8(self.input.read_bytes(len)).unwrap());
        result
    }

    pub fn read_byte(&mut self) -> u8 {
        self.input.read_byte()
    }

    pub fn read_bytes(&mut self, len: usize) -> Vec<u8> {
        let result = self.input.read_bytes(len).to_vec();
        result
    }

    pub fn read_string(&mut self) -> String {
        let len = self.read_vint() as usize;
        let result = String::from(str::from_utf8(&self.input.read_bytes(len)).unwrap());
        result
    }

    pub fn read_vlong(&mut self, allow_negative: bool) -> u64 {
        let mut byte = self.input.read_byte();
        if byte < STOP_BIT {
            return byte.into();
        }
        let mut result: u64 = (byte & 0x7F).into();

        for i in 1..9 {
            let bit_shifts = i * 7;
            byte = self.input.read_byte();
            result |= ((byte & 0x7F) as u64) << bit_shifts;
            if byte < STOP_BIT {
                return result;
            }
        }

        if allow_negative {
            byte = self.input.read_byte();
            result |= ((byte & 0x7F) as u64) << 63;
            if byte == 0 || byte == 1 {
                return result;
            }
        }
        panic!("vlong error");
    }

    pub fn read_int(&mut self) -> u32 {
        ((self.input.read_byte() as u32) << 24)
            | ((self.input.read_byte() as u32) << 16)
            | ((self.input.read_byte() as u32) << 8)
            | (self.input.read_byte() as u32)
    }

    pub fn read_vint(&mut self) -> u32 {
        let mut byte = self.input.read_byte();
        if byte < STOP_BIT {
            return byte.into();
        }
        let mut result: u32 = (byte & 0x7F).into();

        for i in 1..4 {
            let bit_shifts = i * 7;
            byte = self.input.read_byte();
            result |= ((byte & 0x7F) as u32) << bit_shifts;
            if byte < STOP_BIT {
                return result;
            }
        }

        byte = self.input.read_byte();
        result |= ((byte & 0x7F) as u32) << 28;
        if (byte & 0xF0) == 0 {
            return result;
        }

        panic!("vint error");
    }
}
