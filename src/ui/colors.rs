use iced::{color, Color};

pub struct Accents {
    pub color1: Color,
    pub color2: Color,
    pub color3: Color,
    pub color4: Color,
    pub color5: Color,
    pub color6: Color,
    pub color7: Color,
    pub color8: Color,
    pub color9: Color,
    pub color10: Color,
    pub color11: Color,
    pub danger: Color,
    pub warning: Color,
}

impl Accents {
    pub const DEFAULT: Self = Self {
        color1: color!(0x75BEFF),
        color2: color!(0x40A6FF),
        color3: color!(0x3399CC),
        color4: color!(0x3794FF),
        color5: color!(0x0097FB),
        color6: color!(0x007ACC),
        color7: color!(0x0E639C),
        color8: color!(0x264F78),
        color9: color!(0x094771),
        color10: color!(0x062F4A),
        color11: color!(0x001F33),
        danger: color!(0xDC3545),
        warning: color!(0xFFC107),
    };
}

pub struct Base {
    pub base1: Color,
    pub base2: Color,
    pub base3: Color,
    pub base4: Color,
    pub base5: Color,
    pub base6: Color,
    pub base7: Color,
    pub base8: Color,
    pub base9: Color,
    pub base10: Color,
    pub base11: Color,
    pub base12: Color,
    pub base13: Color,
    pub base14: Color,
    pub base15: Color,
    pub base16: Color,
    pub base17: Color,
    pub base18: Color,
    pub base19: Color,
    pub base20: Color,
    pub base21: Color,
}

impl Base {
    pub const DEFAULT: Self = Self {
        base1: color!(0xFFFFFF),
        base2: color!(0xF0F0F0),
        base3: color!(0xE7E7E7),
        base4: color!(0xE5E5E5),
        base5: color!(0xD4D4D4),
        base6: color!(0xCCCCCC),
        base7: color!(0xC6C6C6),
        base8: color!(0xBBBBBB),
        base9: color!(0xA0A0A0),
        base10: color!(0x808080),
        base11: color!(0x7F7F7F),
        base12: color!(0x606060),
        base13: color!(0x454545),
        base14: color!(0x3C3C3C),
        base15: color!(0x3A3D41),
        base16: color!(0x333333),
        base17: color!(0x303031),
        base18: color!(0x292929),
        base19: color!(0x252526),
        base20: color!(0x1E1E1E),
        base21: color!(0x000000),
    };
}

pub struct ColorStyles {
    pub accent: Accents,
    pub base: Base,
}

impl ColorStyles {
    pub const DEFAULT: Self = Self {
        accent: Accents::DEFAULT,
        base: Base::DEFAULT,
    };
}
