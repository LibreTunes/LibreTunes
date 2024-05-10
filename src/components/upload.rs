use leptos::leptos_dom::*;
use leptos::*;
use leptos_icons::*;
use leptos_router::Form;

#[component]
pub fn UploadBtn(dialog_open: RwSignal<bool>) -> impl IntoView {
	let open_dialog = move |_| {
		dialog_open.set(true);
	};

	view! {
		<button class="upload-btn" on:click=open_dialog>
			<div class="add-sign">
				<Icon icon=icondata::IoAddSharp />
			</div>
			Upload
		</button>
	}
}

#[component]
pub fn Upload(open: RwSignal<bool>) -> impl IntoView {
	let close_dialog = move |ev: leptos::ev::MouseEvent| {
		ev.prevent_default();
		open.set(false);
	};

	let click_cancel_bubble = move |ev: leptos::ev::MouseEvent| {
		ev.prevent_default();
		ev.stop_propagation();
	};

	view! {
		<Show when=open fallback=move || view! {}>
			<div class="dialog-container" on:click=close_dialog>
				<dialog open=open on:click=click_cancel_bubble>
					<div class="dialog-header">
						<button class="close-button" on:click=close_dialog><Icon icon=icondata::IoClose /></button>
						<h1>Upload Song</h1>
					</div>

					<Form action="/api/upload" method="POST" enctype=String::from("multipart/form-data")>
						<div class="input-box">
							<input class="info" type="text" name="title" required/>
							<span>Title</span>
							<i></i>
						</div>

						<div class="input-box">
							<input class="info" type="text" name="artist"/>
							<span>Artist</span>
							<i></i>
						</div>

						<div class="input-box">
							<input class="info" type="text" name="album_id"/>
							<span>Album</span>
							<i></i>
						</div>

						<div class="input-box">
							<input class="info" type="number" name="track_number"/>
							<span>Track Number</span>
							<i></i>
						</div>

						<div>
							<input class="info" type="date" name="release_date"/>
							<span>Release Date</span>
							<i></i>
						</div>

						<div>
							<input class="info" type="file" name="file"/>
							<span>File</span>
							<i></i>
						</div>

						<button type="submit">Upload</button>
					</Form>
				</dialog>
			</div>
		</Show>
	}
}
