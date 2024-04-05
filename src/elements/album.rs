use gpui::*;
use std::sync::Arc;

use crate::Album;
use crate::Albums;
use crate::UiEvent;

pub fn album(album: &Arc<Album>, cx: &mut ViewContext<Albums>) -> impl IntoElement {
    let element = div()
        .id(ElementId::Name(
            format!("{}{}", &album.artist_name, &album.title).into(),
        ))
        .size_64();

    let element = if let Some(artwork) = &album.artwork {
        element.child(img(Arc::clone(artwork)).rounded(px(1.)).size_full())
    } else {
        element
    };

    element.on_click({
        let album = Arc::clone(album);
        cx.listener(move |_this, _event, cx| {
            cx.emit(UiEvent::album(&album));
        })
    })
}
