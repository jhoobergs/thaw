use leptos::prelude::*;
use thaw_utils::{class_list};
#[cfg(feature = "runtime-css")]
use thaw_utils::mount_style;

#[component]
pub fn CardFooter(
    #[prop(optional, into)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    #[cfg(feature = "runtime-css")]
    mount_style("card-footer", include_str!("./card-footer.css"));
    view! { <div class=class_list!["thaw-card-footer", class]>{children()}</div> }
}
