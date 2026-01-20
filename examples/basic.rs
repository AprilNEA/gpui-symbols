//! Basic example showing SF Symbols in a GPUI application.

use std::sync::Arc;

use gpui::prelude::*;
use gpui::{
    App, Application, Bounds, Context, ImageSource, RenderImage, SharedString, TitlebarOptions,
    Window, WindowBounds, WindowOptions, div, img, px, rgb, size,
};
use gpui_symbols::SfSymbol;

fn main() {
    Application::new().run(|cx: &mut App| {
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
                    None,
                    size(px(600.), px(400.)),
                    cx,
                ))),
                titlebar: Some(TitlebarOptions {
                    title: Some(SharedString::from("SF Symbols in GPUI")),
                    ..Default::default()
                }),
                ..Default::default()
            },
            |_, cx| cx.new(|_| SfSymbolsView::new()),
        )
        .unwrap();
    });
}

struct SfSymbolsView {
    symbols: Vec<SymbolInfo>,
}

struct SymbolInfo {
    name: &'static str,
    label: &'static str,
    image: Option<Arc<RenderImage>>,
}

impl SfSymbolsView {
    fn new() -> Self {
        let symbol_defs = vec![
            ("gearshape", "Settings"),
            ("star.fill", "Favorite"),
            ("heart.fill", "Like"),
            ("magnifyingglass", "Search"),
            ("house.fill", "Home"),
            ("person.fill", "User"),
            ("folder.fill", "Folder"),
            ("trash", "Delete"),
        ];

        let symbols = symbol_defs
            .into_iter()
            .map(|(name, label)| SymbolInfo {
                name,
                label,
                image: SfSymbol::new(name).size(32.0).render(),
            })
            .collect();

        Self { symbols }
    }
}

impl Render for SfSymbolsView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(rgb(0xffffff))
            .p_4()
            .gap_4()
            .child(
                div()
                    .text_xl()
                    .font_weight(gpui::FontWeight::BOLD)
                    .child("SF Symbols in GPUI"),
            )
            .child(
                div()
                    .flex()
                    .flex_wrap()
                    .gap_4()
                    .children(self.symbols.iter().map(|info| {
                        SymbolCard::new(info.name, info.label, info.image.clone())
                    })),
            )
    }
}

#[derive(IntoElement)]
struct SymbolCard {
    #[allow(dead_code)]
    name: &'static str,
    label: &'static str,
    image: Option<Arc<RenderImage>>,
}

impl SymbolCard {
    fn new(name: &'static str, label: &'static str, image: Option<Arc<RenderImage>>) -> Self {
        Self { name, label, image }
    }
}

impl RenderOnce for SymbolCard {
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
                    .when_some(self.image, |this, img_src| {
                        this.child(img(ImageSource::Render(img_src)).size(px(32.)))
                    }),
            )
            .child(
                div()
                    .text_sm()
                    .text_color(rgb(0x666666))
                    .child(self.label),
            )
    }
}
