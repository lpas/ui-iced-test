use iced::widget::text;

use super::UI;

impl UI {
    pub fn label<'a>(&self, label: impl ToString) -> iced::widget::Text<'a> {
        text(label)
            .size(UI::TEXT_STYLES.label.l1)
            .style(UI::COLOR_STYLES.base.base3)
    }

    pub fn text<'a>(&self, label: impl ToString) -> iced::widget::Text<'a> {
        text(label)
            .size(UI::TEXT_STYLES.body.b1)
            .style(UI::COLOR_STYLES.base.base3)
    }

    pub fn section_heading<'a>(&self, label: impl ToString) -> iced::widget::Text<'a> {
        text(label.to_string().to_uppercase())
            .size(UI::TEXT_STYLES.heading.h3)
            .style(UI::COLOR_STYLES.base.base3)
    }

    pub fn heading<'a>(&self, label: impl ToString) -> iced::widget::Text<'a> {
        text(label)
            .size(UI::TEXT_STYLES.heading.h1)
            .style(UI::COLOR_STYLES.base.base1)
    }
}
