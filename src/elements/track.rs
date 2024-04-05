use gpui::*;
use std::sync::Arc;

use crate::on_right_click::ContextMenuItem;
use crate::theme;
use crate::RightClickEvent;
use crate::Track;
use crate::Tracks;
use crate::UiEvent;

pub fn track(index: usize, track: &Arc<Track>, cx: &mut ViewContext<Tracks>) -> impl IntoElement {
    let on_click = cx.listener({
        let track = Arc::clone(&track);
        move |_this, _event, cx| cx.emit(UiEvent::play(&track))
    });

    let on_right_click = cx.listener({
        let track = Arc::clone(track);
        move |_this, event: &MouseDownEvent, cx: &mut ViewContext<Tracks>| {
            cx.emit(Arc::new(UiEvent::RightClick(RightClickEvent {
                position: event.position,
                items: Arc::new(vec![
                    ContextMenuItem {
                        label: "Play",
                        event: UiEvent::play(&track),
                    },
                    ContextMenuItem {
                        label: "Queue",
                        event: UiEvent::queue(&track),
                    },
                ]),
            })));
        }
    });

    div()
        .id(ElementId::Name(track.title.clone().into()))
        .flex()
        .gap_3()
        .h_8()
        .items_center()
        .rounded(px(1.))
        .hover(|style| style.bg(rgb(theme::colours::TOUCH)))
        .child(
            div()
                .flex()
                .w_8()
                .justify_end()
                .text_color(rgb(theme::colours::YOUTH))
                .child((index + 1).to_string()),
        )
        .child(div().child(track.title.clone()))
        .on_click(on_click)
        .on_mouse_down(MouseButton::Right, on_right_click)
}
