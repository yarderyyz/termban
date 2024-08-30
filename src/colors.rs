/*!
 * Author: Lee Gauthier
 *
 * Description:
 *
 * Implementation of Paul Tol's color palettes, as presented in
 *
 *   https://personal.sron.nl/~pault/data/colourschemes.pdf
 *
 * Inspired by the implementation from https://gist.github.com/gipert/df72b67c1d02bbb41f1dd406b6397811
 *
 * Usage:
 *
 * Notes:
 *
 */

use ratatui::style::Color;
use std::collections::HashMap;
use std::sync::OnceLock;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TolColor {
    BriBlue,
    BriCyan,
    BriGreen,
    BriYellow,
    BriRed,
    BriPurple,
    BriGrey,

    VibBlue,
    VibCyan,
    VibTeal,
    VibOrange,
    VibRed,
    VibMagenta,
    VibGrey,

    MutIndigo,
    MutCyan,
    MutTeal,
    MutGreen,
    MutOlive,
    MutSand,
    MutRose,
    MutWine,
    MutPurple,
    MutPaleGrey,

    LigLightBlue,
    LigLightCyan,
    LigMint,
    LigPear,
    LigOlive,
    LigLightYellow,
    LigOrange,
    LigPink,
    LigPaleGrey,

    PurRed,
    PurGreen,
    PurBlue,
    PurYellow,
    PurCyan,
    PurMagenta,

    CstLigBlue,

    BadData,
}

// Create a static `Lazy` instance to hold the color map
static COLOR_MAP: OnceLock<HashMap<TolColor, Color>> = OnceLock::new();

fn build_color_map() -> HashMap<TolColor, Color> {
    let mut color_map = HashMap::new();

    // Bright color scheme
    color_map.insert(TolColor::BriBlue, Color::Rgb(68, 119, 170));
    color_map.insert(TolColor::BriCyan, Color::Rgb(102, 204, 238));
    color_map.insert(TolColor::BriGreen, Color::Rgb(34, 136, 51));
    color_map.insert(TolColor::BriYellow, Color::Rgb(204, 187, 68));
    color_map.insert(TolColor::BriRed, Color::Rgb(238, 102, 119));
    color_map.insert(TolColor::BriPurple, Color::Rgb(170, 51, 119));
    color_map.insert(TolColor::BriGrey, Color::Rgb(187, 187, 187));

    // Vibrant color scheme
    color_map.insert(TolColor::VibBlue, Color::Rgb(0, 119, 187));
    color_map.insert(TolColor::VibCyan, Color::Rgb(51, 187, 238));
    color_map.insert(TolColor::VibTeal, Color::Rgb(0, 153, 136));
    color_map.insert(TolColor::VibOrange, Color::Rgb(238, 119, 51));
    color_map.insert(TolColor::VibRed, Color::Rgb(204, 51, 17));
    color_map.insert(TolColor::VibMagenta, Color::Rgb(238, 51, 119));
    color_map.insert(TolColor::VibGrey, Color::Rgb(187, 187, 187));

    // Muted color scheme
    color_map.insert(TolColor::MutIndigo, Color::Rgb(51, 34, 136));
    color_map.insert(TolColor::MutCyan, Color::Rgb(136, 204, 238));
    color_map.insert(TolColor::MutTeal, Color::Rgb(68, 170, 153));
    color_map.insert(TolColor::MutGreen, Color::Rgb(17, 119, 51));
    color_map.insert(TolColor::MutOlive, Color::Rgb(153, 153, 51));
    color_map.insert(TolColor::MutSand, Color::Rgb(221, 204, 119));
    color_map.insert(TolColor::MutRose, Color::Rgb(204, 102, 119));
    color_map.insert(TolColor::MutWine, Color::Rgb(136, 34, 85));
    color_map.insert(TolColor::MutPurple, Color::Rgb(170, 68, 153));
    color_map.insert(TolColor::MutPaleGrey, Color::Rgb(221, 221, 221));

    // Light color scheme
    color_map.insert(TolColor::LigLightBlue, Color::Rgb(119, 170, 221));
    color_map.insert(TolColor::LigLightCyan, Color::Rgb(153, 221, 255));
    color_map.insert(TolColor::LigMint, Color::Rgb(68, 187, 153));
    color_map.insert(TolColor::LigPear, Color::Rgb(187, 204, 51));
    color_map.insert(TolColor::LigOlive, Color::Rgb(170, 170, 0));
    color_map.insert(TolColor::LigLightYellow, Color::Rgb(238, 221, 136));
    color_map.insert(TolColor::LigOrange, Color::Rgb(238, 136, 102));
    color_map.insert(TolColor::LigPink, Color::Rgb(255, 170, 187));
    color_map.insert(TolColor::LigPaleGrey, Color::Rgb(221, 221, 221));

    // Simple colors (Pure)
    color_map.insert(TolColor::PurRed, Color::Rgb(255, 0, 0));
    color_map.insert(TolColor::PurBlue, Color::Rgb(0, 255, 0));
    color_map.insert(TolColor::PurGreen, Color::Rgb(0, 0, 255));
    color_map.insert(TolColor::PurYellow, Color::Rgb(255, 255, 0));
    color_map.insert(TolColor::PurCyan, Color::Rgb(0, 255, 255));
    color_map.insert(TolColor::PurMagenta, Color::Rgb(255, 0, 255));

    // Custom Colors
    color_map.insert(TolColor::CstLigBlue, Color::Rgb(222, 255, 255));

    // Bad data color
    color_map.insert(TolColor::BadData, Color::Rgb(255, 238, 153));

    color_map
}

pub fn get_color(color: TolColor) -> Color {
    let color_map = COLOR_MAP.get_or_init(build_color_map);
    *color_map.get(&color).unwrap()
}
