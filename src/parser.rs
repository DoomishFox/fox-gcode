use super::GCommand;
use super::lexer::{Field, FieldValue};

pub fn parse(fields: Vec::<Field>) -> Result<GCommand, ParseError> {
    let mut field_iter = fields.iter();

    match field_iter.next() {
        Some(Field::G(0)) => {
            let mut x: Option<f32> = None;
            let mut y: Option<f32> = None;
            let mut z: Option<f32> = None;
            loop {
                match field_iter.next() {
                    Some(Field::X(Some(FieldValue::Numeric(val)))) => x = Some(*val),
                    Some(Field::Y(Some(FieldValue::Numeric(val)))) => y = Some(*val),
                    Some(Field::Z(Some(FieldValue::Numeric(val)))) => z = Some(*val),
                    Some(_) => continue,
                    None => break Ok(GCommand::G0 {x: x, y: y, z: z, e: None, f: None, s: None}),
                }
            }
        },
        Some(Field::G(1)) => {
            let mut x: Option<f32> = None;
            let mut y: Option<f32> = None;
            let mut z: Option<f32> = None;
            loop {
                match field_iter.next() {
                    Some(Field::X(Some(FieldValue::Numeric(val)))) => x = Some(*val),
                    Some(Field::Y(Some(FieldValue::Numeric(val)))) => y = Some(*val),
                    Some(Field::Z(Some(FieldValue::Numeric(val)))) => z = Some(*val),
                    Some(_) => continue,
                    None => break Ok(GCommand::G1 {x: x, y: y, z: z, e: None, f: None, s: None}),
                }
            }
        },
        Some(Field::G(2)) => {
            let mut x: Option<f32> = None;
            let mut y: Option<f32> = None;
            loop {
                match field_iter.next() {
                    Some(Field::X(Some(FieldValue::Numeric(val)))) => x = Some(*val),
                    Some(Field::Y(Some(FieldValue::Numeric(val)))) => y = Some(*val),
                    Some(_) => continue,
                    None => break Ok(GCommand::G2 {x: x, y: y, i: None, j: None, e: None, f: None}),
                }
            }
        },
        Some(Field::G(3)) => {
            let mut x: Option<f32> = None;
            let mut y: Option<f32> = None;
            loop {
                match field_iter.next() {
                    Some(Field::X(Some(FieldValue::Numeric(val)))) => x = Some(*val),
                    Some(Field::Y(Some(FieldValue::Numeric(val)))) => y = Some(*val),
                    Some(_) => continue,
                    None => break Ok(GCommand::G3 {x: x, y: y, i: None, j: None, e: None, f: None}),
                }
            }
        },
        Some(Field::G(28)) => {
            let mut x = false;
            let mut y = false;
            let mut z = false;
            loop {
                match field_iter.next() {
                    Some(Field::X(None)) => x = true,
                    Some(Field::Y(None)) => y = true,
                    Some(Field::Z(None)) => z = true,
                    Some(_) => continue,
                    None => break Ok(GCommand::G28 {x: x, y: y, z: z}),
                }
            }
        },
        Some(Field::G(90)) => Ok(GCommand::G90),
        Some(Field::G(91)) => Ok(GCommand::G91),
        Some(_) => Err(ParseError::UnsupportedCommand),
        None => Err(ParseError::Generic)
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum ParseError {
    Generic,
    UnsupportedCommand,
    MissingRequiredField,
}
