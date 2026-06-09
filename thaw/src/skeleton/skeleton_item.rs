use leptos::prelude::*;
use thaw_utils::{class_list};
#[cfg(feature = "runtime-css")]
use thaw_utils::mount_style;

#[component]
pub fn SkeletonItem(#[prop(optional, into)] class: MaybeProp<String>) -> impl IntoView {
    #[cfg(feature = "runtime-css")]
    mount_style("skeleton-item", include_str!("./skeleton-item.css"));

    view! { <div class=class_list!["thaw-skeleton-item", class]></div> }
}
