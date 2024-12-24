use leptos::prelude::*;
use leptos::leptos_dom::log;
use leptos_icons::*;
use crate::api::albums::add_album;

#[component]
pub fn AddAlbumBtn(add_album_open: RwSignal<bool>) -> impl IntoView {
    let open_dialog = move |_| {
        add_album_open.set(true);
    };
    view! {
        <button class="add-album-btn add-btns" on:click=open_dialog>
            Add Album
        </button>
    }
}
#[component]
pub fn AddAlbum(open: RwSignal<bool>) -> impl IntoView {
    let album_title = create_rw_signal("".to_string());
    let release_date = create_rw_signal("".to_string());
    let image_path = create_rw_signal("".to_string());

    let close_dialog = move |ev: leptos::ev::MouseEvent| {
		ev.prevent_default();
		open.set(false);
	};

    let on_add_album = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        let album_title_clone = album_title.get();
        let release_date_clone = Some(release_date.get());
        let image_path_clone = Some(image_path.get());

        spawn_local(async move {
            let add_album_result = add_album(album_title_clone, release_date_clone, image_path_clone).await;
            if let Err(err) = add_album_result {
                log!("Error adding album: {:?}", err);
            } else if let Ok(album) = add_album_result {
                log!("Added album: {:?}", album);
                album_title.set("".to_string());
                release_date.set("".to_string());
                image_path.set("".to_string());
            }
        })
    };

    view! {
        <Show when=open fallback=move|| view!{}>
            <div class="add-album-container">
                <div class="upload-header">
                    <h1>Add Album</h1>
                </div>
                <div class="close-button" on:click=close_dialog><Icon icon={icondata::IoClose} /></div>
                <form class="create-album-form" action="POST" on:submit=on_add_album>
                    <div class="input-bx">
                        <input type="text" required class="text-input" 
                            prop:value=album_title
                            on:input=move |ev: leptos::ev::Event| {
                                album_title.set(event_target_value(&ev));
                            }        
                         />
                        <span>Album Title</span>
                    </div>
                    <div class="release-date">
						<div class="left">
							<span>Release</span>
							<span>Date</span>
						</div>
						<input class="info" type="date"
                            prop:value=release_date
                            on:input=move |ev: leptos::ev::Event| {
                                release_date.set(event_target_value(&ev));
                            }
                        />
					</div>
                    <div class="input-bx">
                        <input type="text" class="text-input" 
                            prop:value=image_path
                            on:input=move |ev: leptos::ev::Event| {
                                image_path.set(event_target_value(&ev));
                            }        
                         />
                        <span>Image Path</span>
                    </div>
                    <button type="submit" class="upload-button">Add</button>
                </form>
            </div>
        </Show>
    }

}