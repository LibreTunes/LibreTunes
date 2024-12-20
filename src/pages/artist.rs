use leptos::*;
use leptos_router::use_params_map;
use leptos_icons::*;
use server_fn::error::NoCustomError;

use crate::models::Artist;

use crate::components::loading::*;
use crate::components::error::*;
use crate::components::song_list::*;

use crate::api::artists::*;

#[component]
pub fn ArtistPage() -> impl IntoView {
    let params = use_params_map();

    view! {
        <div class="artist-container home-component">
            {move || params.with(|params| {
                match params.get("id").map(|id| id.parse::<i32>()) {
                    Some(Ok(id)) => {
                        view! { <ArtistIdProfile id /> }.into_view()
                    },
                    Some(Err(e)) => {
                        view! {
                            <Error<String>
                                title="Invalid Artist ID"
                                error=e.to_string()
                            />
                        }.into_view()
                    },
                    None => {
                        view! {
                            <Error<String>
                                title="No Artist ID"
                                message="You must specify an artist ID to view their page."
                            />
                        }.into_view()
                    }
                }
            })}
        </div>
    }
}

#[component]
fn ArtistIdProfile(#[prop(into)] id: MaybeSignal<i32>) -> impl IntoView {
    let artist_info = create_resource(move || id.get(), move |id| {
        get_artist_by_id(id)
    });

    let show_details = create_rw_signal(false);

    view! {
        <Transition
            fallback=move || view! { <LoadingPage /> }
        >
            {move || artist_info.get().map(|artist| {
                match artist {
                    Ok(Some(artist)) => {
                        show_details.set(true);
                        view! { <ArtistProfile artist /> }.into_view()
                    },
                    Ok(None) => view! {
                        <Error<String>
                            title="Artist Not Found"
                            message=format!("Artist with ID {} not found", id.get())
                        />
                    }.into_view(),
                    Err(error) => view! {
                        <ServerError<NoCustomError>
                            title="Error Getting Artist"
                            error
                        />
                    }.into_view(),
                }
            })}
        </Transition>
        <div hidden={move || !show_details.get()}>
            <TopSongsByArtist artist_id=id />
            <AlbumsByArtist artist_id=id />
        </div>
    }
}

#[component]
fn ArtistProfile(artist: Artist) -> impl IntoView {
    let artist_id = artist.id.unwrap();
    let profile_image_path = format!("/assets/images/artist/{}.webp", artist_id);

    leptos::logging::log!("Artist name: {}", artist.name);

    view! {
        <div class="artist-header">
            <object class="artist-image" data={profile_image_path.clone()} type="image/webp">
                <Icon class="artist-image" icon=icondata::CgProfile width="100" height="100"/>
            </object>
            <h1>{artist.name}</h1>
        </div>
    }
}

#[component]
fn TopSongsByArtist(#[prop(into)] artist_id: MaybeSignal<i32>) -> impl IntoView {
    let top_songs = create_resource(move || artist_id.get(), |artist_id| async move {
        let top_songs = top_songs_by_artist(artist_id, Some(10), 1).await;

        top_songs.map(|top_songs| {
            top_songs.into_iter().map(|(song, plays)| {
                let plays = if plays == 1 {
                    "1 play".to_string()
                } else {
                    format!("{} plays", plays)
                };

                (song, plays)
            }).collect::<Vec<_>>()
        })
    });

    view! {
        <h2>"Top Songs"</h2>
        <Transition
            fallback=move || view! { <Loading /> }
        >
            <ErrorBoundary
                fallback=|errors| view! {
                    {move || errors.get()
                        .into_iter()
                        .map(|(_, e)| view! { <p>{e.to_string()}</p>})
                        .collect_view()
                    }
                }
            >
                {move || top_songs.get().map(|top_songs| {
                    top_songs.map(|top_songs| {
                        view! { <SongListExtra songs=top_songs /> }
                    })
                })}
            </ErrorBoundary>
        </Transition>
    }
}

#[component]
fn AlbumsByArtist(#[prop(into)] artist_id: MaybeSignal<i32>) -> impl IntoView {
    use crate::components::dashboard_row::*;
    use crate::components::dashboard_tile::*;

    let albums = create_resource(move || artist_id.get(), |artist_id| async move {
        let albums = albums_by_artist(artist_id, None).await;

        albums.map(|albums| {
            albums.into_iter().map(|album| {
                album
            }).collect::<Vec<_>>()
        })
    });

    view! {
        <Transition
            fallback=move || view! { <Loading /> }
        >
            <ErrorBoundary
                fallback=|errors| view! {
                    {move || errors.get()
                        .into_iter()
                        .map(|(_, e)| view! { <p>{e.to_string()}</p>})
                        .collect_view()
                    }
                }
            >
                {move || albums.get().map(|albums| {
                    albums.map(|albums| {
                        DashboardRow::new("Albums".to_string(), albums.into_iter().map(|album| {
                            Box::new(album) as Box<dyn DashboardTile>
                        }).collect())
                    })
                })}
            </ErrorBoundary>
        </Transition>
    }
}
