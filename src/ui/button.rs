use iced::{border::Radius, widget::button, Background, Border, Color, Element, Vector};

use super::UI;

impl UI {
    fn _default_button_appearance(&self) -> button::Appearance {
        button::Appearance {
            background: Some(Background::from(Self::COLOR_STYLES.accent.color7)),
            text_color: Self::COLOR_STYLES.base.base1,
            border: Border::with_radius(1),
            ..Default::default()
        }
    }
}

// todo add 2 button style!!
impl button::StyleSheet for UI {
    type Style = iced::Theme;

    fn active(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            ..self._default_button_appearance()
        }
    }

    fn disabled(&self, style: &Self::Style) -> button::Appearance {
        let active = self.active(style);

        button::Appearance {
            shadow_offset: Vector::default(),
            background: active.background.map(|background| match background {
                Background::Color(color) => Background::Color(Color { a: 0.4, ..color }),
                Background::Gradient(gradient) => Background::Gradient(gradient.mul_alpha(0.4)),
            }),
            text_color: Color {
                a: 0.4,
                ..active.text_color
            },
            ..active
        }
    }

    fn hovered(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(Background::from(Self::COLOR_STYLES.accent.color6)),
            ..self._default_button_appearance()
        }
    }

    fn pressed(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            ..self._default_button_appearance()
        }
    }
}

struct Secondary {}

impl Secondary {
    fn _default_button_appearance(&self) -> button::Appearance {
        button::Appearance {
            background: Some(Background::from(UI::COLOR_STYLES.base.base15)),
            text_color: UI::COLOR_STYLES.base.base1,
            border: Border::with_radius(1),
            ..Default::default()
        }
    }
}

impl button::StyleSheet for Secondary {
    type Style = iced::Theme;

    fn active(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            ..self._default_button_appearance()
        }
    }

    fn disabled(&self, style: &Self::Style) -> button::Appearance {
        let active = self.active(style);

        button::Appearance {
            shadow_offset: Vector::default(),
            background: active.background.map(|background| match background {
                Background::Color(color) => Background::Color(Color { a: 0.4, ..color }),
                Background::Gradient(gradient) => Background::Gradient(gradient.mul_alpha(0.4)),
            }),
            text_color: Color {
                a: 0.4,
                ..active.text_color
            },
            ..active
        }
    }

    fn hovered(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(Background::from(UI::COLOR_STYLES.base.base13)),
            ..self._default_button_appearance()
        }
    }

    fn pressed(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            ..self._default_button_appearance()
        }
    }
}

struct Small {}
impl Small {
    fn _default_button_appearance(&self) -> button::Appearance {
        button::Appearance {
            background: Some(Background::from(UI::COLOR_STYLES.base.base20)),
            text_color: UI::COLOR_STYLES.base.base3,
            border: Border {
                color: UI::COLOR_STYLES.base.base15,
                width: 1.0,
                radius: Radius::from(1.0),
            },
            ..Default::default()
        }
    }
}

impl button::StyleSheet for Small {
    type Style = iced::Theme;

    fn active(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            ..self._default_button_appearance()
        }
    }

    fn disabled(&self, style: &Self::Style) -> button::Appearance {
        let active = self.active(style);

        button::Appearance {
            shadow_offset: Vector::default(),
            background: active.background.map(|background| match background {
                Background::Color(color) => Background::Color(Color { a: 0.4, ..color }),
                Background::Gradient(gradient) => Background::Gradient(gradient.mul_alpha(0.4)),
            }),
            text_color: Color {
                a: 0.4,
                ..active.text_color
            },
            ..active
        }
    }

    fn hovered(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(Background::from(UI::COLOR_STYLES.base.base17)),
            ..self._default_button_appearance()
        }
    }

    fn pressed(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            ..self._default_button_appearance()
        }
    }
}

impl UI {
    pub fn button<'a, B>(&self, content: impl Into<Element<'a, B>>) -> iced::widget::Button<'a, B> {
        button(content)
            .padding([6, 12])
            .style(iced::theme::Button::Custom(Box::new(*self)))
    }

    pub fn button_secondary<'a, B>(
        &self,
        content: impl Into<Element<'a, B>>,
    ) -> iced::widget::Button<'a, B> {
        button(content)
            .padding([6, 12])
            .style(iced::theme::Button::Custom(Box::new(Secondary {})))
    }

    pub fn button_small<'a, B>(
        &self,
        content: impl Into<Element<'a, B>>,
    ) -> iced::widget::Button<'a, B> {
        button(content)
            .padding([4.5, 12.0])
            .style(iced::theme::Button::Custom(Box::new(Small {})))
    }
}
