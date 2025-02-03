use gpui::{
    prelude::FluentBuilder, App, Corners, Div, Edges, IntoElement, ParentElement, RenderOnce,
    Window,
};

use crate::{
    button::{Button, ButtonVariant, ButtonVariants},
    h_flex, Sizable, Size,
};

#[derive(IntoElement)]
pub struct Segmented {
    base: Div,
    items: Vec<Button>,

    // The button props
    compact: Option<bool>,
    variant: Option<ButtonVariant>,
    size: Option<Size>,
}

impl Segmented {
    pub fn new() -> Self {
        Self {
            base: h_flex(),
            items: Vec::new(),
            compact: None,
            variant: None,
            size: None,
        }
    }

    pub fn child(mut self, child: Button) -> Self {
        self.items.push(child);
        self
    }
}

impl RenderOnce for Segmented {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let items_len = self.items.len();

        self.base.children({
            self.items
                .into_iter()
                .enumerate()
                .map(|(child_index, child)| {
                    let child = if items_len == 1 {
                        child
                    } else if child_index == 0 {
                        // First
                        child
                            .border_corners(Corners {
                                top_left: true,
                                top_right: false,
                                bottom_left: true,
                                bottom_right: false,
                            })
                            .border_edges(Edges {
                                left: true,
                                top: true,
                                right: true,
                                bottom: true,
                            })
                    } else if child_index == items_len - 1 {
                        // Last
                        child
                            .border_edges(Edges {
                                left: false,
                                top: true,
                                right: true,
                                bottom: true,
                            })
                            .border_corners(Corners {
                                top_left: false,
                                top_right: true,
                                bottom_left: false,
                                bottom_right: true,
                            })
                    } else {
                        // Middle
                        child
                            .border_corners(Corners::all(false))
                            .border_edges(Edges {
                                left: false,
                                top: true,
                                right: true,
                                bottom: true,
                            })
                    }
                    .stop_propagation(false)
                    .when_some(self.size, |this, size| this.with_size(size))
                    .when_some(self.variant, |this, variant| this.with_variant(variant))
                    .when_some(self.compact, |this, _| this.compact());

                    child
                })
        })
    }
}
