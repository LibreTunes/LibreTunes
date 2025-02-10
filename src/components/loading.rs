use leptos::prelude::*;

/// A loading indicator
#[component]
pub fn Loading() -> impl IntoView {
	let dots_style = "h-2 w-2 bg-accent rounded-full animate-pulse";

	view! {
		<div class="flex space-x-1 justify-center items-center bg-white my-2">
			<span class="sr-only">"Loading..."</span>
			<div class=dots_style style="animation-duration: 900ms; animation-delay: 0ms;" />
			<div class=dots_style style="animation-duration: 900ms; animation-delay: 300ms"/>
			<div class=dots_style style="animation-duration: 900ms; animation-delay: 600ms;" />
		</div>
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
