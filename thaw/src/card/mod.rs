mod card_footer;
mod card_header;
mod card_preview;

pub use card_footer::*;
pub use card_header::*;
pub use card_preview::*;

use leptos::prelude::*;
use thaw_utils::{class_list};
#[cfg(feature = "runtime-css")]
use thaw_utils::mount_style;

#[component]
pub fn Card(#[prop(optional, into)] class: MaybeProp<String>, children: Children) -> impl IntoView {
    #[cfg(feature = "runtime-css")]
    mount_style("card", include_str!("./card.css"));

    view! {
        <div class=class_list!["thaw-card", class] role="group">
            {children()}
        </div>
    }
}
