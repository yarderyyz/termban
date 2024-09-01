use ndarray::prelude::arr2;
use ndarray::Array2;
use ratatui::style::Color;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct CharPixel {
    pub char: char,
    pub fg: Option<Color>,
    pub bg: Option<Color>,
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
            [CharPixel {char: '▀', fg: Some(Color::from_u32(0)),      bg: None},
             CharPixel {char: '▀', fg: None,            bg: Some(hair1)},
             CharPixel {char: '▀', fg: Some(hair1),     bg: Some(hair2)},
             CharPixel {char: '▀', fg: Some(hair1),     bg: Some(hair2)},
             CharPixel {char: '▀', fg: Some(hair1),     bg: Some(hair2)},
             CharPixel {char: '▀', fg: Some(hair2),     bg: Some(hair1)},
             CharPixel {char: '▀', fg: None,            bg: Some(hair1)},
             CharPixel {char: '▀', fg: None,            bg: None},
            ],
            [CharPixel {char: '▀', fg: None,            bg: None},
             CharPixel {char: '▀', fg: Some(hair1),     bg: None},
             CharPixel {char: '▀', fg: Some(eye),       bg: Some(skin)},
             CharPixel {char: '▀', fg: Some(skin),      bg: Some(skin)},
             CharPixel {char: '▀', fg: Some(eye),       bg: Some(skin)},
             CharPixel {char: '▀', fg: Some(hair1),     bg: Some(skin)},
             CharPixel {char: '▀', fg: Some(hair1),     bg: None},
             CharPixel {char: '▀', fg: Some(Color::from_u32(0)), bg: None},
            ],
            [CharPixel {char: '▀', fg: None,        bg: None},
             CharPixel {char: '▀', fg: None,         bg: Some(cloth1)},
             CharPixel {char: '▀', fg: Some(cloth2), bg: Some(cloth3)},
             CharPixel {char: '▀', fg: Some(cloth2), bg: Some(cloth2)},
             CharPixel {char: '▀', fg: Some(cloth2), bg: Some(cloth2)},
             CharPixel {char: '▀', fg: None,         bg: Some(cloth2)},
             CharPixel {char: '▀', fg: None,         bg: None},
             CharPixel {char: '▀', fg: None,         bg: None},
            ],
            [CharPixel {char: '▀', fg: None,            bg: None},
             CharPixel {char: '▀', fg: Some(cloth1),    bg: None},
             CharPixel {char: '▀', fg: Some(cloth1),    bg: Some(cloth1)},
             CharPixel {char: '▀', fg: Some(cloth3),    bg: Some(cloth3)},
             CharPixel {char: '▀', fg: Some(cloth2),    bg: Some(cloth3)},
             CharPixel {char: '▀', fg: Some(cloth2),    bg: Some(cloth3)},
             CharPixel {char: '▀', fg: Some(cloth2),    bg: Some(cloth3)},
             CharPixel {char: '▀', fg: None,            bg: None},
            ],

        ]
    );
    Sprite { chars }
}

#[allow(dead_code)]
pub fn get_player_sprite_8() -> Sprite {
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
            [CharPixel {char: '@', fg: Some(Color::from_u32(0)),      bg: None},
             CharPixel {char: '▅', fg: None,         bg: Some(hair1)},
             CharPixel {char: '▚', fg: Some(hair1),  bg: Some(hair2)},
             CharPixel {char: '▚', fg: Some(hair1),  bg: Some(hair2)},
             CharPixel {char: '▚', fg: Some(hair1),  bg: Some(hair2)},
             CharPixel {char: '▝', fg: Some(hair2),  bg: Some(hair1)},
             CharPixel {char: '▀', fg: None,         bg: Some(hair1)},
             CharPixel {char: '▀', fg: None,         bg: None},
            ],
            [CharPixel {char: '▀', fg: None,            bg: None},
             CharPixel {char: '▂', fg: None,            bg: Some(hair1)},
             CharPixel {char: '◠', fg: Some(eye),       bg: Some(skin)},
             CharPixel {char: '‿', fg: Some(eye),       bg: Some(skin)},
             CharPixel {char: '◠', fg: Some(eye),       bg: Some(skin)},
             CharPixel {char: '✿', fg: Some(flower),    bg: Some(hair1)},
             CharPixel {char: '▀', fg: Some(hair1),     bg: None},
             CharPixel {char: '@', fg: Some(Color::from_u32(0)), bg: None},
            ],
            [CharPixel {char: '▀', fg: None,            bg: None},
             CharPixel {char: '▀', fg: None,            bg: Some(cloth1)},
             CharPixel {char: '▚', fg: Some(cloth3),    bg: Some(cloth2)},
             CharPixel {char: '▀', fg: Some(cloth2),    bg: Some(cloth2)},
             CharPixel {char: '▚', fg: Some(cloth2),    bg: Some(cloth4)},
             CharPixel {char: '▖', fg: Some(cloth2),    bg: None},
             CharPixel {char: '▀', fg: None,            bg: None},
             CharPixel {char: '▀', fg: None,            bg: None},
            ],
            [CharPixel {char: '▀', fg: None,                  bg: None},
             CharPixel {char: '▝', fg: Some(cloth1),          bg: None},
             CharPixel {char: '▙', fg: Some(cloth1),          bg: Some(cloth2)},
             CharPixel {char: '▙', fg: Some(cloth3),          bg: Some(cloth2)},
             CharPixel {char: '▀', fg: Some(cloth2),          bg: Some(cloth3)},
             CharPixel {char: '▂', fg: Some(cloth3),          bg: Some(cloth2)},
             CharPixel {char: '▂', fg: Some(cloth3),          bg: None},
             CharPixel {char: '▀', fg: None,                  bg: None},
            ],

        ]
    );
    Sprite { chars }
}

#[allow(dead_code)]
pub fn get_player_sprite_6() -> Sprite {
    let hair1 = Color::Rgb(52, 32, 33);
    let hair2 = Color::Rgb(72, 42, 42);
    let skin = Color::Rgb(239, 204, 165);
    let cloth1 = Color::Rgb(8, 25, 61);
    let cloth2 = Color::Rgb(37, 75, 75);
    let cloth3 = Color::Rgb(56, 109, 82);
    #[rustfmt::skip]
    #[allow(clippy::cast_precision_loss)]
    let chars = arr2(
        &[
            [CharPixel {char: '@', fg: Some(Color::from_u32(0)),      bg: None},
             CharPixel {char: '▅', fg: None,      bg: Some(hair1)},
             CharPixel {char: '▚', fg: Some(hair1),           bg: Some(hair2)},
             CharPixel {char: '▝', fg: Some(hair2),           bg: Some(hair1)},
             CharPixel {char: '▀', fg: None,      bg: Some(hair1)},
             CharPixel {char: '▀', fg: None,      bg: None},
            ],
            [CharPixel {char: '▀', fg: None,      bg: None},
             CharPixel {char: '▂', fg: None,      bg: Some(hair1)},
             CharPixel {char: '▆', fg: Some(skin),            bg: Some(hair1)},
             CharPixel {char: '▆', fg: Some(skin),            bg: Some(hair1)},
             CharPixel {char: '▂', fg: Some(skin),            bg: Some(hair1)},
             CharPixel {char: '▀', fg: Some(hair1),           bg: None},
            ],
            [CharPixel {char: '▀', fg: None,      bg: None},
             CharPixel {char: '▝', fg: Some(cloth1),          bg: None},
             CharPixel {char: '▚', fg: Some(cloth3),          bg: Some(cloth2)},
             CharPixel {char: '▀', fg: Some(cloth2),          bg: Some(cloth2)},
             CharPixel {char: '▙', fg: Some(cloth2),          bg: None},
             CharPixel {char: '▖', fg: Some(cloth2),          bg: None},
            ],
        ]
    );
    Sprite { chars }
}

#[allow(dead_code)]
pub fn get_player_sprite_4() -> Sprite {
    let hair1 = Color::Rgb(52, 32, 33);
    let skin = Color::Rgb(239, 204, 165);
    let cloth1 = Color::Rgb(8, 25, 61);
    let cloth2 = Color::Rgb(37, 75, 75);
    let cloth3 = Color::Rgb(56, 109, 82);
    #[rustfmt::skip]
    #[allow(clippy::cast_precision_loss)]
    let chars = arr2(
        &[
            [CharPixel {char: '▝', fg: Some(hair1),           bg: None},
             CharPixel {char: '▅', fg: Some(skin),            bg: Some(hair1)},
             CharPixel {char: '▂', fg: Some(skin),            bg: Some(hair1)},
             CharPixel {char: '▙', fg: Some(hair1),           bg: None},
            ],
            [CharPixel {char: '▝', fg: Some(cloth1),          bg: None},
             CharPixel {char: '▚', fg: Some(cloth3),          bg: Some(cloth2)},
             CharPixel {char: '▀', fg: Some(cloth2),          bg: Some(cloth2)},
             CharPixel {char: '▖', fg: Some(cloth2),          bg: None},
            ],
        ]
    );
    Sprite { chars }
}
