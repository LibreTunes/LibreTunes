use leptos::*;
use leptos_icons::*;
use leptos::leptos_dom::*;
use crate::models::Playlist;
use crate::models::Song;
use crate::api::playlists::get_songs;
use crate::api::songs::get_artists;
use crate::api::playlists::remove_song;

fn total_duration(songs: ReadSignal<Vec<Song>>) -> i32 {
    let mut total_duration = 0;
    for song in songs.get() {
        total_duration += song.duration;
    }
    total_duration
}

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

fn convert_to_text_time(seconds: i32) -> String {
    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    format!("{} hour{}, {} minute{}", hours, if hours > 1 { "s" } else { "" }, minutes, if minutes > 1 { "s" } else { "" })
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
                <p>{move || songs.get().len()} songs {move || convert_to_text_time(total_duration(songs))}</p>
            </div>
            <div class="options">
                <button><Icon class="button-icons" icon=icondata::BsPlayFill />Play</button>
                <button><Icon class="button-icons" icon=icondata::IoShuffle />Shuffle</button>
            </div>
            <ul class="songs">
                {
                    move || songs.get().iter().enumerate().map(|(index,song)| view! {
                        <PlaylistSong song=song.clone()  playlist_id=playlist.id.clone() set_songs=set_songs />
                    }).collect::<Vec<_>>()
                }
            </ul>
        </div>
    }
}
#[component]
pub fn PlaylistSong(song: Song, playlist_id: Option<i32>, set_songs: WriteSignal<Vec<Song>>) -> impl IntoView {
    let (artists, set_artists) = create_signal("".to_string());
    let (is_hovered, set_is_hovered) = create_signal(false);

    let delete_song = move |_| {
        spawn_local(async move {
            let delete_result = remove_song(song.id.clone(), playlist_id).await;
            if let Err(err) = delete_result {
                // Handle the error here, e.g., log it or display to the user
                log!("Error deleting song: {:?}", err);
            } else {
                log!("Song deleted successfully!");
                set_songs.update(|value| *value = value.iter().filter(|s| s.id != song.id).cloned().collect());
            }
        })
    };


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
                <div class="delete-song" on:click=delete_song>
                    <Icon icon=icondata::FaTrashCanRegular />
                </div>
                </Show>
            </div>
        </div>
    }
}