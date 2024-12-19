use leptos::*;
use leptos_router::use_params_map;
use leptos_icons::*;
use server_fn::error::NoCustomError;

use crate::components::loading::*;
use crate::components::error::*;

#[component]
pub fn ArtistPage() -> impl IntoView {
    let params = use_params_map();

    view! {
        <div class="artist-container home-component">
            {move || params.with(|params| {
                match params.get("id").map(|id| id.parse::<i32>()) {
                    Some(Ok(id)) => {
                        view! { <ArtistProfile id /> }.into_view()
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
fn ArtistProfile(#[prop(into)] id: MaybeSignal<i32>) -> impl IntoView {
    let artist_info = create_resource(move || id.get(), move |id| {
        get_artist_by_id(id)
    });

    view! {
        <Transition
            fallback=move || view! { <LoadingPage /> }
        >
            {move || artist_info.get().map(|artist| {
                match artist {
                    Ok(Some(artist)) => view! { <ArtistDetails artist /> }.into_view(),
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
    }
}

#[component]
fn ArtistDetails(artist: Artist) -> impl IntoView {
    let artist_id = artist.id.unwrap();
    let profile_image_path = format!("/assets/images/artist/{}.webp", artist_id);

    view! {
        <div class="artist-header">
            <object class="artist-image" data={profile_image_path.clone()} type="image/webp">
                <Icon class="artist-image" icon=icondata::CgProfile width="100" height="100"/>
            </object>
            <h1>{artist.name}</h1>
        </div>
        <div class="artist-details">
            <p>{artist.bio.unwrap_or_else(|| "No bio available.".to_string())}</p>
        </div>
    }
}

#[component]
fn TopSongsByArtist(#[prop(into)] artist_id: MaybeSignal<i32>) -> impl IntoView {
    let top_songs = create_resource(move || artist_id.get(), |artist_id| async move {
        top_songs_by_artist(artist_id, Some(10)).await
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
fn RelatedArtists(#[prop(into)] artist_id: MaybeSignal<i32>) -> impl IntoView {
    let related_artists = create_resource(move || artist_id.get(), |artist_id| async move {
        get_related_artists(artist_id).await
    });

    view! {
        <h2>"Related Artists"</h2>
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
                {move || related_artists.get().map(|related_artists| {
                    related_artists.map(|artists| {
                        let tiles = artists.into_iter().map(|artist| {
                            Box::new(artist) as Box<dyn DashboardTile>
                        }).collect::<Vec<_>>();

                        DashboardRow::new("Related Artists", tiles)
                    })
                })}
            </ErrorBoundary>
        </Transition>
    }
}

pub async fn get_artist_by_id(artist_id: i32) -> Result<Option<Artist>, NoCustomError> {
    // todo
    Ok(Some(Artist {
        id: Some(artist_id),
        name: "Example Artist".to_string(),
        bio: Some("A great artist!".to_string()),
    }))
}

pub async fn get_related_artists(artist_id: i32) -> Result<Vec<Artist>, NoCustomError> {
    // todo
    Ok(vec![
        Artist {
            id: Some(artist_id + 1),
            name: "Related Artist 1".to_string(),
            bio: None,
        },
        Artist {
            id: Some(artist_id + 2),
            name: "Related Artist 2".to_string(),
            bio: None,
        },
    ])
}