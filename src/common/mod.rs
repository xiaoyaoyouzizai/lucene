use fxhash::FxHashMap;
use std::str;

const STOP_BIT: u32 = 128;

pub fn read_string_set(bytes: &mut &[u8]) -> FxHashMap<String, ()> {
    let count = read_vint(bytes) as usize;
    match count {
        0 => FxHashMap::<String, ()>::with_capacity_and_hasher(0, Default::default()),
        _ => {
            let mut map =
                FxHashMap::<String, ()>::with_capacity_and_hasher(count, Default::default());
            for _ in 0..count {
                map.insert(read_string(bytes), ());
            }
            map
        }
    }
}

pub fn read_string_map(bytes: &mut &[u8]) -> FxHashMap<String, String> {
    let count = read_vint(bytes) as usize;
    match count {
        0 => FxHashMap::<String, String>::with_capacity_and_hasher(0, Default::default()),
        _ => {
            let mut map =
                FxHashMap::<String, String>::with_capacity_and_hasher(count, Default::default());
            for _ in 0..count {
                map.insert(read_string(bytes), read_string(bytes));
            }
            map
        }
    }
}

pub fn read_short_string(bytes: &mut &[u8]) -> String {
    let len = read_byte(bytes) as usize & 0xFF;
    let result = String::from(str::from_utf8(&bytes[..len]).unwrap());
    *bytes = &bytes[len..];
    result
}

pub fn read_byte(bytes: &mut &[u8]) -> u8 {
    let result = bytes[0];
    //println!("read_byte: {}", result);
    *bytes = &bytes[1..];
    result
}

pub fn read_bytes(bytes: &mut &[u8], len: usize) -> Vec<u8> {
    let result = bytes[..len].to_vec();
    *bytes = &bytes[len..];
    result
}

pub fn read_string(bytes: &mut &[u8]) -> String {
    let len = read_vint(bytes) as usize;
    let result = String::from(str::from_utf8(&bytes[..len]).unwrap());
    *bytes = &bytes[len..];
    result
}

pub fn read_int(bytes: &mut &[u8]) -> u32 {
    let result = read_int_from_bytes(bytes);
    *bytes = &bytes[4..];
    result
}

pub fn read_vint(bytes: &mut &[u8]) -> u32 {
    let (result, len) = read_vint_from_bytes(bytes);
    *bytes = &bytes[len..];
    result
}

pub fn read_int_from_bytes(bytes: &[u8]) -> u32 {
    ((bytes[0] as u32) << 24)
        | ((bytes[1] as u32) << 16)
        | ((bytes[2] as u32) << 8)
        | (bytes[3] as u32)
}

type VInt = (u32, usize);

pub fn read_vint_from_bytes(bytes: &[u8]) -> VInt {
    let mut result: u32 = 0;

    if let Some(vint) = merge_init(&mut result, bytes[0].into()) {
        return vint;
    }

    for i in 1..4 {
        if let Some(vint) = merge_byte(&mut result, bytes[i].into(), i) {
            return vint;
        }
    }

    if let Some(vint) = merge_end(&mut result, bytes[4].into()) {
        return vint;
    }

    panic!("vint error");
}

fn merge_init(result: &mut u32, byte: u32) -> Option<VInt> {
    if byte < STOP_BIT {
        return Some((byte, 1));
    }

    *result = byte & 0x7F;

    None
}

fn merge_byte(result: &mut u32, byte: u32, count: usize) -> Option<VInt> {
    if byte < STOP_BIT {
        *result |= byte << (count * 7);
        return Some((*result, count + 1));
    }
    *result |= (byte & 0x7F) << (count * 7);

    None
}

fn merge_end(result: &mut u32, byte: u32) -> Option<VInt> {
    *result |= (byte & 0x0F) << 28;
    if (byte & 0xF0) == 0 {
        return Some((*result, 5));
    }

    None
}
