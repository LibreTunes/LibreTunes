use leptos::prelude::*;
use leptos::text_prop::TextProp;

#[slot]
pub struct DashboardTile {
	#[prop(into)]
	image_path: TextProp,
	#[prop(into)]
	title: TextProp,
	#[prop(into)]
	link: TextProp,
	#[prop(into, optional)]
	description: Option<TextProp>,
}
