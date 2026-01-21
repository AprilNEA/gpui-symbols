//! Basic example showing SF Symbols in a GPUI application.
//!
//! This example demonstrates two ways to use SF Symbols:
//! 1. Using the `Icon` component (like GPUI Components)
//! 2. Using the `SfSymbol` builder for raw image rendering

use gpui::prelude::*;
use gpui::{
    App, Application, Bounds, Context, SharedString, TitlebarOptions, Window, WindowBounds,
    WindowOptions, div, px, rgb, size,
};
use gpui_symbols::{Icon, define_icons};

// Define your own icon enum using the macro
define_icons! {
    pub enum AppIcon {
        Settings => "gearshape",
        Favorite => "star.fill",
        Like => "heart.fill",
        Search => "magnifyingglass",
        Home => "house.fill",
        User => "person.fill",
        Folder => "folder.fill",
        Delete => "trash",
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
                    None,
                    size(px(600.), px(500.)),
                    cx,
                ))),
                titlebar: Some(TitlebarOptions {
                    title: Some(SharedString::from("SF Symbols in GPUI")),
                    ..Default::default()
                }),
                ..Default::default()
            },
            |_, cx| cx.new(|_| SfSymbolsView),
        )
        .unwrap();
    });
}

struct SfSymbolsView;

impl Render for SfSymbolsView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(rgb(0xffffff))
            .p_4()
            .gap_6()
            // Title
            .child(
                div()
                    .text_xl()
                    .font_weight(gpui::FontWeight::BOLD)
                    .child("SF Symbols in GPUI"),
            )
            // Section 1: Using Icon component with strings
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .child(div().text_base().child("Using Icon::new() with strings:"))
                    .child(
                        div()
                            .flex()
                            .flex_wrap()
                            .gap_4()
                            .child(IconCard::new("star.fill", "Favorite"))
                            .child(IconCard::new("heart.fill", "Like"))
                            .child(IconCard::new("gearshape", "Settings"))
                            .child(IconCard::new("magnifyingglass", "Search")),
                    ),
            )
            // Section 2: Using Icon component with custom enum
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .child(div().text_base().child("Using Icon::from_name() with enum:"))
                    .child(
                        div()
                            .flex()
                            .flex_wrap()
                            .gap_4()
                            .child(EnumIconCard::new(AppIcon::Home, "Home"))
                            .child(EnumIconCard::new(AppIcon::User, "User"))
                            .child(EnumIconCard::new(AppIcon::Folder, "Folder"))
                            .child(EnumIconCard::new(AppIcon::Delete, "Delete")),
                    ),
            )
            // Section 3: Colored icons
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .child(div().text_base().child("Colored icons with text_color():"))
                    .child(
                        div()
                            .flex()
                            .flex_wrap()
                            .gap_4()
                            .child(ColoredIconCard::new("heart.fill", 0xFF3B30, "Red"))
                            .child(ColoredIconCard::new("star.fill", 0xFFCC00, "Yellow"))
                            .child(ColoredIconCard::new("leaf.fill", 0x34C759, "Green"))
                            .child(ColoredIconCard::new("drop.fill", 0x007AFF, "Blue")),
                    ),
            )
    }
}

/// Icon card using Icon::new() with string name
#[derive(IntoElement)]
struct IconCard {
    name: &'static str,
    label: &'static str,
}

impl IconCard {
    fn new(name: &'static str, label: &'static str) -> Self {
        Self { name, label }
    }
}

impl RenderOnce for IconCard {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .items_center()
            .gap_2()
            .p_3()
            .rounded_lg()
            .bg(rgb(0xf5f5f5))
            .hover(|style| style.bg(rgb(0xe8e8e8)))
            .child(
                div()
                    .size(px(48.))
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(Icon::new(self.name).with_size(px(32.))),
            )
            .child(div().text_sm().text_color(rgb(0x666666)).child(self.label))
    }
}

/// Icon card using Icon::from_name() with enum
#[derive(IntoElement)]
struct EnumIconCard {
    icon: AppIcon,
    label: &'static str,
}

impl EnumIconCard {
    fn new(icon: AppIcon, label: &'static str) -> Self {
        Self { icon, label }
    }
}

impl RenderOnce for EnumIconCard {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .items_center()
            .gap_2()
            .p_3()
            .rounded_lg()
            .bg(rgb(0xf0f0ff))
            .hover(|style| style.bg(rgb(0xe0e0f0)))
            .child(
                div()
                    .size(px(48.))
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(Icon::from_name(self.icon).with_size(px(32.))),
            )
            .child(div().text_sm().text_color(rgb(0x666666)).child(self.label))
    }
}

/// Icon card with custom color
#[derive(IntoElement)]
struct ColoredIconCard {
    name: &'static str,
    color: u32,
    label: &'static str,
}

impl ColoredIconCard {
    fn new(name: &'static str, color: u32, label: &'static str) -> Self {
        Self { name, color, label }
    }
}

impl RenderOnce for ColoredIconCard {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .items_center()
            .gap_2()
            .p_3()
            .rounded_lg()
            .bg(rgb(0xf5f5f5))
            .hover(|style| style.bg(rgb(0xe8e8e8)))
            .child(
                div()
                    .size(px(48.))
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(
                        Icon::new(self.name)
                            .with_size(px(32.))
                            .text_color(self.color),
                    ),
            )
            .child(div().text_sm().text_color(rgb(0x666666)).child(self.label))
    }
}
