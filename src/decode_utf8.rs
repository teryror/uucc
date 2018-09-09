#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Utf8Error {
    NotALeadingByte,
    NotAContinuationByte,
    OverlongEncoding,
    SurrogateCharacter,
    OutOfCharacterRange,
    UnexpectedEndOfBuffer,
}

#[derive(Clone)]
pub struct Utf8Decoder {
    status: Result<(), Utf8Error>,
    first: * const u8,
    next: * const u8,
    end: * const u8
}

pub fn decode_utf8(raw: &[u8]) -> Utf8Decoder {
    unsafe {
        let first = &raw[0] as * const u8;
        Utf8Decoder {
            first: first,
            status: Ok(()),
            next: first,
            end: first.offset(raw.len() as isize),
        }
    }
}

#[derive(Clone, Copy)]
pub struct Utf8DecoderPosition{raw: * const u8}

// 
// Unicode Property General_Category
// 

pub type GeneralCategory = u32;

pub trait BitSet {
    fn subset_of(self, superset: Self) -> bool;
}

impl BitSet for GeneralCategory {
    fn subset_of(self, superset: GeneralCategory) -> bool {
        (self & superset) == self
    }
}

const LU: u8 = 0x00;
const LL: u8 = 0x01;
const LT: u8 = 0x02;
const LM: u8 = 0x03;
const LO: u8 = 0x04;
const MN: u8 = 0x05;
const MC: u8 = 0x06;
const ME: u8 = 0x07;
const ND: u8 = 0x08;
const NL: u8 = 0x09;
const NO: u8 = 0x0A;
const PC: u8 = 0x0B;
const PD: u8 = 0x0C;
const PS: u8 = 0x0D;
const PE: u8 = 0x0E;
const PI: u8 = 0x0F;
const PF: u8 = 0x10;
const PO: u8 = 0x11;
const SM: u8 = 0x12;
const SC: u8 = 0x13;
const SK: u8 = 0x14;
const SO: u8 = 0x15;
const ZS: u8 = 0x16;
const ZL: u8 = 0x17;
const ZP: u8 = 0x18;
const CC: u8 = 0x19;
const CF: u8 = 0x1A;
const CS: u8 = 0x1B;
const CO: u8 = 0x1C;
const CN: u8 = 0x1D;

pub const UPPERCASE_LETTER: GeneralCategory = 1 << LU;
pub const LOWERCASE_LETTER: GeneralCategory = 1 << LL;
pub const TITLECASE_LETTER: GeneralCategory = 1 << LT;
pub const MODIFIER_LETTER: GeneralCategory = 1 << LM;
pub const OTHER_LETTER: GeneralCategory = 1 << LO;
pub const CASED_LETTER: GeneralCategory =
    UPPERCASE_LETTER | LOWERCASE_LETTER | TITLECASE_LETTER;
pub const LETTER: GeneralCategory =
    CASED_LETTER | MODIFIER_LETTER | OTHER_LETTER;

pub const NONSPACING_MARK: GeneralCategory = 1 << MN;
pub const SPACING_MARK: GeneralCategory = 1 << MC;
pub const ENCLOSING_MARK: GeneralCategory = 1 << ME;
pub const MARK: GeneralCategory = NONSPACING_MARK | SPACING_MARK | ENCLOSING_MARK;

pub const DECIMAL_NUMBER: GeneralCategory = 1 << ND;
pub const LETTER_NUMBER: GeneralCategory = 1 << NL;
pub const OTHER_NUMBER: GeneralCategory = 1 << NO;
pub const NUMBER: GeneralCategory = DECIMAL_NUMBER | LETTER_NUMBER | OTHER_NUMBER;

pub const CONNECTOR_PUNCTUATION: GeneralCategory = 1 << PC;
pub const DASH_PUNCTUATION: GeneralCategory = 1 << PD;
pub const OPEN_PUNCTUATION: GeneralCategory = 1 << PS;
pub const CLOSE_PUNCTUATION: GeneralCategory = 1 << PE;
pub const INITIAL_PUNCTUATION: GeneralCategory = 1 << PI;
pub const FINAL_PUNCTUATION: GeneralCategory = 1 << PF;
pub const OTHER_PUNCTUATION: GeneralCategory = 1 << PO;
pub const PUNCTUATION: GeneralCategory =
    CONNECTOR_PUNCTUATION | DASH_PUNCTUATION | OPEN_PUNCTUATION | CLOSE_PUNCTUATION |
    INITIAL_PUNCTUATION   | FINAL_PUNCTUATION | OTHER_PUNCTUATION;

pub const MATH_SYMBOL: GeneralCategory = 1 << SM;
pub const CURRENCY_SYMBOL: GeneralCategory = 1 << SC;
pub const MODIFIER_SYMBOL: GeneralCategory = 1 << SK;
pub const OTHER_SYMBOL: GeneralCategory = 1 << SO;
pub const SYMBOL: GeneralCategory =
    MATH_SYMBOL | CURRENCY_SYMBOL | MODIFIER_SYMBOL | OTHER_SYMBOL;

pub const SPACE_SEPERATOR: GeneralCategory = 1 << ZS;
pub const LINE_SEPERATOR: GeneralCategory = 1 << ZL;
pub const PARAGRAPH_SEPERATOR: GeneralCategory = 1 << ZP;
pub const SEPERATOR: GeneralCategory =
    SPACE_SEPERATOR | LINE_SEPERATOR | PARAGRAPH_SEPERATOR;

pub const CONTROL: GeneralCategory = 1 << CC;
pub const FORMAT: GeneralCategory = 1 << CF;
pub const SURROGATE: GeneralCategory = 1 << CS;
pub const PRIVATE_USE: GeneralCategory = 1 << CO;
pub const UNASSIGNED: GeneralCategory = 1 << CN;
pub const OTHER: GeneralCategory =
    CONTROL | FORMAT | SURROGATE | PRIVATE_USE | UNASSIGNED;

// 
// Input Byte Classification
// 

const ASC: u8 =  0; // US-ASCII, leading byte of single byte sequence
const C00: u8 =  1; // continuation byte, bits[5..6] = 00
const C01: u8 =  9; // continuation byte, bits[5..6] = 01
const C1X: u8 =  7; // continuation byte, bits[5..6] = 1x
const L2N: u8 =  2; // leading byte of 2-byte sequence, no additional checks
const L3N: u8 =  3; // leading byte of 3-byte sequence, no additional checks
const L4N: u8 =  6; // leading byte of 4-byte sequence, no additional checks
const L3O: u8 = 10; // leading byte of 3-byte sequence, potentially overlong
const L3S: u8 =  4; // leading byte of 3-byte sequence, potentially surrogate
const L4O: u8 = 11; // leading byte of 4-byte sequence, potentially overlong
const L4R: u8 =  5; // leading byte of 4-byte sequence, potentially out of range
const ERR: u8 =  8; // always illegal

const CHAR_CLASSES: [u8; 256] = [
    ASC, ASC, ASC, ASC, ASC, ASC, ASC, ASC, ASC, ASC, ASC, ASC, ASC, ASC, ASC, ASC,
    ASC, ASC, ASC, ASC, ASC, ASC, ASC, ASC, ASC, ASC, ASC, ASC, ASC, ASC, ASC, ASC,
    ASC, ASC, ASC, ASC, ASC, ASC, ASC, ASC, ASC, ASC, ASC, ASC, ASC, ASC, ASC, ASC,
    ASC, ASC, ASC, ASC, ASC, ASC, ASC, ASC, ASC, ASC, ASC, ASC, ASC, ASC, ASC, ASC,
    ASC, ASC, ASC, ASC, ASC, ASC, ASC, ASC, ASC, ASC, ASC, ASC, ASC, ASC, ASC, ASC,
    ASC, ASC, ASC, ASC, ASC, ASC, ASC, ASC, ASC, ASC, ASC, ASC, ASC, ASC, ASC, ASC,
    ASC, ASC, ASC, ASC, ASC, ASC, ASC, ASC, ASC, ASC, ASC, ASC, ASC, ASC, ASC, ASC,
    ASC, ASC, ASC, ASC, ASC, ASC, ASC, ASC, ASC, ASC, ASC, ASC, ASC, ASC, ASC, ASC,
    C00, C00, C00, C00, C00, C00, C00, C00, C00, C00, C00, C00, C00, C00, C00, C00,
    C01, C01, C01, C01, C01, C01, C01, C01, C01, C01, C01, C01, C01, C01, C01, C01,
    C1X, C1X, C1X, C1X, C1X, C1X, C1X, C1X, C1X, C1X, C1X, C1X, C1X, C1X, C1X, C1X,
    C1X, C1X, C1X, C1X, C1X, C1X, C1X, C1X, C1X, C1X, C1X, C1X, C1X, C1X, C1X, C1X,
    ERR, ERR, L2N, L2N, L2N, L2N, L2N, L2N, L2N, L2N, L2N, L2N, L2N, L2N, L2N, L2N,
    L2N, L2N, L2N, L2N, L2N, L2N, L2N, L2N, L2N, L2N, L2N, L2N, L2N, L2N, L2N, L2N,
    L3O, L3N, L3N, L3N, L3N, L3N, L3N, L3N, L3N, L3N, L3N, L3N, L3N, L3S, L3N, L3N,
    L4O, L4N, L4N, L4N, L4R, ERR, ERR, ERR, ERR, ERR, ERR, ERR, ERR, ERR, ERR, ERR,
];

// US-ASCII only for now; will need a different data structure for higher codepoints
const GENERAL_CATEGORY: [u8; 128] = [
    CC, CC, CC, CC, CC, CC, CC, CC, CC, CC, CC, CC, CC, CC, CC, CC, 
    CC, CC, CC, CC, CC, CC, CC, CC, CC, CC, CC, CC, CC, CC, CC, CC, 
    ZS, PO, PO, PO, SC, PO, PO, PO, PS, PE, PO, SM, PO, PD, PO, PO,
    ND, ND, ND, ND, ND, ND, ND, ND, ND, ND, PO, PO, SM, SM, SM, PO,
    PO, LU, LU, LU, LU, LU, LU, LU, LU, LU, LU, LU, LU, LU, LU, LU,
    LU, LU, LU, LU, LU, LU, LU, LU, LU, LU, LU, PS, PO, PE, SK, PC,
    SK, LL, LL, LL, LL, LL, LL, LL, LL, LL, LL, LL, LL, LL, LL, LL,
    LL, LL, LL, LL, LL, LL, LL, LL, LL, LL, LL, PS, SM, PE, SM, CC,
];

// 
// State Transitions
// 

const OK: u8 =  0 * 12; // initial state, accept
const G1: u8 =  1 * 12; // get 1 more byte
const G2: u8 =  2 * 12; // get 2 more bytes
const O2: u8 =  3 * 12; // get 2 more bytes, check overlong
const S2: u8 =  4 * 12; // get 2 more bytes, check surrogate
const G3: u8 =  5 * 12; // get 3 more bytes,
const O3: u8 =  6 * 12; // get 3 more bytes, check overlong
const R3: u8 =  7 * 12; // get 3 more bytes, check range
const EL: u8 =  8 * 12; // error: not a leading byte
const EC: u8 =  9 * 12; // error: not a continuation byte
const EO: u8 = 10 * 12; // error: overlong encoding
const ES: u8 = 11 * 12; // error: surrogate char
const ER: u8 = 12 * 12; // error: out of char range

const NEXT_STATE: [u8; 156] = [
    /*       ASC C00 L2N L3N L3S L4R L4N C1X ERR C01 L3O L4O */
    /* OK */ OK, EL, G1, G2, S2, R3, G3, EL, EL, EL, O2, O3,
    /* G1 */ EC, OK, EC, EC, EC, EC, EC, OK, EC, OK, EC, EC,
    /* G2 */ EC, G1, EC, EC, EC, EC, EC, G1, EC, G1, EC, EC,
    /* O2 */ EC, EO, EC, EC, EC, EC, EC, G1, EC, EO, EC, EC,
    /* S2 */ EC, G1, EC, EC, EC, EC, EC, ES, EC, G1, EC, EC,
    /* G3 */ EC, G2, EC, EC, EC, EC, EC, G2, EC, G2, EC, EC,
    /* O3 */ EC, EO, EC, EC, EC, EC, EC, G2, EC, G2, EC, EC,
    /* R3 */ EC, G2, EC, EC, EC, EC, EC, ER, EC, ER, EC, EC,
    /* EL */ EL, EL, EL, EL, EL, EL, EL, EL, EL, EL, EL, EL,
    /* EC */ EC, EC, EC, EC, EC, EC, EC, EC, EC, EC, EC, EC,
    /* EO */ EO, EO, EO, EO, EO, EO, EO, EO, EO, EO, EO, EO,
    /* ES */ ES, ES, ES, ES, ES, ES, ES, ES, ES, ES, ES, ES,
    /* ER */ ER, ER, ER, ER, ER, ER, ER, ER, ER, ER, ER, ER,
];

use self::Utf8Error::*;
use std::char::from_u32_unchecked;
use std::slice::from_raw_parts;
use std::str::from_utf8_unchecked;

macro_rules! cont_decode {
    ( $this:expr, $lead:expr, $ret:expr ) => {
        {
            let class = CHAR_CLASSES[$lead as usize];
            let mut codepoint = (0xFFu32 >> class) & ($lead as u32);
            let mut state = NEXT_STATE[class as usize];
            
            for _ in 1..4 {
                if state >= EL { break; }
                if $this.next >= $this.end { state = 0; break; }
                
                let byte = * $this.next;
                $this.next = $this.next.offset(1);
                
                let class = CHAR_CLASSES[byte as usize];
                codepoint = (byte as u32 & 0x3F) | (codepoint << 6);
                state = NEXT_STATE[(state + class) as usize];
                
                if state == OK { return Some(($ret)(codepoint)); }
            }
            
            $this.first = $this.next;
            $this.status = Err(match state {
                              EL => NotALeadingByte,
                              EC => NotAContinuationByte,
                              EO => OverlongEncoding,
                              ES => SurrogateCharacter,
                              ER => OutOfCharacterRange,
                              0 => UnexpectedEndOfBuffer,
                              _ => unreachable!()
                              });
            return None;
        }
    }
}

impl Utf8Decoder {
    pub fn status(&self) -> Result<(), Utf8Error> {
        self.status
    }
    
    pub fn next_char(&mut self) -> Option<char> {
        if self.next >= self.end { return None; }
        
        unsafe {
            let byte = *self.next;
            self.next = self.next.offset(1);
            if byte < 0x80 { return Some(byte as char); }
            
            cont_decode!(self, byte, from_u32_unchecked);
        }
    }
    
    pub fn next_char_and_category(&mut self) -> Option<(char, GeneralCategory)> {
        if self.next >= self.end { return None; }
        
        unsafe {
            let byte = *self.next;
            self.next = self.next.offset(1);
            
            if byte < 0x80 {
                let cat = GENERAL_CATEGORY[byte as usize];
                return Some((byte as char, (1 << cat) as GeneralCategory));
            }
            
            cont_decode!(self, byte, |_| { unreachable!(); });
        }
    }
    
    pub fn mark(&self) -> Utf8DecoderPosition {
        Utf8DecoderPosition{raw: self.next}
    }
    
    pub fn try_get_marked_string(&self, mark: Utf8DecoderPosition)
      -> Result<&str, Utf8Error>
    {
        match self.status {
            Ok(_) => {
                assert!(self.first <= mark.raw && mark.raw <= self.next);
            },
            Err(e) => if mark.raw < self.first {
                return Err(e);
            } else {
                assert!(mark.raw <= self.next);
            }
        }
        
        let size = (self.next as usize) - (mark.raw as usize);
        unsafe { Ok(from_utf8_unchecked(from_raw_parts(mark.raw, size))) }
    }
}