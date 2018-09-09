pub mod decode_utf8;

#[cfg(test)]
mod tests {
    use std::str::from_utf8;
    use decode_utf8::*;
    use self::Utf8Error::*;
    
    macro_rules! err_tests {
        ($($name:ident: $expected_err:expr, $dat:expr,)*) => {
        $(
            #[test]
            fn $name() {
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
