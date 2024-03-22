mod button;
mod colors;
mod select;
mod text;
mod text_input;
mod texts;

use self::{colors::ColorStyles, texts::TextStyles};

#[derive(Clone, Copy)]
pub struct UI {}

impl UI {
    pub const COLOR_STYLES: ColorStyles = ColorStyles::DEFAULT;
    pub const TEXT_STYLES: TextStyles = TextStyles::DEFAULT;
}
