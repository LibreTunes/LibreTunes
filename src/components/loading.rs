use leptos::prelude::*;

/// A loading indicator
#[component]
pub fn Loading() -> impl IntoView {
	view! {
		<div class="loading"></div>
	}
}

/// A full page, centered loading indicator
#[component]
pub fn LoadingPage() -> impl IntoView {
	view!{
		<div class="loading-page">
			<Loading />
		</div>
	}
}
