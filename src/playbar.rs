use crate::models::Artist;
use crate::songdata::SongData;
use crate::api::songs;
use crate::util::state::GlobalState;
use leptos::ev::MouseEvent;
use leptos::html::{Audio, Div};
use leptos::leptos_dom::*;
use leptos_meta::Title;
use leptos::prelude::*;
use leptos_icons::*;
use leptos_use::{utils::Pausable, use_interval_fn};
use leptos::task::spawn_local;

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

/// Threshold in seconds for considering when the user has listened to a song, for adding it to the history
const HISTORY_LISTEN_THRESHOLD: u64 = MIN_SKIP_BACK_TIME as u64;

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
pub fn get_song_time_duration() -> Option<(f64, f64)> {
	GlobalState::play_status().with_untracked(|status| {
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
pub fn skip_to(time: f64) {
    if time.is_infinite() || time.is_nan() {
        error!("Unable to skip to non-finite time: {}", time);
        return
    }

    GlobalState::play_status().update(|status| {
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
pub fn set_playing(play: bool) {
    GlobalState::play_status().update(|status| {
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

fn toggle_queue() {
	GlobalState::play_status().update(|status| {
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
fn set_play_src(src: String) {
    GlobalState::play_status().update(|status| {
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
fn PlayControls() -> impl IntoView {
    let status = GlobalState::play_status();

    // On click handlers for the skip and play/pause buttons

    let skip_back = move |_| {
        if let Some(duration) = get_song_time_duration() {
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
                    let next_src = last_played_song.song_path.clone();
                    status.update(|status| status.queue.push_front(last_played_song));
                    set_play_src(next_src);
                    set_playing(true);
                } else {
                    warn!("Unable to skip back: No previous song");
                }
            }
        }

        // Default to skipping to start of current song, and playing
        log!("Skipping to start of current song");
        skip_to(0.0);
        set_playing(true);
    };

    let skip_forward = move |_| {
		if let Some(duration) = get_song_time_duration() {
            skip_to(duration.1);
            set_playing(true);
		} else {
			error!("Unable to skip forward: Unable to get current duration");
		}
    };

    let toggle_play = move |_| {
        let playing = status.with_untracked(|status| { status.playing });
		set_playing(!playing);
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
                icondata::BsPauseFill
            } else {
                icondata::BsPlayFill
            }
        })
    });

    view! {
        <div class="playcontrols" >

        <button on:click=skip_back on:mousedown=prevent_focus>
        <Icon width=SKIP_BTN_SIZE height=SKIP_BTN_SIZE icon={icondata::BsSkipStartFill} {..} class="controlbtn" />
        </button>

        <button on:click=toggle_play on:mousedown=prevent_focus>
        <Icon width=PLAY_BTN_SIZE height=PLAY_BTN_SIZE icon={icon} {..} class="controlbtn" />
        </button>

        <button on:click=skip_forward on:mousedown=prevent_focus>
        <Icon width=SKIP_BTN_SIZE height=SKIP_BTN_SIZE icon={icondata::BsSkipEndFill} {..} class="controlbtn" />
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
        <div class="playduration" >
        {play_duration}
        </div>
    }
}

/// The name, artist, and album of the current song
#[component]
fn MediaInfo() -> impl IntoView {
    let status = GlobalState::play_status();

    let name = Signal::derive(move || {
		status.with(|status| {
			status.queue.front().map_or("No media playing".into(), |song| song.title.clone())
		})
    });

	let artist = Signal::derive(move || {
		status.with(|status| {
			status.queue.front().map_or("".into(), |song| format!("{}", Artist::display_list(&song.artists)))
		})
	});

	let album = Signal::derive(move || {
		status.with(|status| {
			status.queue.front().map_or("".into(), |song|
                song.album.as_ref().map_or("".into(), |album| album.title.clone()))
		})
	});

	let image = Signal::derive(move || {
		status.with(|status| {
			status.queue.front().map_or("/images/placeholders/MusicPlaceholder.svg".into(),
                |song| song.image_path.clone())
		})
	});

    view! {
        <img class="media-info-img" src={image}/>
        <div class="media-info-text">
            {name}
            <br/>
            {artist} - {album}
        </div>
    }
}

/// The like and dislike buttons
#[component]
fn LikeDislike() -> impl IntoView {
    let status = GlobalState::play_status();

    let like_icon = Signal::derive(move || {
        status.with(|status| {
            match status.queue.front() {
                Some(SongData { like_dislike: Some((true, _)), .. }) => icondata::TbThumbUpFilled,
                _ => icondata::TbThumbUp,
            }
        })
    });

    let dislike_icon = Signal::derive(move || {
        status.with(|status| {
            match status.queue.front() {
                Some(SongData { like_dislike: Some((_, true)), .. }) => icondata::TbThumbDownFilled,
                _ => icondata::TbThumbDown,
            }
        })
    });

    let toggle_like = move |_| {
        status.update(|status| {
            match status.queue.front_mut() {
                Some(SongData { id, like_dislike: Some((liked, disliked)), .. }) => {
                    *liked = !*liked;

                    if *liked {
                        *disliked = false;
                    }

                    let id = *id;
                    let liked = *liked;
                    spawn_local(async move {
                        if let Err(e) = songs::set_like_song(id, liked).await {
                            error!("Error liking song: {:?}", e);
                        }
                    });
                },
                Some(SongData { id, like_dislike, .. }) => {
                    // This arm should only be reached if like_dislike is None
                    // In this case, the buttons will show up not filled, indicating that the song is not
                    // liked or disliked. Therefore, clicking the like button should like the song.

                    *like_dislike = Some((true, false));

                    let id = *id;
                    spawn_local(async move {
                        if let Err(e) = songs::set_like_song(id, true).await {
                            error!("Error liking song: {:?}", e);
                        }
                    });
                },
                _ => {
                    log!("Unable to like song: No song in queue");
                    return;
                }
            }
        });
    };

    let toggle_dislike = move |_| {
        status.update(|status| {
            match status.queue.front_mut() {
                Some(SongData { id, like_dislike: Some((liked, disliked)), .. }) => {
                    *disliked = !*disliked;

                    if *disliked {
                        *liked = false;
                    }

                    let id = *id;
                    let disliked = *disliked;
                    spawn_local(async move {
                        if let Err(e) = songs::set_dislike_song(id, disliked).await {
                            error!("Error disliking song: {:?}", e);
                        }
                    });
                },
                Some(SongData { id, like_dislike, .. }) => {
                    // This arm should only be reached if like_dislike is None
                    // In this case, the buttons will show up not filled, indicating that the song is not
                    // liked or disliked. Therefore, clicking the dislike button should dislike the song.
                    
                    *like_dislike = Some((false, true));

                    let id = *id;
                    spawn_local(async move { 
                        if let Err(e) = songs::set_dislike_song(id, true).await {
                            error!("Error disliking song: {:?}", e);
                        }
                    });
                },
                _ => {
                    log!("Unable to dislike song: No song in queue");
                    return;
                }
            }
        });
    };

    view! {
        <div class="like-dislike">
            <button on:click=toggle_dislike>
                <Icon width=SKIP_BTN_SIZE height=SKIP_BTN_SIZE icon={dislike_icon} {..} class="controlbtn hmirror" />
            </button>
            <button on:click=toggle_like>
                <Icon width=SKIP_BTN_SIZE height=SKIP_BTN_SIZE icon={like_icon} {..} class="controlbtn" />
            </button>
        </div>
    }
}

/// The play progress bar, and click handler for skipping to a certain time in the song
#[component]
fn ProgressBar(percentage: MaybeSignal<f64>) -> impl IntoView {
    // Keep a reference to the progress bar div so we can get its width and calculate the time to skip to
    let progress_bar_ref = NodeRef::<Div>::new();

    let progress_jump = move |e: MouseEvent| {
        let x_click_pos = e.offset_x() as f64;
        log!("Progress bar clicked at x: {}", x_click_pos);

        if let Some(progress_bar) = progress_bar_ref.get() {
            let width = progress_bar.offset_width() as f64;
            let percentage = x_click_pos / width * 100.0;

			if let Some(duration) = get_song_time_duration() {
				let time = duration.1 * percentage / 100.0;
				skip_to(time);
                set_playing(true);
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
        <div class="invisible-media-progress" node_ref=progress_bar_ref on:click=progress_jump> // Larger click area
        <div class="media-progress"> // "Unfilled" progress bar
        <div class="media-progress-solid" style=bar_width_style> // "Filled" progress bar
		</div>
        </div>
        </div>
    }
}

#[component]
fn QueueToggle() -> impl IntoView {
    let update_queue = move |_| {
        toggle_queue();
		log!("queue button pressed, queue status: {:?}",
            GlobalState::play_status().with_untracked(|status| status.queue_open));
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
        <Icon width=QUEUE_BTN_SIZE height=QUEUE_BTN_SIZE icon={icondata::RiPlayListMediaFill} {..} class="controlbtn" />
        </button>
        </div>
    }
}

/// Renders the title of the page based on the currently playing song
#[component]
pub fn CustomTitle() -> impl IntoView {
    let title = create_memo(move |_| {
        GlobalState::play_status().with(|play_status| {
            play_status.queue.front().map_or("LibreTunes".to_string(), |song_data| {
                    format!("{} - {} | {}",song_data.title.clone(),Artist::display_list(&song_data.artists), "LibreTunes")
                })
            })
        });
    view! {
        <Title text=title />
    }
}

/// The main play bar component, containing the progress bar, media info, play controls, and play duration
#[component]
pub fn PlayBar() -> impl IntoView {
    let status = GlobalState::play_status();

    // Listen for key down events -- arrow keys don't seem to trigger key press events
    let _arrow_key_handle = window_event_listener(leptos::ev::keydown, move |e: leptos::ev::KeyboardEvent| {
        if e.key() == "ArrowRight" {
            e.prevent_default();
            log!("Right arrow key pressed, skipping forward by {} seconds", ARROW_KEY_SKIP_TIME);

            if let Some(duration) = get_song_time_duration() {
                let mut time = duration.0 + ARROW_KEY_SKIP_TIME;
                time = time.clamp(0.0, duration.1);
                skip_to(time);
                set_playing(true);
            } else {
                error!("Unable to skip forward: Unable to get current duration");
            }

        } else if e.key() == "ArrowLeft" {
            e.prevent_default();
            log!("Left arrow key pressed, skipping backward by {} seconds", ARROW_KEY_SKIP_TIME);

            if let Some(duration) = get_song_time_duration() {
                let mut time = duration.0 - ARROW_KEY_SKIP_TIME;
                time = time.clamp(0.0, duration.1);
                skip_to(time);
                set_playing(true);
            } else {
                error!("Unable to skip backward: Unable to get current duration");
            }
        }
    });

    // Listen for space bar presses to play/pause
    let _space_bar_handle = window_event_listener(leptos::ev::keypress, move |e: leptos::ev::KeyboardEvent| {
        if e.key() == " " {
            e.prevent_default();
            log!("Space bar pressed, toggling play/pause");

            let playing = status.with_untracked(|status| status.playing);
            set_playing(!playing);
        }
    });

    // Keep a reference to the audio element so we can set its source and play/pause it
    let audio_ref = NodeRef::<Audio>::new();
    status.update(|status| status.audio_player = Some(audio_ref));

    // Create signals for song time and progress
    let (elapsed_secs, set_elapsed_secs) = signal(0);
    let (total_secs, set_total_secs) = signal(0);
    let (percentage, set_percentage) = signal(0.0);

    audio_ref.on_load(move |audio| {
        log!("Audio element loaded");

        status.with_untracked(|status| {
            // Start playing the first song in the queue, if available
            if let Some(song) = status.queue.front() {
                log!("Starting playing with song: {}", song.title);

                // Don't use the set_play_src / set_playing helper function
                // here because we already have access to the audio element
                audio.set_src(&song.song_path);

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

    let current_song_id = create_memo(move |_| {
        status.with(|status| {
            status.queue.front().map(|song| song.id)
        })
    });

    // Track the last song that was added to the history to prevent duplicates
    let last_history_song_id = RwSignal::new(None);

    let Pausable { 
        is_active: hist_timeout_pending,
        resume: resume_hist_timeout,
        pause: pause_hist_timeout,
        ..
    } = use_interval_fn(move || {
        if last_history_song_id.get_untracked() == current_song_id.get_untracked() {
            return;
        }

        if let Some(current_song_id) = current_song_id.get_untracked() {
            last_history_song_id.set(Some(current_song_id));

            spawn_local(async move {
                if let Err(e) = crate::api::history::add_history(current_song_id).await {
                    error!("Error adding song {} to history: {}", current_song_id, e);
                }
            });
        }
    }, HISTORY_LISTEN_THRESHOLD * 1000);

    // Initially pause the timeout, since the audio starts off paused
    pause_hist_timeout();

    let on_play = move |_| {
        log!("Audio playing");
        status.update(|status| status.playing = true);
    };

    let on_pause = move |_| {
        log!("Audio paused");
        status.update(|status| status.playing = false);
        pause_hist_timeout();
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

        // If time is updated, audio is playing, so make sure the history timeout is running
        if !hist_timeout_pending.get_untracked() {
            resume_hist_timeout();
        }
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
            status.queue.front().map(|song| song.song_path.clone())
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
        <audio node_ref=audio_ref on:play=on_play on:pause=on_pause
            on:timeupdate=on_time_update on:ended=on_end />
        <div class="playbar">
        <ProgressBar percentage=percentage.into() />
        <div class="playbar-left-group">
        <MediaInfo />
        <LikeDislike />
        </div>
        <PlayControls />
        <PlayDuration elapsed_secs=elapsed_secs.into() total_secs=total_secs.into() />
        <QueueToggle />
        </div>
    }
}
