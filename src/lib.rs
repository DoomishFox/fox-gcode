pub mod lexer;
pub mod parser;
//pub mod writer;

#[allow(dead_code)]
#[derive(Debug)]
pub enum GCommand {
    // rapid move
    G0 {
        x: Option<f32>,
        y: Option<f32>,
        z: Option<f32>,
        e: Option<f32>,
        f: Option<f32>,
        s: Option<f32>,
    },
    // linear move
    G1 {
        x: Option<f32>,
        y: Option<f32>,
        z: Option<f32>,
        e: Option<f32>,
        f: Option<f32>,
        s: Option<f32>,
    },
    // controlled arc move (clockwise)
    G2 {
        x: Option<f32>,
        y: Option<f32>,
        i: Option<f32>,
        j: Option<f32>,
        e: Option<f32>,
        f: Option<f32>,
    },
    // controlled arc move (counter-clockwise)
    G3 {
        x: Option<f32>,
        y: Option<f32>,
        i: Option<f32>,
        j: Option<f32>,
        e: Option<f32>,
        f: Option<f32>,
    },
    // direct stepper move
    G6 {
        a: Option<f32>,
        b: Option<f32>,
        c: Option<f32>,
        r: bool,
    },
    // move to origin
    G28 {
        x: bool,
        y: bool,
        z: bool,
    },
    // move to grid point
    G42 {
        i: Option<u32>,
        j: Option<u32>,
    },
    // set to absolute positioning
    G90,
    // set to relative positioning
    G91,
    // set virtual position
    G92 {
        x: Option<f32>,
        y: Option<f32>,
        z: Option<f32>,
    },
}
