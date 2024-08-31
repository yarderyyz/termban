use ndarray::prelude::arr2;
use ndarray::Array2;
use ratatui::style::Color;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct CharPixel {
    pub char: char,
    pub fg: Color,
    pub bg: Color,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Sprite {
    pub chars: Array2<CharPixel>,
}

#[allow(dead_code)]
const BOX_ELEMENTS: [char; 32] = [
    '▀', '▁', '▂', '▃', '▄', '▅', '▆', '▇', '█', '▉', '▊', '▋', '▌', '▍', '▎', '▏',
    '▐', '░', '▒', '▓', '▔', '▕', '▖', '▗', '▘', '▙', '▚', '▛', '▜', '▝', '▞', '▟',
];

#[allow(dead_code)]
pub fn get_player_sprite_8_simple() -> Sprite {
    let background = Color::Rgb(133, 133, 133);
    let hair1 = Color::Rgb(52, 32, 33);
    let hair2 = Color::Rgb(72, 42, 43);
    let skin = Color::Rgb(239, 204, 165);
    let eye = Color::Rgb(19, 19, 19);
    let cloth1 = Color::Rgb(8, 25, 61);
    let cloth2 = Color::Rgb(37, 75, 75);
    let cloth3 = Color::Rgb(56, 109, 82);
    #[rustfmt::skip]
    #[allow(clippy::cast_precision_loss)]
    let chars = arr2(
        &[
            [CharPixel {char: '▀', fg: Color::from_u32(0),      bg: background},
                CharPixel {char: '▀', fg: background,      bg: hair1},
                CharPixel {char: '▀', fg: hair1,           bg: hair2},
                CharPixel {char: '▀', fg: hair1,           bg: hair2},
                CharPixel {char: '▀', fg: hair1,           bg: hair2},
                CharPixel {char: '▀', fg: hair2,           bg: hair1},
                CharPixel {char: '▀', fg: background,      bg: hair1},
                CharPixel {char: '▀', fg: background,      bg: background},
            ],
            [CharPixel {char: '▀', fg: background,      bg: background},
                CharPixel {char: '▀', fg: hair1,           bg: background},
                CharPixel {char: '▀', fg: eye,             bg: skin},
                CharPixel {char: '▀', fg: skin,            bg: skin},
                CharPixel {char: '▀', fg: eye,             bg: skin},
                CharPixel {char: '▀', fg: hair1,           bg: skin},
                CharPixel {char: '▀', fg: hair1,           bg: background},
                CharPixel {char: '▀', fg: Color::from_u32(0),      bg: background},
            ],
            [CharPixel {char: '▀', fg: background,      bg: background},
                CharPixel {char: '▀', fg: background,      bg: cloth1},
                CharPixel {char: '▀', fg: cloth2,          bg: cloth3},
                CharPixel {char: '▀', fg: cloth2,          bg: cloth2},
                CharPixel {char: '▀', fg: cloth2,          bg: cloth2},
                CharPixel {char: '▀', fg: background,      bg: cloth2},
                CharPixel {char: '▀', fg: background,      bg: background},
                CharPixel {char: '▀', fg: background,      bg: background},
            ],
            [CharPixel {char: '▀', fg: background,      bg: background},
                CharPixel {char: '▀', fg: cloth1,          bg: background},
                CharPixel {char: '▀', fg: cloth1,          bg: cloth1},
                CharPixel {char: '▀', fg: cloth3,          bg: cloth3},
                CharPixel {char: '▀', fg: cloth2,          bg: cloth3},
                CharPixel {char: '▀', fg: cloth2,          bg: cloth3},
                CharPixel {char: '▀', fg: cloth2,          bg: cloth3},
                CharPixel {char: '▀', fg: background,      bg: background},
            ],

        ]
    );
    Sprite { chars }
}

#[allow(dead_code)]
pub fn get_player_sprite_8() -> Sprite {
    let background = Color::Rgb(133, 133, 133);
    let hair1 = Color::Rgb(52, 32, 33);
    let hair2 = Color::Rgb(72, 42, 43);
    let skin = Color::Rgb(239, 204, 165);
    let eye = Color::Rgb(19, 19, 19);
    let cloth1 = Color::Rgb(8, 25, 61);
    let cloth2 = Color::Rgb(37, 75, 75);
    let cloth3 = Color::Rgb(56, 109, 82);
    let cloth4 = Color::Rgb(45, 95, 65);
    let flower = Color::Rgb(255, 192, 203);
    #[rustfmt::skip]
    #[allow(clippy::cast_precision_loss)]
    let chars = arr2(
        &[
            [CharPixel {char: '@', fg: Color::from_u32(0),      bg: background},
                CharPixel {char: '▅', fg: background,      bg: hair1},
                CharPixel {char: '▚', fg: hair1,           bg: hair2},
                CharPixel {char: '▚', fg: hair1,           bg: hair2},
                CharPixel {char: '▚', fg: hair1,           bg: hair2},
                CharPixel {char: '▝', fg: hair2,           bg: hair1},
                CharPixel {char: '▀', fg: background,      bg: hair1},
                CharPixel {char: '▀', fg: background,      bg: background},
            ],
// (◠)
            [CharPixel {char: '▀', fg: background,      bg: background},
                CharPixel {char: '▂', fg: background,      bg: hair1},
                CharPixel {char: '◠', fg: eye,             bg: skin},
                CharPixel {char: '‿', fg: eye,            bg: skin},
                CharPixel {char: '◠', fg: eye,             bg: skin},
                CharPixel {char: '✿', fg: flower,            bg: hair1},
                CharPixel {char: '▀', fg: hair1,           bg: background},
                CharPixel {char: '@', fg: Color::from_u32(0),      bg: background},
            ],
            [CharPixel {char: '▀', fg: background,      bg: background},
                CharPixel {char: '▀', fg: background,      bg: cloth1},
                CharPixel {char: '▚', fg: cloth3,          bg: cloth2},
                CharPixel {char: '▀', fg: cloth2,          bg: cloth2},
                CharPixel {char: '▚', fg: cloth2,          bg: cloth4},
                CharPixel {char: '▖', fg: cloth2,          bg: background},
                CharPixel {char: '▀', fg: background,      bg: background},
                CharPixel {char: '▀', fg: background,      bg: background},
            ],
            [CharPixel {char: '▀', fg: background,      bg: background},
                CharPixel {char: '▝', fg: cloth1,          bg: background},
                CharPixel {char: '▙', fg: cloth1,          bg: cloth2},
                CharPixel {char: '▙', fg: cloth3,          bg: cloth2},
                CharPixel {char: '▀', fg: cloth2,          bg: cloth3},
                CharPixel {char: '▂', fg: cloth3,          bg: cloth2},
                CharPixel {char: '▂', fg: cloth3,          bg: background},
                CharPixel {char: '▀', fg: background,      bg: background},
            ],

        ]
    );
    Sprite { chars }
}

#[allow(dead_code)]
pub fn get_player_sprite_6() -> Sprite {
    let background = Color::Rgb(133, 133, 133);
    let hair1 = Color::Rgb(52, 32, 33);
    let hair2 = Color::Rgb(72, 42, 43);
    let skin = Color::Rgb(239, 204, 165);
    let cloth1 = Color::Rgb(8, 25, 61);
    let cloth2 = Color::Rgb(37, 75, 75);
    let cloth3 = Color::Rgb(56, 109, 82);
    #[rustfmt::skip]
    #[allow(clippy::cast_precision_loss)]
    let chars = arr2(
        &[
            [CharPixel {char: '@', fg: Color::from_u32(0),      bg: background},
                CharPixel {char: '▅', fg: background,      bg: hair1},
                CharPixel {char: '▚', fg: hair1,           bg: hair2},
                CharPixel {char: '▝', fg: hair2,           bg: hair1},
                CharPixel {char: '▀', fg: background,      bg: hair1},
                CharPixel {char: '▀', fg: background,      bg: background},
            ],
            [CharPixel {char: '▀', fg: background,      bg: background},
                CharPixel {char: '▂', fg: background,      bg: hair1},
                CharPixel {char: '▆', fg: skin,            bg: hair1},
                CharPixel {char: '▆', fg: skin,            bg: hair1},
                CharPixel {char: '▂', fg: skin,            bg: hair1},
                CharPixel {char: '▀', fg: hair1,           bg: background},
            ],
            [CharPixel {char: '▀', fg: background,      bg: background},
                CharPixel {char: '▝', fg: cloth1,          bg: background},
                CharPixel {char: '▚', fg: cloth3,          bg: cloth2},
                CharPixel {char: '▀', fg: cloth2,          bg: cloth2},
                CharPixel {char: '▙', fg: cloth2,          bg: background},
                CharPixel {char: '▖', fg: cloth2,      bg: background},
            ],
        ]
    );
    Sprite { chars }
}

#[allow(dead_code)]
pub fn get_player_sprite_4() -> Sprite {
    let background = Color::Rgb(133, 133, 133);
    let hair1 = Color::Rgb(52, 32, 33);
    let skin = Color::Rgb(239, 204, 165);
    let cloth1 = Color::Rgb(8, 25, 61);
    let cloth2 = Color::Rgb(37, 75, 75);
    let cloth3 = Color::Rgb(56, 109, 82);
    #[rustfmt::skip]
    #[allow(clippy::cast_precision_loss)]
    let chars = arr2(
        &[
            [CharPixel {char: '▝', fg: hair1,           bg: background},
                CharPixel {char: '▅', fg: skin,            bg: hair1},
                CharPixel {char: '▂', fg: skin,            bg: hair1},
                CharPixel {char: '▙', fg: hair1,           bg: background},
            ],
            [CharPixel {char: '▝', fg: cloth1,          bg: background},
                CharPixel {char: '▚', fg: cloth3,          bg: cloth2},
                CharPixel {char: '▀', fg: cloth2,          bg: cloth2},
                CharPixel {char: '▖', fg: cloth2,          bg: background},
            ],
        ]
    );
    Sprite { chars }
}
