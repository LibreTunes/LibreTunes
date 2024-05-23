use leptos::*;
use leptos_icons::*;
use crate::components::upload::*;
use crate::components::add_artist::*;
use crate::components::add_album::*;

#[component]
pub fn UploadDropdownBtn(dropdown_open: RwSignal<bool>) -> impl IntoView {
    let open_dropdown = move |_| {
        dropdown_open.set(!dropdown_open.get());
    };
    view! {
        <button class={move || if dropdown_open() {"upload-dropdown-btn upload-dropdown-btn-active"} else {"upload-dropdown-btn"}} on:click=open_dropdown>
			<div class="add-sign">
				<Icon icon=icondata::IoAddSharp />
			</div>
		</button>
    }
}

#[component]
pub fn UploadDropdown(upload_open: RwSignal<bool>, add_artist_open: RwSignal<bool>, add_album_open: RwSignal<bool>) -> impl IntoView {
    view! {
        <div class="upload-dropdown">
            <UploadBtn dialog_open=upload_open/>
            <AddArtistBtn add_artist_open=add_artist_open/>
            <AddAlbumBtn add_album_open=add_album_open/>
        </div>
    }
}