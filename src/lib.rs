mod decode_utf8;

#[cfg(test)]
mod tests {
    use std::str::from_utf8;
    use decode_utf8::*;
    
    #[test]
    fn decodes_demo_utf8_txt() {
        let demo_utf8_txt = include_bytes!("../tst-dat/demo-utf8.txt");
        
        let text_content = from_utf8(demo_utf8_txt).unwrap();
        let mut iter = decode_utf8(demo_utf8_txt);
        
        for expected in text_content.chars() {
            let got = iter.next().unwrap();
            assert_eq!(got, expected);
        }
        
        assert!(iter.next().is_none());
    }
}
