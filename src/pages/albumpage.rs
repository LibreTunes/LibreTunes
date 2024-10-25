use leptos::leptos_dom::*;
use leptos::*;
use leptos_router::*;
use crate::models::*;
use crate::components::song_list::*;


#[derive(Params, PartialEq)]
struct AlbumParams {
    id: i32
}

/*
#[component]
pub fn AlbumPage() -> impl IntoView {
    let params = use_params::<AlbumParams>();

    let id = move || {params.with(|params| {
            params.as_ref()
                .map(|params| params.id)
                .map_err(|e| e.clone())
        })
    };

    let song_list = create_resource(
        id,
        |value| async move {
            match value {
                Ok(v) => {get_album(v).await},
                Err(e) => {Err(ServerFnError::Request("Invalid album!".into()))},
            }
        },
    );

    view! {
        <Suspense
            fallback=move || view! { <p>"Loading..."</p> }
        >
            {move || {
                song_list.with( |song_list| {
                    match song_list {
                        Some(Ok(s)) => {
                            view! { <SongList songs=s.clone().into()/> }.into_view()
                        },
                        Some(Err(e)) => {
                            view! { <div>"Error loading albums: :e"</div> }.into_view()
                        },
                        None => {view! { }.into_view()}
                    }
                })
            }}
        </Suspense>
    }
}
*/
