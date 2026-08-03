//! A window bar widget for the application.

use iced::{
    Border, Element, Length, Renderer,
    alignment::Horizontal,
    border::Radius,
    widget::{MouseArea, button, container, mouse_area, row, space, text},
};

use crate::{Message, fonts::MATERIAL_SYMBOLS_FILLED_ROUNDED_FONT, theme::Theme};

/// Creates a window bar with the given content and menu.
pub fn window_bar<'a>(
    is_maximized: bool,
    menu: Option<impl Into<Element<'a, Message, Theme, Renderer>> + 'a>,
    content: Option<impl Into<Element<'a, Message, Theme, Renderer>> + 'a>,
) -> MouseArea<'a, Message, Theme, Renderer> {
    let content = content.map(|c| c.into());
    let menu = menu.map(|m| m.into()).unwrap_or_else(|| {
        container(space().width(Length::Shrink).height(Length::Shrink))
            .width(Length::Fill)
            .height(Length::Fill)
            .style(container::transparent)
            .into()
    });

    let center_slot = container(
        content.unwrap_or_else(|| space().width(Length::Shrink).height(Length::Shrink).into()),
    )
    .width(Length::Shrink)
    .height(Length::Fill)
    .align_x(Horizontal::Center);

    let buttons = row![
        button(text("\u{e931}").font(MATERIAL_SYMBOLS_FILLED_ROUNDED_FONT))
            .on_press(Message::Minimize),
        button(
            text(if is_maximized { "\u{f507}" } else { "\u{f830}" })
                .font(MATERIAL_SYMBOLS_FILLED_ROUNDED_FONT)
        )
        .on_press(Message::Maximize),
        button(text("\u{e5cd}").font(MATERIAL_SYMBOLS_FILLED_ROUNDED_FONT))
            .on_press(Message::Close)
            .style(move |theme: &Theme, status: button::Status| {
                match (theme, status) {
                    (Theme::Light, button::Status::Hovered | button::Status::Pressed) => {
                        button::Style {
                            background: Some(theme.palette().danger.into()),
                            border: Border {
                                width: 1.0,
                                radius: Radius::new(0).top_right(if is_maximized { 0 } else { 10 }),
                                color: theme.palette().danger,
                            },
                            ..Default::default()
                        }
                    }
                    (Theme::Dark, button::Status::Hovered | button::Status::Pressed) => {
                        button::Style {
                            background: Some(theme.palette().danger.into()),
                            border: Border {
                                width: 1.0,
                                radius: Radius::new(0).top_right(if is_maximized { 0 } else { 10 }),
                                color: theme.palette().danger,
                            },
                            ..Default::default()
                        }
                    }
                    _ => button::Style {
                        border: Border {
                            width: 1.0,
                            radius: Radius::new(0).top_right(if is_maximized { 0 } else { 10 }),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                }
            }),
    ];

    let right_slot = container(buttons)
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(Horizontal::Right)
        .style(move |_: &Theme| container::Style {
            border: Border {
                width: 1.0,
                radius: Radius::new(0).top_right(if is_maximized { 0 } else { 10 }),
                ..Default::default()
            },
            ..Default::default()
        });

    mouse_area(row![menu, center_slot, right_slot].height(30))
        .on_double_click(Message::Maximize)
        .on_press(Message::Drag)
}
