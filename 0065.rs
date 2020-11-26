// Validate if a given string can be interpreted as a decimal number.

// Some examples:
// "0" => true
// " 0.1 " => true
// "abc" => false
// "1 a" => false
// "2e10" => true
// " -90e3   " => true
// " 1e" => false
// "e3" => false
// " 6e-1" => true
// " 99e2.5 " => false
// "53.5e93" => true
// " --6 " => false
// "-+3" => false
// "95a54e53" => false

// Note: It is intended for the problem statement to be ambiguous. It would be best if you gathered all requirements up front before implementing one. However, here is a list of characters that can be in a valid decimal number:

// Numbers 0-9
// Exponent - "e"
// Positive/negative sign - "+"/"-"
// Decimal point - "."
// Of course, the context of these characters also matters in the input.

// Example 1:

// Input: s = "0"
// Output: true
// Example 2:

// Input: s = "3"
// Output: true

// Constraints:

// 1 <= s.length <= 20
// s consists of only English letters, digits, space ' ', plus '+', minus '-', or dot '.'.

fn is_number(s: String) -> bool {
    #[derive(Debug, Eq, PartialEq)]
    enum State {
        IntegerInitial,
        IntegerSigned,
        Integer,
        FractionInitial,
        FractionInitialWithInteger,
        Fraction,
        ExpInitial,
        ExpSigned,
        Exp,
        End
    };

    let mut state = State::IntegerInitial;

    let mut s = s;
    s.push(' '); // trick to force into End state from any valid states
    for c in s.chars() {
        macro_rules! transition {
            ($($($from:ident)|+ @ $p:pat => $to:ident);* $(;)?) => {
                match (&state, c) {
                    $($(
                        (State::$from, $p) => state = State::$to,
                    )+)*
                    _ => return false
                }
            };
        }

        transition! {
            IntegerInitial @ ' ' => IntegerInitial;
            IntegerInitial @ '+' => IntegerSigned;
            IntegerInitial @ '-' => IntegerSigned;
            IntegerInitial | IntegerSigned | Integer @ '0'..='9' => Integer;
            IntegerInitial | IntegerSigned @ '.' => FractionInitial;
            Integer @ '.' => FractionInitialWithInteger;
            FractionInitial | FractionInitialWithInteger | Fraction @ '0'..='9' => Fraction;
            Integer | FractionInitialWithInteger | Fraction @ 'e' => ExpInitial;
            ExpInitial @ '+' => ExpSigned;
            ExpInitial @ '-' => ExpSigned;
            ExpInitial | ExpSigned | Exp @ '0'..='9' => Exp;
            Integer | FractionInitialWithInteger | Fraction | Exp | End @ ' ' => End
        };
    }

    state == State::End
}

fn main() {
    assert!(is_number("0".into()));
    assert!(is_number(" 0.1 ".into()));
    assert!(!is_number("abc".into()));
    assert!(!is_number("1 a".into()));
    assert!(is_number("2e10".into()));
    assert!(is_number(" -90e3   ".into()));
    assert!(!is_number(" 1e".into()));
    assert!(!is_number("e3".into()));
    assert!(is_number(" 6e-1".into()));
    assert!(!is_number(" 99e2.5 ".into()));
    assert!(is_number("53.5e93".into()));
    assert!(!is_number(" --6 ".into()));
    assert!(!is_number("-+3".into()));
    assert!(!is_number("95a54e53".into()));
}

// faster than 100.00%, less memory than 96.30%.