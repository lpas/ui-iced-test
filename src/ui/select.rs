use std::{borrow::Borrow, rc::Rc};

use iced::{
    border::Radius,
    overlay::menu,
    widget::{pick_list, text::LineHeight, PickList},
    Background, Border, Length, Pixels,
};

use super::UI;

impl UI {
    fn _default_select_appearance(&self) -> pick_list::Appearance {
        pick_list::Appearance {
            text_color: Self::COLOR_STYLES.base.base3,
            placeholder_color: Self::COLOR_STYLES.base.base9,
            handle_color: Self::COLOR_STYLES.base.base3,
            background: Background::from(Self::COLOR_STYLES.base.base16),
            border: Border {
                color: Self::COLOR_STYLES.base.base13,
                width: 1.0,
                radius: Radius::from(1),
            },
        }
    }
}

impl pick_list::StyleSheet for UI {
    type Style = iced::Theme;

    fn active(&self, _style: &<Self as pick_list::StyleSheet>::Style) -> pick_list::Appearance {
        pick_list::Appearance {
            border: Border {
                color: Self::COLOR_STYLES.accent.color6,
                width: 1.0,
                radius: Radius::from([1.0, 1.0, 0.0, 0.0]),
            },
            ..self._default_select_appearance()
        }
    }

    fn hovered(&self, _style: &<Self as pick_list::StyleSheet>::Style) -> pick_list::Appearance {
        pick_list::Appearance {
            ..self._default_select_appearance()
        }
    }
}

impl menu::StyleSheet for UI {
    type Style = iced::Theme;

    fn appearance(&self, _style: &Self::Style) -> menu::Appearance {
        menu::Appearance {
            text_color: Self::COLOR_STYLES.base.base3,
            background: Background::from(Self::COLOR_STYLES.base.base16),
            border: Border {
                color: Self::COLOR_STYLES.accent.color6,
                width: 1.0,
                radius: Radius::from([0.0, 0.0, 1.0, 1.0]),
            },
            selected_text_color: Self::COLOR_STYLES.base.base1,
            selected_background: Background::from(Self::COLOR_STYLES.accent.color7),
        }
    }
}

impl UI {
    pub fn select<'a, T, L, V, Message>(
        &self,
        options: L,
        selected: Option<V>,
        on_selected: impl Fn(T) -> Message + 'a,
    ) -> PickList<'a, T, L, V, Message>
    where
        T: ToString + PartialEq + Clone + 'a,
        L: Borrow<[T]> + 'a,
        V: Borrow<T> + 'a,
        Message: Clone,
    {
        pick_list(options, selected, on_selected)
            .width(Length::Fill)
            .text_size(UI::TEXT_STYLES.body.b1)
            .text_line_height(LineHeight::Absolute(Pixels::from(16))) // todo configure this
            .style(iced::theme::PickList::Custom(
                Rc::new(*self),
                Rc::new(*self),
            ))
            .padding([4, 8])
    }
}
