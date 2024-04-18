use leptos::*;
use leptos_icons::*;
use leptos::leptos_dom::*;
use crate::models::Playlist;
use crate::models::Song;
use crate::api::playlists::get_songs;
use crate::api::songs::get_artists;

fn convert_seconds(seconds: i32) -> String {
    let minutes = seconds / 60;
    let seconds = seconds % 60;
    let seconds_string = if seconds < 10 {
        format!("0{}", seconds)
    } else {
        format!("{}", seconds)
    };
    format!("{}:{}", minutes, seconds_string)
}

#[component]
pub fn Playlist(playlist: Playlist) -> impl IntoView {
    let (show_playlist, set_show_playlist) = create_signal(false);

    view! {
        <div class="playlist" on:click=move|_| set_show_playlist.update(|value| *value=true) >
            <h1 class="name">{playlist.name.clone()}</h1>

            <Show
                when=move || show_playlist()
                fallback=move || view! {<div></div>}
            >
                <div class="screen-darkener"></div>
            </Show>
            
            <Show
                when=move || show_playlist()
                fallback=move || view! {<div></div>}
            >
                <PlayListPopUp playlist=playlist.clone() set_show_playlist=set_show_playlist />
            </Show>
        </div>
    }
}
#[component]
pub fn PlayListPopUp(playlist: Playlist, set_show_playlist: WriteSignal<bool>) -> impl IntoView {
    let (songs, set_songs) = create_signal(vec![]);
    
    create_effect(move |_| {
        spawn_local(async move {
            let playlist_songs = get_songs(playlist.id.clone()).await;
            if let Err(err) = playlist_songs {
                // Handle the error here, e.g., log it or display to the user
                log!("Error getting songs: {:?}", err);
            } else {
                log!("Songs: {:?}", playlist_songs);
                log!("number of songs: {:?}", playlist_songs.clone().expect("REASON").len());
                set_songs.update(|value| *value = playlist_songs.unwrap());
            }
        })
    });

    view! {
        <div class="playlist-container">
            <div class="close-button" on:click=move |_| set_show_playlist.update(|value| *value = false)>
                <Icon icon=icondata::IoCloseSharp />
            </div>
            <div class="info">
                <h1>{playlist.name.clone()}</h1>
                <h1>{move || songs.get().len()}</h1>
            </div>
            
            <ul class="songs">
                {
                    move || songs.get().iter().enumerate().map(|(index,song)| view! {
                        <PlaylistSong song=song.clone() />
                    }).collect::<Vec<_>>()
                }
            </ul>
        </div>
    }
}
#[component]
pub fn PlaylistSong(song: Song) -> impl IntoView {
    let (artists, set_artists) = create_signal("".to_string());
    let (is_hovered, set_is_hovered) = create_signal(false);


    create_effect(move |_| {
        spawn_local(async move {
            let song_artists = get_artists(song.id.clone()).await;
            if let Err(err) = song_artists {
                // Handle the error here, e.g., log it or display to the user
                log!("Error getting artist: {:?}", err);
            } else {
                log!("Artist: {:?}", song_artists);
                let mut artist_string = "".to_string();
                let song_artists = song_artists.unwrap();
                for i in 0..song_artists.len() {
                    if i == song_artists.len() - 1 {
                        artist_string.push_str(&song_artists[i].name);
                    } else {
                        artist_string.push_str(&song_artists[i].name);
                        artist_string.push_str(" & ");
                    }
                }
                set_artists.update(|value| *value = artist_string);
            }
        })
    });

    view! {
        <div class="song" on:mouseenter=move |_| set_is_hovered.update(|value| *value=true) on:mouseleave=move |_| set_is_hovered.update(|value| *value=false)>
            <img src={song.image_path.clone()} alt={song.title.clone()} />
            <div class="song-info">
				<h3>{song.title.clone()}</h3>
				<p>{move || artists.get()}</p>
			</div>
            <div class="right">
                <Show
                    when=move || is_hovered()
                    fallback=move || view! {<p class="duration">{convert_seconds(song.duration)}</p>}            
                >
                    <Icon icon=icondata::FaTrashCanRegular />
                </Show>
            </div>
        </div>
    }
}