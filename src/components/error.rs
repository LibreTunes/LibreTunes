use leptos::prelude::*;
use leptos::text_prop::TextProp;
use leptos_icons::*;
use std::fmt::Display;

#[component]
pub fn ServerError<E: Display + 'static>(
	#[prop(optional, into, default="An Error Occurred".into())]
	title: TextProp,
	#[prop(optional, into)]
	message: TextProp,
	#[prop(optional, into)]
	error: Option<ServerFnError<E>>,
) -> impl IntoView {
	view!{
		<div class="error-container">
			<div class="error-header">
				<Icon icon={icondata::BiErrorSolid} />
				<h1>{move || title.get()}</h1>
			</div>
			<p>{move || message.get()}</p>
			<p>{error.map(|error| format!("{}", error))}</p>
		</div>
	}
}

#[component]
pub fn Error<E: Display + 'static>(
	#[prop(optional, into, default="An Error Occurred".into())]
	title: TextProp,
	#[prop(optional, into)]
	message: TextProp,
	#[prop(optional, into)]
	error: Option<E>,
) -> impl IntoView {
	view! {
		<div class="text-red-800">
			<div class="grid grid-cols-[max-content_1fr] gap-1">
				<Icon icon={icondata::BiErrorSolid} {..} class="self-center" />
				<h1 class="self-center">{move || title.get()}</h1>
			</div>
			<p>{move || message.get()}</p>
			<p>{error.map(|error| format!("{}", error))}</p>
		</div>
	}
}
