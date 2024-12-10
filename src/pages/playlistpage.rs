use leptos::leptos_dom::*;
use leptos::*;
use leptos_router::*;
use crate::components::song_list::*;
use crate::api::playlist::*;


#[derive(Params, PartialEq)]
struct PlaylistParams {
    id: i32
}

#[component]
pub fn PlaylistPage() -> impl IntoView {
    let params = use_params::<PlaylistParams>();

    let id = move || {params.with(|params| {
            params.as_ref()
                .map(|params| params.id)
                .map_err(|e| e.clone())
        })
    };

    let playlist_data = create_resource(
        id,
        |value| async move {
            match value {
                Ok(v) => {get_playlist(v).await},
                Err(e) => {Err(ServerFnError::Request(format!("Error getting song data: {}", e).into()))},
            }
        }
    );

    view! {
        <div class="album-page-container">
            <div class="album-header">
                <Suspense
                    fallback=move || view! { <p class="loading">"Loading..."</p> }
                >
                    {move || {
                        playlist_data.with( |playlist_data| {
                            match playlist_data {
                                Some(Ok(s)) => {
                                    view! { <h1>{(*s).clone().title}</h1> }
                                },
                                Some(Err(e)) => {
                                    view! { <div class="error">{format!("Error loading playlist : {}",e)}</div> }.into_view()
                                },
                                None => {view! { }.into_view()}
                            }
                        })
                    }}
                </Suspense>
            </div>
        </div>
    }
}

