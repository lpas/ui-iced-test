use iced::Pixels;

pub struct Label {
    pub l1: Pixels,
}

impl Label {
    pub const DEFAULT: Self = Self { l1: Pixels(14.) };
}

pub struct Body {
    pub b1: Pixels,
    pub b2: Pixels,
    pub b3: Pixels,
}

impl Body {
    pub const DEFAULT: Self = Self {
        b1: Pixels(13.),
        b2: Pixels(12.),
        b3: Pixels(11.),
    };
}

pub struct Heading {
    pub h1: Pixels,
    pub h3: Pixels,
}

impl Heading {
    pub const DEFAULT: Self = Self {
        h1: Pixels(26.),
        h3: Pixels(12.),
    };
}

pub struct TextStyles {
    pub label: Label,
    pub body: Body,
    pub heading: Heading,
}

impl TextStyles {
    pub const DEFAULT: Self = Self {
        label: Label::DEFAULT,
        body: Body::DEFAULT,
        heading: Heading::DEFAULT,
    };
}
