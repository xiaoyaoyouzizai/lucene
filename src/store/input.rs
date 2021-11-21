use crate::common::read_int_from_bytes;
use crate::common::read_vint_from_bytes;
use fxhash::FxHashMap;
use std::str;

pub struct DataInput<'a> {
    data: &'a [u8],
}

impl<'a> DataInput<'a> {
    pub fn new(data: &'a [u8]) -> DataInput<'a> {
        DataInput { data }
    }

    pub fn read_string_set(&mut self) -> FxHashMap<String, ()> {
        let count = self.read_vint() as usize;
        match count {
            0 => FxHashMap::<String, ()>::with_capacity_and_hasher(0, Default::default()),
            _ => {
                let mut map =
                    FxHashMap::<String, ()>::with_capacity_and_hasher(count, Default::default());
                for _ in 0..count {
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
        let len = self.read_byte() as usize & 0xFF;
        let result = String::from(str::from_utf8(&self.data[..len]).unwrap());
        self.data = &self.data[len..];
        result
    }

    pub fn read_byte(&mut self) -> u8 {
        let result = self.data[0];
        //println!("read_byte: {}", result);
        self.data = &self.data[1..];
        result
    }

    pub fn read_bytes(&mut self, len: usize) -> Vec<u8> {
        let result = self.data[..len].to_vec();
        self.data = &self.data[len..];
        result
    }

    pub fn read_string(&mut self) -> String {
        let len = self.read_vint() as usize;
        let result = String::from(str::from_utf8(&self.data[..len]).unwrap());
        self.data = &self.data[len..];
        result
    }

    pub fn read_int(&mut self) -> u32 {
        let result = read_int_from_bytes(self.data);
        self.data = &self.data[4..];
        result
    }

    pub fn read_vint(&mut self) -> u32 {
        let (result, len) = read_vint_from_bytes(self.data);
        self.data = &self.data[len..];
        result
    }
}
