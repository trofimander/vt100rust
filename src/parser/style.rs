use parser::codes::Code;
use parser::VtParser;

#[derive(Clone, Debug, PartialEq)]
pub enum Color {
    Default, Indexed(u32), Rgb{r:u32, g:u32, b:u32}
}

pub const BLACK: Color = Color::Indexed(0);
pub const WHITE: Color = Color::Indexed(15);

#[derive(Clone, Debug, PartialEq)]
pub enum Style {
    Bold,
    Italic,
    Blink,
    Dim,
    Inverse,
    Underlined,
    Hidden
}

fn color256(argv:&Vec<u32>, i:usize) -> (Option<Color>, u32) {
    let arg = |i: usize, default: i32| -> i32 {
        if i<argv.len() {argv[i] as i32} else {default}
    };

    match arg(i+1, 0) {
        2 => {
            let r = arg(i+2, -1);
            let g = arg(i+3, -1);
            let b = arg(i+4, -1);
            if (r>=0 && r<256) && (g>=0 && g<256) && (b>=0 && b<256) {
                (Some(Color::Rgb{r:r as u32, g:g as u32, b:b as u32}), 5)
            } else {
                (None, 5) //TODO: pass error message
            }

        },
        5 => (Some(Color::Indexed(arg(i+2, 0 as i32) as u32)), 3),
        _ => (None, 1)
    }

}

pub fn emit_style_codes(emu: &VtParser, argv:&Vec<u32>) {
    let mut i = 0;

    while i<argv.len() {
        let mut step: usize = 1;
        match argv[i] {
            0 => emu.emit(Code::DefaultStyle),
            1 => emu.emit(Code::StyleOption(Style::Bold, true)),
            2 => emu.emit(Code::StyleOption(Style::Dim, true)),
            3 => emu.emit(Code::StyleOption(Style::Italic, true)),
            4 => emu.emit(Code::StyleOption(Style::Underlined, true)),
            5 => emu.emit(Code::StyleOption(Style::Blink, true)),
            7 => emu.emit(Code::StyleOption(Style::Inverse, true)),
            8 => emu.emit(Code::StyleOption(Style::Hidden, true)),
            22 => {
                emu.emit(Code::StyleOption(Style::Bold, false));
                emu.emit(Code::StyleOption(Style::Dim, false))
            },
            23 => emu.emit(Code::StyleOption(Style::Italic, false)),
            24 => emu.emit(Code::StyleOption(Style::Underlined, false)),
            25 => emu.emit(Code::StyleOption(Style::Inverse, false)),
            28 => emu.emit(Code::StyleOption(Style::Hidden, false)),
            //Foreground
            i@30...37 => emu.emit(Code::Foreground(Color::Indexed(i - 30))),
            38 => {
                let (color, s) = color256(argv, i);
                step = s as usize;
                match color {
                    Some(c) => emu.emit(Code::Foreground(c)),
                    None => emu.error_msg("Bogus color setting".to_string())
                };
            },
            39 => emu.emit(Code::Foreground(Color::Default)),
            //Background
            i@40...47 => emu.emit(Code::Background(Color::Indexed(i-40))),
            48 => { // Set xterm-256 background color
                let (color, s) = color256(argv, i);
                step = s as usize;
                match color {
                    Some(c) => emu.emit(Code::Background(c)),
                    None => emu.error_msg("Bogus color setting".to_string())
                };
            },
            49 => emu.emit(Code::Background(Color::Default)),

            //Bright versions of the ISO colors for foreground
            i@90...97 => emu.emit(Code::Foreground(Color::Indexed(i - 82))),
            //Bright versions of the ISO colors for background
            i@100...107 => emu.emit(Code::Foreground(Color::Indexed(i - 92))),

            //TODO: handle iso colors
            _ => {}

        };

        i += step;



    }

}
