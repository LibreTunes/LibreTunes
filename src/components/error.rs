use leptos::prelude::*;
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
				<Icon icon=icondata::BiErrorSolid />
				<h1>{title}</h1>
			</div>
			<p>{message}</p>
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
		<div class="error-container">
			<div class="error-header">
				<Icon icon=icondata::BiErrorSolid />
				<h1>{title}</h1>
			</div>
			<p>{message}</p>
			<p>{error.map(|error| format!("{}", error))}</p>
		</div>
	}
}
