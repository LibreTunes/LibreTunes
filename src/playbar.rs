use crate::playstatus::PlayStatus;
use crate::api::songs::get_artists;
use crate::api::albums::get_album;
use leptos::ev::MouseEvent;
use leptos::html::{Audio, Div};
use leptos::leptos_dom::*;
use leptos::*;
use leptos_icons::BsIcon::*;
use leptos_icons::RiIcon::*;
use leptos_icons::*;

/// Width and height of the forward/backward skip buttons
const SKIP_BTN_SIZE: &str = "3.5em";
/// Width and height of the play/pause button
const PLAY_BTN_SIZE: &str = "5em";

// Width and height of the queue button
const QUEUE_BTN_SIZE: &str = "3.5em";

/// Threshold in seconds for skipping to the previous song instead of skipping to the start of the current song
const MIN_SKIP_BACK_TIME: f64 = 5.0;

/// How many seconds to skip forward/backward when the user presses the arrow keys
const ARROW_KEY_SKIP_TIME: f64 = 5.0;

// TODO Handle errors better, when getting audio HTML element and when playing/pausing audio

/// Get the current time and duration of the current song, if available
/// 
/// # Arguments
/// 
/// * `status` - The `PlayStatus` to get the audio element from, as a signal
/// 
/// # Returns
/// 
/// * `None` if the audio element is not available
/// * `Some((current_time, duration))` if the audio element is available
/// 
pub fn get_song_time_duration(status: impl SignalWithUntracked<Value = PlayStatus>) -> Option<(f64, f64)> {
	status.with_untracked(|status| {
		if let Some(audio) = status.get_audio() {
			Some((audio.current_time(), audio.duration()))
		} else {
			error!("Unable to get current duration: Audio element not available");
			None
		}
	})
}

/// Skip to a certain time in the current song, optionally playing it
/// 
/// If the given time is +/- infinity or NaN, logs an error and returns
/// Logs an error if the audio element is not available, or if playing the song fails
/// 
/// # Arguments
/// 
/// * `status` - The `PlayStatus` to get the audio element from, as a signal
/// * `time` - The time to skip to, in seconds
/// 
pub fn skip_to(status: impl SignalUpdate<Value = PlayStatus>, time: f64) {
    if time.is_infinite() || time.is_nan() {
        error!("Unable to skip to non-finite time: {}", time);
        return
    }

    status.update(|status| {
        if let Some(audio) = status.get_audio() {
            audio.set_current_time(time);
            log!("Player skipped to time: {}", time);
        } else {
            error!("Unable to skip to time: Audio element not available");
        }
    });
}

/// Play or pause the current song
/// 
/// Logs an error if the audio element is not available, or if playing/pausing the song fails
/// 
/// # Arguments
/// * `status` - The `PlayStatus` to get the audio element from, as a signal
/// * `play` - `true` to play the song, `false` to pause it
/// 
pub fn set_playing(status: impl SignalUpdate<Value = PlayStatus>, play: bool) {
    status.update(|status| {
        if let Some(audio) = status.get_audio() {
            if play {
                if let Err(e) = audio.play() {
                    error!("Unable to play audio: {:?}", e);
                } else {
                    status.playing = true;
                    log!("Successfully played audio");
                }
            } else {
                if let Err(e) = audio.pause() {
                    error!("Unable to pause audio: {:?}", e);
                } else {
                    status.playing = false;
                    log!("Successfully paused audio");
                }
            }
        } else {
            error!("Unable to play/pause audio: Audio element not available");
        }
    });
}

fn toggle_queue(status: impl SignalUpdate<Value = PlayStatus>) {
	status.update(|status| {
		status.queue_open = !status.queue_open;
	});

	
}

/// Set the source of the audio player
/// 
/// Logs an error if the audio element is not available
/// 
/// 
/// # Arguments
/// * `status` - The `PlayStatus` to get the audio element from, as a signal
/// * `src` - The source to set the audio player to
/// 
fn set_play_src(status: impl SignalUpdate<Value = PlayStatus>, src: String) {
    status.update(|status| {
        if let Some(audio) = status.get_audio() {
            audio.set_src(&src);
            log!("Player set src to: {}", src);
        } else {
            error!("Unable to set src: Audio element not available");
        }
    });
}

/// The play, pause, and skip buttons
#[component]
fn PlayControls(status: RwSignal<PlayStatus>) -> impl IntoView {
    // On click handlers for the skip and play/pause buttons

    let skip_back = move |_| {
        if let Some(duration) = get_song_time_duration(status) {
            // Skip to previous song if the current song is near the start
            // Also skip to the previous song if we're at the end of the current song
            // This is because after running out of songs in the queue, the current song will be at the end
            // but our queue will be empty. We *do* want to "skip the start of the current song",
            // but first we need to get the *previous* song from the history since that's what we were playing before
            if duration.0 < MIN_SKIP_BACK_TIME || duration.0 >= duration.1 {
                log!("Skipping to the previous song");

                // Pop the most recently played song from the history if possible
                let mut last_played_song = None;
                status.update(|status| last_played_song = status.history.pop_back());

                if let Some(last_played_song) = last_played_song {
                    // Push the popped song to the front of the queue, and play it
                    let next_src = last_played_song.storage_path.clone();
                    status.update(|status| status.queue.push_front(last_played_song));
                    set_play_src(status, next_src);
                    set_playing(status, true);
                } else {
                    warn!("Unable to skip back: No previous song");
                }
            }
        }

        // Default to skipping to start of current song, and playing
        log!("Skipping to start of current song");
        skip_to(status, 0.0);
        set_playing(status, true);
    };

    let skip_forward = move |_| {
		if let Some(duration) = get_song_time_duration(status) {
            skip_to(status, duration.1);
            set_playing(status, true);
		} else {
			error!("Unable to skip forward: Unable to get current duration");
		}
    };

    let toggle_play = move |_| {
        let playing = status.with_untracked(|status| { status.playing });
		set_playing(status, !playing);
    };

    // We use this to prevent the buttons from being focused when clicked
    // If buttons were focused on clicks, then pressing space bar to play/pause would "click" the button
    // and trigger unwanted behavior
    let prevent_focus = move |e: MouseEvent| {
        e.prevent_default();
    };

    // Change the icon based on whether the song is playing or not
    let icon = Signal::derive(move || {
        status.with(|status| {
            if status.playing {
                Icon::from(BsPauseFill)
            } else {
                Icon::from(BsPlayFill)
            }
        })
    });

    view! {
        <div class="playcontrols" align="center">

        <button on:click=skip_back on:mousedown=prevent_focus>
        <Icon class="controlbtn" width=SKIP_BTN_SIZE height=SKIP_BTN_SIZE icon=Icon::from(BsSkipStartFill) />
        </button>

        <button on:click=toggle_play on:mousedown=prevent_focus>
        <Icon class="controlbtn" width=PLAY_BTN_SIZE height=PLAY_BTN_SIZE icon={icon} />
        </button>

        <button on:click=skip_forward on:mousedown=prevent_focus>
        <Icon class="controlbtn" width=SKIP_BTN_SIZE height=SKIP_BTN_SIZE icon=Icon::from(BsSkipEndFill) />
        </button>

        </div>
    }
}

/// The elapsed time and total time of the current song
#[component]
fn PlayDuration(elapsed_secs: MaybeSignal<i64>, total_secs: MaybeSignal<i64>) -> impl IntoView {
    // Create a derived signal that formats the elapsed and total seconds into a string
    let play_duration = Signal::derive(move || {
        let elapsed_mins = (elapsed_secs.get() - elapsed_secs.get() % 60) / 60;
        let total_mins = (total_secs.get() - total_secs.get() % 60) / 60;
        let elapsed_secs = elapsed_secs.get() % 60;
        let total_secs = total_secs.get() % 60;

        // Format as "MM:SS / MM:SS"
        format!("{}:{:0>2} / {}:{:0>2}", elapsed_mins, elapsed_secs, total_mins, total_secs)
    });

    view! {
        <div class="playduration" align="right">
        {play_duration}
        </div>
    }
}

/// The name, artist, and album of the current song
#[component]
fn MediaInfo(status: RwSignal<PlayStatus>) -> impl IntoView {
    let name = Signal::derive(move || {
		status.with(|status| {
			status.queue.front().map_or("No media playing".into(), |song| song.title.clone())
		})
    });

	let song_id = Signal::derive(move || {
		status.with(|status| {
			status.queue.front().map_or(None, |song| song.id)
		})
	});

	let song_artists_resource = create_resource(song_id, move |song_id| async move {
		let artists_vec = get_artists(song_id).await.unwrap_or(Vec::new());
		// convert the vec of artists to a string of artists separated by commas
		let artists_string = artists_vec.iter().map(|artist| artist.name.clone()).collect::<Vec<String>>().join(", ");
		artists_string
	});

	let album_id = Signal::derive(move || {
		status.with(|status| {
			status.queue.front().map_or(None, |song| song.album_id)
		})
	});

	let album_resource = create_resource(album_id, move |album_id| async move {
		// get the album name attribute or return "Unknown Album"
		let album_name = get_album(album_id).await.map_or("".to_string(), |album| album.title);
		album_name
	});

	let image = Signal::derive(move || {
		status.with(|status| {
			// TODO Use some default / unknown image?
			status.queue.front().map_or("".into(), |song| song.image_path.clone().unwrap_or("".into()))
		})
	});

    view! {
        <div class="media-info">
        <img class="media-info-img" align="left" src={image}/>
        <div class="media-info-text">
            {name}
            <br/>
            <Suspense 
				fallback=move || {
					view! {"Loading Artists..."}
				}
			>
			{move || {
				song_artists_resource.get().map(|artists_string| view! {
					<p>{artists_string}</p>
				})
			}}
			</Suspense>
			<Suspense
				fallback=move || {
					view! {"Loading Album..."}
				}
			>
			{move || {
				album_resource.get().map(|album_name| view! {
					<p>{album_name}</p>
				})
			}}
			</Suspense>
        </div>
        </div>
    }
}

/// The play progress bar, and click handler for skipping to a certain time in the song
#[component]
fn ProgressBar(percentage: MaybeSignal<f64>, status: RwSignal<PlayStatus>) -> impl IntoView {
    // Keep a reference to the progress bar div so we can get its width and calculate the time to skip to
    let progress_bar_ref = create_node_ref::<Div>();

    let progress_jump = move |e: MouseEvent| {
        let x_click_pos = e.offset_x() as f64;
        log!("Progress bar clicked at x: {}", x_click_pos);

        if let Some(progress_bar) = progress_bar_ref.get() {
            let width = progress_bar.offset_width() as f64;
            let percentage = x_click_pos / width * 100.0;

			if let Some(duration) = get_song_time_duration(status) {
				let time = duration.1 * percentage / 100.0;
				skip_to(status, time);
                set_playing(status, true);
			} else {
                error!("Unable to skip to time: Unable to get current duration");
            }
        } else {
            error!("Unable to skip to time: Progress bar not available");
        }
    };

    // Create a derived signal that formats the song percentage into a CSS style string for width
    let bar_width_style = Signal::derive(move || format!("width: {}%;", percentage.get()));

    view! {
        <div class="invisible-media-progress" _ref=progress_bar_ref on:click=progress_jump> // Larger click area
        <div class="media-progress"> // "Unfilled" progress bar
        <div class="media-progress-solid" style=bar_width_style> // "Filled" progress bar
		</div>
        </div>
        </div>
    }
}

#[component]
fn QueueToggle(status: RwSignal<PlayStatus>) -> impl IntoView {

    let update_queue = move |_| {
        toggle_queue(status);
		log!("queue button pressed, queue status: {:?}", status.with_untracked(|status| status.queue_open));
    };

	// We use this to prevent the buttons from being focused when clicked
    // If buttons were focused on clicks, then pressing space bar to play/pause would "click" the button
    // and trigger unwanted behavior
    let prevent_focus = move |e: MouseEvent| {
        e.prevent_default();
    };

    view! {
        <div class="queue-toggle">
        <button on:click=update_queue on:mousedown=prevent_focus>
        <Icon class="controlbtn" width=QUEUE_BTN_SIZE height=QUEUE_BTN_SIZE icon=Icon::from(RiPlayListMediaFill) />
        </button>
        </div>
    }
}

/// The main play bar component, containing the progress bar, media info, play controls, and play duration
#[component]
pub fn PlayBar(status: RwSignal<PlayStatus>) -> impl IntoView {
    // Listen for key down events -- arrow keys don't seem to trigger key press events
    let _arrow_key_handle = window_event_listener(ev::keydown, move |e: ev::KeyboardEvent| {
        if e.key() == "ArrowRight" {
            e.prevent_default();
            log!("Right arrow key pressed, skipping forward by {} seconds", ARROW_KEY_SKIP_TIME);

            if let Some(duration) = get_song_time_duration(status) {
                let mut time = duration.0 + ARROW_KEY_SKIP_TIME;
                time = time.clamp(0.0, duration.1);
                skip_to(status, time);
                set_playing(status, true);
            } else {
                error!("Unable to skip forward: Unable to get current duration");
            }

        } else if e.key() == "ArrowLeft" {
            e.prevent_default();
            log!("Left arrow key pressed, skipping backward by {} seconds", ARROW_KEY_SKIP_TIME);

            if let Some(duration) = get_song_time_duration(status) {
                let mut time = duration.0 - ARROW_KEY_SKIP_TIME;
                time = time.clamp(0.0, duration.1);
                skip_to(status, time);
                set_playing(status, true);
            } else {
                error!("Unable to skip backward: Unable to get current duration");
            }
        }
    });

    // Listen for space bar presses to play/pause
    let _space_bar_handle = window_event_listener(ev::keypress, move |e: ev::KeyboardEvent| {
        if e.key() == " " {
            e.prevent_default();
            log!("Space bar pressed, toggling play/pause");

            let playing = status.with_untracked(|status| status.playing);
            set_playing(status, !playing);
        }
    });

    // Keep a reference to the audio element so we can set its source and play/pause it
    let audio_ref = create_node_ref::<Audio>();
    status.update(|status| status.audio_player = Some(audio_ref));

    // Create signals for song time and progress
    let (elapsed_secs, set_elapsed_secs) = create_signal(0);
    let (total_secs, set_total_secs) = create_signal(0);
    let (percentage, set_percentage) = create_signal(0.0);

    audio_ref.on_load(move |audio| {
        log!("Audio element loaded");

        status.with_untracked(|status| {
            // Start playing the first song in the queue, if available
            if let Some(song) = status.queue.front() {
                log!("Starting playing with song: {}", song.title);

                // Don't use the set_play_src / set_playing helper function
                // here because we already have access to the audio element
                audio.set_src(&song.storage_path);

                if let Err(e) = audio.play() {
                    error!("Error playing audio on load: {:?}", e);
                } else {
                    log!("Audio playing on load");
                }
            } else {
                log!("Queue is empty, no first song to play");
            }
        });
    });

    let on_play = move |_| {
        log!("Audio playing");
        status.update(|status| status.playing = true);
    };

    let on_pause = move |_| {
        log!("Audio paused");
        status.update(|status| status.playing = false);
    };

    let on_time_update = move |_| {
        status.with_untracked(|status| {
            if let Some(audio) = status.get_audio() {
                set_elapsed_secs(audio.current_time() as i64);
                set_total_secs(audio.duration() as i64);

                if elapsed_secs.get_untracked() > 0 {
                    set_percentage(elapsed_secs.get_untracked() as f64 / total_secs.get_untracked() as f64 * 100.0);
                } else {
                    set_percentage(0.0);
                }
            } else {
                error!("Unable to update time: Audio element not available");
            }
        });
    };

    let on_end = move |_| {
        log!("Song ended");

        // Move the now-finshed song to the history
        // TODO Somehow make sure next song starts playing before repeatedly jumping to next
        status.update(|status| {
            let prev_song = status.queue.pop_front();

            if let Some(prev_song) = prev_song {
                log!("Adding song to history: {}", prev_song.title);
                status.history.push_back(prev_song);
            } else {
                log!("Queue empty, no previous song to add to history");
            }
        });

        // Get the next song to play, if available
        let next_src = status.with_untracked(|status| {
            status.queue.front().map(|song| song.storage_path.clone())
        });

        if let Some(audio) = audio_ref.get() {
            if let Some(next_src) = next_src {
                log!("Playing next song: {}", next_src);
                audio.set_src(&next_src);

                if let Err(e) = audio.play() {
                    error!("Error playing audio after song change: {:?}", e);
                } else {
                    log!("Audio playing after song change");
                }
            }
        } else {
            error!("Unable to play next song: Audio element not available");
        }
    };

    view! {
        <audio _ref=audio_ref on:play=on_play on:pause=on_pause
            on:timeupdate=on_time_update on:ended=on_end type="audio/mpeg" />
        <div class="playbar">
        <ProgressBar percentage=percentage.into() status=status />
        <MediaInfo status=status />
        <PlayControls status=status />
        <PlayDuration elapsed_secs=elapsed_secs.into() total_secs=total_secs.into() />
        <QueueToggle status=status />
        </div>
    }
}
