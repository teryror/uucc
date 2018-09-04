use std::char::from_u32_unchecked;

pub struct Utf8Decoder {
    next: * const u8,
    end: * const u8
}

pub fn decode_utf8(raw: &[u8]) -> Utf8Decoder {
    unsafe {
        let first = &raw[0] as * const u8;
        Utf8Decoder {
            next: first,
            end: first.offset(raw.len() as isize),
        }
    }
}

const CHAR_CLASSES: [u8; 256] = [
    0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
    1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,1,
    9,9,9,9,9,9,9,9, 9,9,9,9,9,9,9,9,
    7,7,7,7,7,7,7,7, 7,7,7,7,7,7,7,7,
    7,7,7,7,7,7,7,7, 7,7,7,7,7,7,7,7,
    8,8,2,2,2,2,2,2, 2,2,2,2,2,2,2,2,
    2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2,
    10,3,3,3,3,3,3,3,3,3,3,3,3,4,3,3,
    11,6,6,6,5,8,8,8,8,8,8,8,8,8,8,8
];

const NEXT_STATE: [u8; 108] = [
    0,12,24,36,60,96,84,12,12,12,48,72,
    12,12,12,12,12,12,12,12,12,12,12,12,
    12,0,12,12,12,12,12,0,12,0,12,12,
    12,24,12,12,12,12,12,24,12,24,12,12,
    12,12,12,12,12,12,12,24,12,12,12,12,
    12,24,12,12,12,12,12,12,12,24,12,12,
    12,12,12,12,12,12,12,36,12,36,12,12,
    12,36,12,12,12,12,12,36,12,36,12,12,
    12,36,12,12,12,12,12,12,12,12,12,12
];

impl Iterator for Utf8Decoder {
    type Item = char;
    
    fn next(&mut self) -> Option<char> {
        if self.next >= self.end { return None; }
        
        unsafe {
            let byte = *self.next;
            self.next = self.next.offset(1);
            
            let class = CHAR_CLASSES[byte as usize];
            if class == 0 { return Some(byte as char); }
            
            let mut codepoint = (0xFFu32 >> class) & (byte as u32);
            let mut state = NEXT_STATE[class as usize];
            
            for _ in 1..4 {
                if self.next >= self.end { return None; }
                let byte = *self.next;
                self.next = self.next.offset(1);
                
                let class = CHAR_CLASSES[byte as usize];
                codepoint = (byte as u32 & 0x3F) | (codepoint << 6);
                state = NEXT_STATE[(state + class) as usize];
                
                if state == 0 { return Some(from_u32_unchecked(codepoint)); }
            }
            
            None
        }
    }
}