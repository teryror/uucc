pub mod decode_utf8;
mod tables;

#[cfg(test)]
mod tests {
    use std::char::from_u32;
    use std::str::from_utf8;
    use decode_utf8::*;
    
    macro_rules! err_tests {
        ($($name:ident: $expected_err:expr, $dat:expr,)*) => {
        $(
            #[test]
            fn $name() {
                use self::Utf8Error::*;
                
                let bytes = $dat;
                let mut iter = decode_utf8(&bytes);
                
                assert!(iter.next_char().is_none());
                assert_eq!(iter.status().unwrap_err(), $expected_err);
            }
        )*
        }
    }
    
    err_tests! {
        detects_illegal_lead: NotALeadingByte, [0x80],
        detects_illegal_cont: NotAContinuationByte, [0xC2, 0x20],
        detects_overlong_encoding3: OverlongEncoding, [0xE0, 0x80, 0x80],
        detects_overlong_encoding4: OverlongEncoding, [0xF0, 0x80, 0x80, 0x80],
        detects_surrogate_character: SurrogateCharacter, [0xED, 0xA0, 0x80],
        detects_illegal_codepoint: OutOfCharacterRange, [0xF4, 0xBF, 0xBF],
        detects_early_end_of_stream: UnexpectedEndOfBuffer, [0xC2],
    }
    
    #[test]
    fn decodes_demo_utf8_txt() {
        let demo_utf8_txt = include_bytes!("../tst-dat/demo-utf8.txt");
        
        let text_content = from_utf8(demo_utf8_txt).unwrap();
        let mut iter = decode_utf8(demo_utf8_txt);
        
        for expected in text_content.chars() {
            if let Some(got) = iter.next_char() {
                assert_eq!(got, expected);
            } else {
                unreachable!();
            }
        }
        
        assert!(iter.next_char().is_none());
    }
    
    #[test]
    fn categorizes_ascii() {
        let bytes = [0x0A, 0x20, 0x2D, 0x32, 0x5A, 0x7A];
        let mut iter = decode_utf8(&bytes);
        
        let cat = iter.next_char_and_category().unwrap().1;
        assert!(cat.subset_of(OTHER));
        assert_eq!(cat, CONTROL);
        
        let cat = iter.next_char_and_category().unwrap().1;
        assert!(cat.subset_of(SEPERATOR));
        assert_eq!(cat, SPACE_SEPERATOR);
        
        let cat = iter.next_char_and_category().unwrap().1;
        assert!(cat.subset_of(PUNCTUATION));
        assert_eq!(cat, DASH_PUNCTUATION);
        
        let cat = iter.next_char_and_category().unwrap().1;
        assert!(cat.subset_of(NUMBER));
        assert_eq!(cat, DECIMAL_NUMBER);
        
        let cat = iter.next_char_and_category().unwrap().1;
        assert!(cat.subset_of(CASED_LETTER));
        assert!(cat.subset_of(LETTER));
        assert_eq!(cat, UPPERCASE_LETTER);
        
        let cat = iter.next_char_and_category().unwrap().1;
        assert!(cat.subset_of(CASED_LETTER));
        assert!(cat.subset_of(LETTER));
        assert_eq!(cat, LOWERCASE_LETTER);
        
        assert!(iter.next_char_and_category().is_none());
    }
    
    #[test]
    fn identifies_scripts() {
        use self::Script::*;
        
        // codepoints in Private Use Area are script "Unknown" (0)
        let mut s = String::new();
        let pau_ranges = [(0xE000, 0xF8FF), (0xF0000, 0xFFFFD), (0x100000, 0x10FFFD)];
        for range in pau_ranges.iter() {
            for codepoint in range.0..range.1+1 {
                s.push(from_u32(codepoint).unwrap());
                let mut iter = decode_utf8(s.as_str().as_bytes());
                let (char, script) = iter.next_char_and_script().unwrap();
                assert_eq!(char as u32, codepoint); assert_eq!(script, Unknown);
                assert!(iter.next_char().is_none());
                s.clear();
            }
        }
        
        // examples of other scripts
        let examples = [
            (0x10u32, Common), (0xB2, Common), (0x3015, Common), (0x10100, Common),
            (0x41, Latin), (0x294, Latin), (0x2094, Latin), (0xFF41, Latin),
            (0x370, Greek), (0x1D234, Greek), (0x400, Cyrillic), (0xFE2F, Cyrillic),
            (0x531, Armenian), (0xFB17, Armenian), (0x591, Hebrew), (0xFB4F, Hebrew),
            (0x600, Arabic), (0x1EEF1, Arabic), (0x700, Syriac), (0x86A, Syriac),
            (0x7AA, Thaana), (0x958, Devanagari), (0x9C1, Bengali), (0xA33, Gurmukhi),
            (0xAC1, Gujarati), (0xB47, Oriya), (0xBCD, Tamil), (0xC0C, Telugu),
            (0xCBD, Kannada), (0xD12, Malayalam), (0xDBD, Sinhala), (0xE5A, Thai),
            (0xEAD, Lao), (0xF35, Tibetan), (0x1087, Myanmar), (0x2D2D, Georgian),
            (0x1100, Hangul), (0x302E, Hangul), (0xA960, Hangul), (0xFFDC, Hangul),
            (0x1200, Ethiopic), (0xAB2E, Ethiopic), (0x13A0, Cherokee), (0xABBF, Cherokee),
            (0x3041, Hiragana), (0x1F200, Hiragana), (0x30A1, Katakana), (0x1B000, Katakana),
            (0x2E80, Han), (0x4DB5, Han), (0x65A1, Han), (0xF900, Han), (0x2FA1D, Han)
        ];
        
        for (codepoint, expected) in examples.iter() {
            s.push(from_u32(*codepoint).unwrap());
            let mut iter = decode_utf8(s.as_str().as_bytes());
            let (char, script) = iter.next_char_and_script().unwrap();
            assert_eq!(char as u32, *codepoint); assert_eq!(script, *expected);
            assert!(iter.next_char().is_none());
            s.clear();
        }
    }
    
    #[test]
    fn marked_strings_respect_boundaries() {
        let mut iter = decode_utf8(&[0x20, 0x20, 0x80, 0x20, 0x20]);
        
        let start = iter.mark();
        assert_eq!(iter.try_get_marked_string(start).unwrap(), "");
        
        iter.next_char(); iter.next_char();
        assert_eq!(iter.try_get_marked_string(start).unwrap(), "  ");
                
        let before_error = iter.mark();
        assert_eq!(iter.try_get_marked_string(before_error).unwrap(), "");
        
        assert!(iter.next_char().is_none());
        let after_error = iter.mark();
        assert_eq!(iter.try_get_marked_string(after_error).unwrap(), "");
        
        assert!(iter.try_get_marked_string(start).is_err());
        assert!(iter.try_get_marked_string(before_error).is_err());
        
        iter.next_char(); iter.next_char();
        let end = iter.mark();
        assert_eq!(iter.try_get_marked_string(end).unwrap(), "");
        assert_eq!(iter.try_get_marked_string(after_error).unwrap(), "  ");
        
        assert!(iter.try_get_marked_string(start).is_err());
        assert!(iter.try_get_marked_string(before_error).is_err());
        
    }
}
