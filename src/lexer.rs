pub fn lex(command_str: &str) -> Result<Option<Vec::<Field>>, LexError> {
    if command_str.starts_with(';') | command_str.trim().is_empty() {
        return Ok(None)
    }

    let mut scanner = Scanner::new(command_str);
    let mut fields = Vec::<Field>::new();

    loop {
        match scanner.peek() {
            // parenthetical comments
            Some('(') => {
                scanner.pop();
                let mut paren_indent_level: u32 = 1;
                loop {
                    match scanner.peek() {
                        Some('(') => paren_indent_level += 1,
                        Some(')') => paren_indent_level -= 1,
                        _ => {},
                    }
                    // if we're inside a parenthesis comment go ahead and skip
                    // any characters until we're back out
                    if paren_indent_level > 0 {
                        scanner.pop();
                        continue
                    }
                }
            },
            // quoted strings (needs to get moved into field parsing)
            //Some('"') => {},
            // expressions (needs to get moved into field parsing)
            //Some('{') => {},
            // end of command (semicolon comments are always at EOL)
            Some(';')
            // checksum / CRC is always at the end of the line and i really
            // dont care that much about them right now
            | Some('*')
            // additionally, if there is no character return we also want to
            // end the command parsing
            | None => return Ok(Some(fields)),
            // now that we've covered our bases, we can skip all remaining
            // spaces. nothing else cares about them
            Some(' ') => _ = scanner.pop(),
            // any other character gets checked as a field
            // this field check will eat the next character if there is no
            // available field
            // (yes this could be consolidated and a couple Result and
            // Option wrappings could be removed but this is cleaner)
            Some(_) => {
                match field(&mut scanner) {
                    // valid next field
                    Ok(Some(next_field)) => fields.push(next_field),
                    // no parsable field at this location
                    Ok(None) => _ = scanner.pop(),
                    // parse error
                    Err(e) => return Err(e),
                }
            }
        }
    }
}

// there has GOT to be a better way to do this
fn field(scanner: &mut Scanner) -> Result<Option<Field>, LexError> {
    match scanner.peek() {
        Some('M') => {
            scanner.pop();
            match scanner.scan_numeric() {
                Some(val) => Ok(Some(Field::M(val as u32))),
                _ => Err(LexError::NonNumericNextToken)
            }
        },
        Some('G') => {
            scanner.pop();
            match scanner.scan_numeric() {
                Some(val) => Ok(Some(Field::G(val as u32))),
                _ => Err(LexError::NonNumericNextToken)
            }
        },
        Some('X') => {
            scanner.pop();
            match scanner.scan_numeric() {
                // (see Field below for what this horseshit is)
                Some(val) => Ok(Some(Field::X(Some(FieldValue::Numeric(val))))),
                None => Ok(Some(Field::X(None))),
            }
        },
        Some('Y') => {
            scanner.pop();
            match scanner.scan_numeric() {
                Some(val) => Ok(Some(Field::Y(Some(FieldValue::Numeric(val))))),
                None => Ok(Some(Field::Y(None))),
            }
        },
        Some('Z') => {
            scanner.pop();
            match scanner.scan_numeric() {
                Some(val) => Ok(Some(Field::Z(Some(FieldValue::Numeric(val))))),
                None => Ok(Some(Field::Z(None))),
            }
        }
        _ => Ok(None)
    }
}

struct Scanner {
    cursor: usize,
    characters: Vec<char>
}

#[allow(dead_code)]
impl Scanner {
        pub fn new(string: &str) -> Self {
            Self {
                cursor: 0,
                characters: string.chars().collect(),
            }
        }

        pub fn cursor(&self) -> usize {
            self.cursor
        }

        pub fn peek(&self) -> Option<&char> {
            self.characters.get(self.cursor)
        }

        pub fn is_done(&self) -> bool {
            self.cursor == self.characters.len()
        }

        pub fn pop(&mut self) -> Option<&char> {
            match self.characters.get(self.cursor) {
                Some(character) => {
                    self.cursor += 1;
                    Some(character)
                }
                None => None,
            }
        }

        pub fn scan_numeric(&mut self) -> Option<f32> {
            let mut sequence = String::new();

            loop {
                match self.characters.get(self.cursor) {
                    Some(character) => {
                        // if the current character is part of a numeric,
                        // ingest it and move the cursor forward
                        if character.is_numeric() || character == &'.'{
                            sequence.push(*character);
                            self.cursor += 1;
                        } else {
                            // otherwise parse the ingested characters
                            // (if there are any)
                            break if !sequence.is_empty() {
                                Some(sequence.parse::<f32>().unwrap())
                            } else { None }
                        }
                    }
                    None => break None
                }
            }
        }
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum LexError {
    Generic,
    NonNumericNextToken
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum Field {
    G (u32), // u32 to make the whole process a little easier
    M (u32),
    // ok so there's a lot of Option<FieldValue> in here but that's
    // because a lot of these fields have variable value types. i'm
    // not particularily happy about this, but it seemed easier than
    // a struct at the time. sorry
    T (Option<FieldValue>),
    S (Option<FieldValue>),
    P (Option<FieldValue>),
    X (Option<FieldValue>),
    Y (Option<FieldValue>),
    Z (Option<FieldValue>),
    U (Option<FieldValue>),
    V (Option<FieldValue>),
    W (Option<FieldValue>),
    I (Option<FieldValue>),
    J (Option<FieldValue>),
    D (Option<FieldValue>),
    H (Option<FieldValue>),
    F (Option<FieldValue>),
    R (Option<FieldValue>),
    Q (Option<FieldValue>),
    E (Option<FieldValue>),
    N (Option<FieldValue>),
    Ast (u32),
}

// TODO: nicer debug printing would be nice but im not going
// to do it if i need to have a case for every fucking variant
/*
impl std::fmt::Debug for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self {
            Field (value) => {}
        }
        write!(f, "Field: {:?}{:?}", self., self.arg_v)
    }
}
*/

#[allow(dead_code)]
#[derive(Debug)]
pub enum FieldValue {
    Numeric (f32),
    String (String),
    Expr (String),
}

// TODO: same story with debug printing here as with Field
/*
impl std::fmt::Debug for FieldValue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.val_t {
            FieldValueType::Numeric (val) => write!(f, "{:?}(n)", self.val_n.unwrap()),
            FieldValueType::String => write!(f, "\"{:?}\"(s)", self.val_s),
            FieldValueType::Expr => write!(f, "{{{:?}}}(e)", self.val_s),
        }
    }
}
*/
