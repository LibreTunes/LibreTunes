use crate::playstatus::PlayStatus;
use leptos::leptos_dom::*;
use leptos::*;

#[component]
pub fn Queue(status: RwSignal<PlayStatus>) -> impl IntoView {

	view!{
		<Show
			when=move || status.with(|status| status.queue_open)
			fallback=|| view!{""}>
			<div class="queue">
				<div class="queue-header">
					<h2>Queue</h2>
				</div>
				<ul>
					{
						status.with(|status| status.queue.iter()
							.map(|song| view! {
								<div class="queue-song">
									<img src={song.image_path.clone()} alt={song.name.clone()}/>
									<div class="queue-song-info">
										<h3>{song.name.clone()}</h3>
										<p>{song.artist.clone()}</p>
									</div>
								</div>
							})
							.collect::<Vec<_>>())
					}
				</ul>
			</div>
		</Show>

	}
}
