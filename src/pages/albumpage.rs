use leptos::leptos_dom::*;
use leptos::*;
use leptos_router::*;
use crate::components::song_list::*;
use crate::api::album::*;
use crate::components::album_info::*;


#[derive(Params, PartialEq)]
struct AlbumParams {
    id: i32
}

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
                Ok(v) => {get_songs(v).await},
                Err(e) => {Err(ServerFnError::Request(format!("Error getting song data: {}", e).into()))},
            }
        },
    );

    let albumdata = create_resource(
        id,
        |value| async move {
            match value {
                Ok(v) => {get_album(v).await},
                Err(e) => {Err(ServerFnError::Request(format!("Error getting song data: {}", e).into()))},
            }
        },
    );

    view! {
        <div class="album-page-container">
            <div class="album-header">
                <Suspense
                    fallback=move || view! { <p class="loading">"Loading..."</p> }
                >
                    {move || {
                        albumdata.with( |albumdata| {
                            match albumdata {
                                Some(Ok(s)) => {
                                    view! { <AlbumInfo albumdata=(*s).clone() /> }
                                },
                                Some(Err(e)) => {
                                    view! { <div class="error">{format!("Error loading album : {}",e)}</div> }.into_view()
                                },
                                None => {view! { }.into_view()}
                            }
                        })
                    }}
                </Suspense>
            </div>
        
            <Suspense
                fallback=move || view! { <p class="loading">"Loading..."</p> }
            >
                {move || {
                    song_list.with( |song_list| {
                        match song_list {
                            Some(Ok(s)) => {
                                view! { <SongList songs=(*s).clone()/> }
                            },
                            Some(Err(e)) => {
                                view! { <div class="error">{format!("Error loading albums: : {}",e)}</div> }.into_view()
                            },
                            None => {view! { }.into_view()}
                        }
                    })
                }}
            </Suspense>
        </div>
    }
}

