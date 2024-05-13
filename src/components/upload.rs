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
			<div class="upload-container" open=open>
				<div class="close-button" on:click=close_dialog><Icon icon=icondata::IoClose /></div>
				<div class="upload-header">
					<h1>Upload Song</h1>
				</div>
				<Form action="/api/upload" method="POST" enctype=String::from("multipart/form-data") class="upload-form">

					<div class="input-bx">
						<input type="text" name="title" required class="text-input" required/>
						<span>Title</span>
					</div>
					<div class="input-bx">
						<input type="text" name="artist_ids" class="text-input" required/>
						<span>Artists</span>
					</div>
					<div class="input-bx">
						<input type="text" name="album_id" class="text-input" required/>
						<span>Album ID</span>
					</div>
					<div class="input-bx">
						<input type="number" name="track_number" class="text-input" required/>
						<span>Track Number</span>
					</div>

					<div class="release-date">
						<div class="left">
							<span>Release</span>
							<span>Date</span>
						</div>
						<input class="info" type="date" name="release_date"/>
					</div>

					<div class="file">
						<span>File</span>
						<input class="info" type="file" name="file"/>
					</div>

					<button type="submit" class="upload-button">Upload</button>
				</Form>
			</div>
		</Show>
	}
}
