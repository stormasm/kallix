use gpui::*;
use std::sync::Arc;

use crate::on_right_click::ContextMenuItem;
use crate::theme;
use crate::Album;
use crate::Albums;
use crate::RightClickEvent;
use crate::Track;
use crate::Tracks;
use crate::UiEvent;

pub struct UiAction {
    pub label: &'static str,
    pub event: Arc<UiEvent>,
}

pub fn tab_bar<V: EventEmitter<Arc<UiEvent>>>(
    tabs: Vec<UiAction>,
    selected: usize,
    cx: &mut ViewContext<V>,
) -> impl IntoElement {
    div()
        .flex()
        .children(tabs.into_iter().enumerate().map(|(index, item)| {
            let tab = div()
                .id(item.label)
                .flex_1()
                .py_1()
                .px_3()
                .flex()
                .justify_center()
                .child(item.label);

            if index == selected {
                tab.bg(rgb(theme::colours::AMSTERDAM))
                    .text_color(rgb(theme::colours::WINTER))
                    .rounded_t_sm()
            } else {
                tab.text_color(rgb(theme::colours::AMSTERDAM))
                    .rounded_t_sm()
                    .hover(|style| {
                        style
                            .rounded_sm()
                            .bg(rgb(theme::colours::YOUTH))
                            .text_color(rgb(theme::colours::SHALLOWS))
                    })
                    .on_click(cx.listener(move |_this, _event, cx| {
                        cx.emit(Arc::clone(&item.event));
                    }))
            }
        }))
}
