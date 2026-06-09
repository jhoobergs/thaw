use leptos::prelude::*;
use thaw_utils::{class_list};
#[cfg(feature = "runtime-css")]
use thaw_utils::mount_style;

#[component]
pub fn ButtonGroup(
    #[prop(optional, into)] class: MaybeProp<String>,
    /// Directions of buttons in the group.
    #[prop(optional)]
    vertical: bool,
    children: Children,
) -> impl IntoView {
    #[cfg(feature = "runtime-css")]
    mount_style("button-group", include_str!("./button-group.css"));
    view! {
        <div
            class=class_list!["thaw-button-group", class]
            class=("thaw-button-group--vertical", vertical)
        >
            {children()}
        </div>
    }
}
