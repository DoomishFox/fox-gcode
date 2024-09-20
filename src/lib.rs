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
    // dwell
    G4 {
        p: Option<f32>,
        s: Option<f32>,
    },
    // direct stepper move
    G6 {
        a: Option<f32>,
        b: Option<f32>,
        c: Option<f32>,
        r: bool,
    },
    // retract
    // technically, reprap will recognize G10 as a set tool offset
    // command if the P parameter is present. i think this is a dumb
    // idea so im ignoring it.
    G10 {
        s: Option<f32>,
    },
    // unretract
    G11 {
        s: Option<f32>,
    },
    // clean tool
    G12 {
        p: Option<u32>,
        s: Option<f32>,
        t: Option<u32>,
        e: Option<u32>,
    },
    // G17..19 are CNC specific and i dont want to deal with them.
    // set units to inches
    G20,
    // set units to mm
    G21,
    // firmware retract
    // mostly unsupported
    G22,
    // firmware recover
    // mostly unsupported
    G23,
    // mesh validation pattern
    // marlin only
    G26 {
        b: Option<f32>,
        c: bool,
        d: bool,
        h: Option<f32>, // defaults to 205C
        f: Option<f32>, // defaults to 1.75mm
        k: bool,
        l: Option<f32>, // defaults to .20mm
        o: Option<f32>, // defaults to .3mm
        p: Option<f32>,
        q: Option<f32>,
        r: Option<u32>,
        s: Option<f32>, // defaults to .4mm
        //u: uhhhhh???
        x: Option<f32>,
        y: Option<f32>,
    },
    // park toolhead
    // marlin only
    G27 {
        p: Option<u32>,
    },
    // move to origin
    G28 {
        x: bool,
        y: bool,
        z: bool,
    },
    // detailed z-probe
    // every single fucking firmware has a different goddamn set
    // of parameters for this stupid command and that's not even
    // including machinekit which includes *fucking subcommands
    // for some goddamn reason*.
    G29,
    // single z-probe
    G30 {
        p: Option<u32>,
        x: Option<f32>,
        y: Option<f32>,
        z: Option<f32>,
        h: Option<f32>,
        s: Option<f32>,
    },
    // dock probe sled
    // another multiple implementation type command, im going
    // with marlin.
    G31,
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
