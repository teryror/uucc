# Technical Details

## UTF-8 Decoding

The basic decoder is based on the [Flexible and Economical UTF-8 Decoder by Bjoern
Hoerhmann][1]. A short, cleaned up description of the state machine underlying it
follows:

[1]: https://bjoern.hoehrmann.de/utf-8/decoder/dfa/

### Character Classes:

Each input byte is categorized into character classes before getting fed to the
DFA, drastically reducing the size of the table required.

       _0 _1 _2 _3 _4 _5 _6 _7 _8 _9 _A _B _C _D _E _F
    0_  A  A  A  A  A  A  A  A  A  A  A  A  A  A  A  A
    1_  A  A  A  A  A  A  A  A  A  A  A  A  A  A  A  A
    2_  A  A  A  A  A  A  A  A  A  A  A  A  A  A  A  A
    3_  A  A  A  A  A  A  A  A  A  A  A  A  A  A  A  A
    4_  A  A  A  A  A  A  A  A  A  A  A  A  A  A  A  A
    5_  A  A  A  A  A  A  A  A  A  A  A  A  A  A  A  A
    6_  A  A  A  A  A  A  A  A  A  A  A  A  A  A  A  A
    7_  A  A  A  A  A  A  A  A  A  A  A  A  A  A  A  A
    8_ c0 c0 c0 c0 c0 c0 c0 c0 c0 c0 c0 c0 c0 c0 c0 c0
    9_ c1 c1 c1 c1 c1 c1 c1 c1 c1 c1 c1 c1 c1 c1 c1 c1
    A_  C  C  C  C  C  C  C  C  C  C  C  C  C  C  C  C
    B_  C  C  C  C  C  C  C  C  C  C  C  C  C  C  C  C
    C_  O  O  2  2  2  2  2  2  2  2  2  2  2  2  2  2
    D_  2  2  2  2  2  2  2  2  2  2  2  2  2  2  2  2
    E_  z  3  3  3  3  3  3  3  3  3  3  3  3  S  3  3
    F_  e  4  4  4  a  X  X  X  X  X  X  X  X  X  X  X
    
    Key: A = US-ASCII
         c0 = Continuation (bits[4..5] == 00)
         c1 = Continuation (bits[4..5] == 01)
         C  = Continuation (bits[4..5] == 1x)
         O = Illegal (always overlong)
         2 = Leading Byte (2 cont. B)
         z = Leading Byte (3 cont. B, potentially overlong)
         3 = Leading Byte (3 cont. B)
         S = Leading Byte (3 cont. B, potentially surrogate)
         e = Leading Byte (4 cont. B, potentially overlong)
         4 = Leading Byte (4 cont. B)
         a = Leading Byte (4 cont. B, potentially out of legal range)
         X = Illegal (always out of range)

Continuation bytes are sub-categorized based on the values of their two most
significant bits, which are used for detecting invalid codepoints.

### State Machine:

This state machine has more distinct states than the original one it is based on,
allowing the consumer of the API to distinguish different types of decoding error.

    // State Transition Table:
       A c0 c1  C  O 2 z  3  S e  4 a  X
     . .  E  E  E eO 1 2o 2 2s 3o 3 3r E // Start
    3o E Eo  2  2  E E  E E  E  E E  E E // get 3 and check overlong
     3 E  2  2  2  E E  E E  E  E E  E E // get 3
    3r E  2 Er Er  E E  E E  E  E E  E E // get 3 and check range
    2o E Eo Eo  1  E E  E E  E  E E  E E // get 2 and check overlong
     3 E  1  1  1  E E  E E  E  E E  E E // get 2
    2s E  1  1 Es  E E  E E  E  E E  E E // get 2 and check surrogate
     1 E  .  .  .  E E  E E  E  E E  E E // get 1
    eO E Eo Eo Eo  E E  E E  E  E E  E E // get 1 (always overlong)
    E  // Eo, Er, Es, E all loop back to themselves for all inputs
    Eo //...
    Er //...
    Es //...
    
    Key: Eo = Overlong Encoding
         Er = Invalid Code Point (out of range)
         Es = Invalid Code Point (surrogate char)
         E  = Invalid Sequence

### Implementation

The C implementation suggested in the linked article looks like this:

    uint32_t inline
    decode(uint32_t* state, uint32_t* codepoint, uint8_t byte) {
        uint8_t type = CHARACTER_CLASS[byte];
        *codepoint = (*state != UTF8_ACCEPT) ?
          (byte & 0x3Fu) | (*codepoint << 6) :
          (0xFF >> type) & (byte);
        *state = NEXT_STATE[*state + type];
        return *state;
    }
    
    void usage_example(uint8_t* src, uint8_t* end) {
        uint32_t state = 0;    uint32_t codepoint = 0;
        while (src < end) {
            if (decode(&state, &codepoint, *src++) != UTF8_ACCEPT) continue;
            
            // ... process codepoint here ...
        }
    }

An API in line with (the now deprecated) `std::char::DecodeUtf8` seems like more
idiomatic Rust, and calls for a slightly tweaked implementation. Just like above,
we'll be decoding one codepoint at a time and decide what to do with each one, but
following an iterator protocol:

    impl Iterator for DecodeUtf8 {
        type Item = Result<char, Utf8Error>;
        
        fn next(&mut self) -> Option<Result<char, Utf8Error>> {
            let state = 0; let codepoint = 0;
            for _ in 0..4 {
                if self.src >= self.end {
                    return Some(Err(Utf8Error::UnexpectedEndOfStream));
                }
                
                self.decode(&mut state, &mut codepoint);
                if state == UTF8_ACCEPT {
                    return Some(Ok(char::from_u32_unchecked(codepoint)));
                } else if state >= UTF8_ERROR {
                    return Some(Err(Utf8Error::from_decoder_state(state)));
                }
            }
            
            // 
            // Cannot reach here because the design of the DFA guarantees that
            // one of the early returns will be hit after consuming at most 4 bytes.
            // 
            
            unreachable!();
        }
    }

Knowing that we will consume between one and four bytes, and that the first byte
in a sequence will get special treatment, we can simplify things by unrolling the
first loop iteration and manually inlining `self.decode`. We can further exploit
this knowledge by splitting this function into a fast path, doing only a single
bounds check, which will decode most of a stream, and a slow, paranoid path, which
will decode at most the last four bytes.

It may also be profitable to encode the number of consumed bytes into the state
representation somehow so that `self.src` need only be incremented once, getting
rid of false serial dependencies between the loop iterations.

## Character Classification

Since the primary use case of this decoder will be in lexers, the exact codepoint
often will not be as important as one or two of its Unicode properties. It may be
worthwhile to do decoding and categorization of some sort in one go.

For example, since the first byte will get special treatment anyway, we could replace
the ASCII range of the character class table with indicators of the final category
we care about; by reading from the table before branching on `byte < 0x80`, we can
leverage the one load in both branches, hopefully reducing the relative cost of
misprediction.

One of the most important Unicode properties Lexers will care about, is the
`General_Category`, which can have one of 30 distinct values, which are in turn
grouped into eight larger categories:

    General_Category Values:
    - L (Letter)
      - LC (Cased_Letter)
        - Lu (Uppercase_Letter)
        - Ll (Lowercase_Letter)
        - Lt (Titlecase_Letter)
      - Lm (Modifier_Letter)
      - Lo (Other_Letter)
    - M (Mark)
      - Mn (Nonspacing_Mark)
      - Mc (Spacing_Mark)
      - Me (Enclosing_Mark)
    - N (Number)
      - Nd (Decimal_Number)
      - Nl (Letter_Number)
      - No (Other_Number)
    - P (Punctuation)
      - Pc (Connector_Punctuation)
      - Pd (Dash_Punctuation)
      - Ps (Open_Punctuation)
      - Pe (Close_Punctuation)
      - Pi (Initial_Punctuation)
      - Pf (Final_Punctuation)
      - Po (Other_Punctuation)
    - S (Symbol)
      - Sm (Math_Symbol)
      - Sc (Currency_Symbol)
      - Sk (Modifier_Symbol)
      - So (Other_Symbol)
    - Z (Seperator)
      - Zs (Space_Seperator)
      - Zl (Line_Seperator)
      - Zp (Paragraph_Seperator)
    - C (Other)
      - Cc (Control)
      - Cf (Format)
      - Cs (Surrogate)
      - Co (Private_Use)
      - Cn (Unassigned)

Of course, we could fit 30 distinct values into a 5-bit code, but this complicates
the (common) case where you want to check whether a character belongs to one of
several of these categories.

We could instead represent a general category with 6 bits, where the upper three
bits distinguish between `L`, `M`, `N`, `P`, `S`, `Z`, and `O`, and the lower three
bits encode the subcategory. This allows, for example, testing for the supercategories
with a single bitwise `&` and a compare, like `is_number(cat: u8) = (cat & 0x3F)
== CAT_NUMBER`. We can optimize the most important properties deriving from the
general categories (like [`ID_Start` and `ID_Continue`][2]) by clever choice of
bit patterns.

[2]: http://unicode.org/reports/tr31/#Table_Lexical_Classes_for_Identifiers

Alternatively, we could chose a dense 5-bit code which gets expanded into a 32-bit
value with just a single bit set: `expand(cat: u8) = (1u32 << cat)`. We can then
test for any combination of categories in a consistent manner: `is_letter_or_number(cat:
u8) = (expand(cat) & (CAT_NUMBER | CAT_LETTER)) != 0`.