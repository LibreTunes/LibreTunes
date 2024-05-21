use leptos::*;
use leptos_icons::*;
use leptos::leptos_dom::*;

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
pub fn UploadDropdown() -> impl IntoView {
    view! {
        <div class="upload-dropdown">
            hello
        </div>
    }
}