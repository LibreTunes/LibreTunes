use leptos::prelude::*;
use leptos::either::*;
use leptos_router::hooks::use_params_map;
use server_fn::error::NoCustomError;
use crate::components::song_list::*;
use crate::components::loading::*;
use crate::components::error::*;
use crate::api::album::*;
use crate::models::frontend;

#[component]
pub fn AlbumPage() -> impl IntoView {
    let params = use_params_map();

    view! {
        {move || params.with(|params| {
            match params.get("id").map(|id| id.parse::<i32>()) {
                Some(Ok(id)) => {
                    Either::Left(view! { <AlbumIdPage id /> })
                },
                Some(Err(e)) => {
                    Either::Right(view! {
                        <Error<String>
                            title="Invalid Album ID"
                            error=e.to_string()
                        />
                    })
                },
                None => {
                    Either::Right(view! {
                        <Error<String>
                            title="No Album ID"
                            message="You must specify an album ID to view its page."
                        />
                    })
                }
            }
        })}
    }
}

#[component]
fn AlbumIdPage(#[prop(into)] id: Signal<i32>) -> impl IntoView {
    let album = Resource::new(id, get_album);

    let show_songs = RwSignal::new(false);

    view! {
        <Transition
            fallback=move || view! { <Loading /> }
        >
            {move || album.get().map(|album| {
                match album {
                    Ok(Some(album)) => {
                        show_songs.set(true);
                        EitherOf3::A(view! { <AlbumInfo album /> })
                    },
                    Ok(None) => EitherOf3::B(view! {
                        <Error<String>
                            title="Album Not Found"
                            message=format!("Album with ID {} not found", id.get())
                        />
                    }),
                    Err(error) => EitherOf3::C(view! {
                        <ServerError<NoCustomError>
                            title="Error Getting Album"
                            error
                        />
                    }),
                }
            })}
        </Transition>
        <Show when=show_songs>
            <AlbumSongs id />
        </Show>
    }
}

#[component]
fn AlbumInfo(album: frontend::Album) -> impl IntoView {
	view! {
		<div class="flex">
			<img class="w-70 h-70 p-5" src={album.image_path} alt="Album Cover" />
			<div class="self-center">
				<h1 class="text-4xl">{album.title}</h1>
                <SongArtists artists=album.artists />
			</div>
		</div>
	}.into_view()
}

#[component]
fn AlbumSongs(#[prop(into)] id: Signal<i32>) -> impl IntoView {
    let songs = Resource::new(id, get_songs);

    view! {
        <Transition
            fallback= move || view! { <Loading /> }
        >
            <ErrorBoundary
                fallback=|errors| view! {
                    {move || errors.get()
                        .into_iter()
                        .map(|(_, e)| view! { <p>{e.to_string()}</p> })
                        .collect_view()
                    }
                }
            >
                {move || songs.get().map(|songs| {
                    songs.map(|songs| {
                        view! { <SongList songs=songs /> }
                    })
                })}
            </ErrorBoundary>
        </Transition>
    }
}
