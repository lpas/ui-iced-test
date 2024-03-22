use iced::{
    border::Radius,
    widget::{text_input, TextInput},
    Background, Border, Color,
};

use super::UI;

impl UI {
    fn _default_text_input_appearance(&self) -> text_input::Appearance {
        text_input::Appearance {
            background: Background::from(Self::COLOR_STYLES.base.base16),
            border: Border {
                color: Self::COLOR_STYLES.base.base13,
                width: 1.0,
                radius: Radius::from(1),
            },
            icon_color: Self::COLOR_STYLES.base.base3,
        }
    }
}

impl text_input::StyleSheet for UI {
    type Style = iced::Theme;

    fn active(&self, _style: &Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            ..self._default_text_input_appearance()
        }
    }

    fn focused(&self, _style: &Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            border: Border {
                color: Self::COLOR_STYLES.accent.color6,
                width: 1.0,
                radius: Radius::from(1),
            },
            ..self._default_text_input_appearance()
        }
    }

    fn placeholder_color(&self, _style: &Self::Style) -> Color {
        Self::COLOR_STYLES.base.base9
    }

    fn value_color(&self, _style: &Self::Style) -> Color {
        Self::COLOR_STYLES.base.base3
    }

    fn disabled_color(&self, _style: &Self::Style) -> Color {
        Self::COLOR_STYLES.base.base3 // todo
    }

    fn selection_color(&self, _style: &Self::Style) -> Color {
        // Color::from_rgb(0.25, 0.25, 0.25)
        Self::COLOR_STYLES.accent.color7
    }

    fn hovered(&self, _style: &Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            // border_color: self
            //     .accent_color
            //     .border_color(BorderColorVariant::HoveredGrayscale),
            ..self._default_text_input_appearance()
        }
    }

    fn disabled(&self, _style: &Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            // background: FILL_DISABLED,
            ..self._default_text_input_appearance()
        }
    }
}

impl UI {
    pub fn text_input<'a, MessageType: Clone>(
        &self,
        placeholder: &str,
        label: &str,
    ) -> TextInput<'a, MessageType> {
        text_input(placeholder, label)
            .padding([4, 8])
            .style(iced::theme::TextInput::Custom(Box::new(*self)))
    }
}
