use leptos::leptos_dom::*;
use leptos::*;

pub trait DashboardTile {
	fn image_path(&self) -> String;
	fn title(&self) -> String;
	fn link(&self) -> String;
	fn description(&self) -> Option<String> { None }
}

impl IntoView for &dyn DashboardTile {
	fn into_view(self) -> View {
		let link = self.link();

		view! {
			<div class="dashboard-tile">
				<a href={link}>
					<img src={self.image_path()} alt="dashboard-tile" />
					<p class="dashboard-tile-title">{self.title()}</p>
					<p class="dashboard-tile-description">
						{self.description().unwrap_or_default()}
					</p>
				</a>
			</div>
		}.into_view()
	}
}
