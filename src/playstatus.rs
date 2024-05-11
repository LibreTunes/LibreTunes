use leptos::HtmlElement;
use leptos::NodeRef;
use leptos::html::Audio;
use std::collections::VecDeque;

use crate::songdata::SongData;

/// Represents the global state of the audio player feature of LibreTunes
pub struct PlayStatus {
	/// Whether or not the audio player is currently playing
    pub playing: bool,
	/// Whether or not the queue is open
	pub queue_open: bool,
	/// A reference to the HTML audio element
    pub audio_player: Option<NodeRef<Audio>>,
	/// A queue of songs that have been played, ordered from oldest to newest
    pub history: VecDeque<SongData>,
	/// A queue of songs that have yet to be played, ordered from next up to last
    pub queue: VecDeque<SongData>,
	/// Whether the current playing song is liked
	pub liked: bool,
	/// Whether the current playing song is disliked
	pub disliked: bool,
}

impl PlayStatus {
	/// Returns the HTML audio element if it has been created and is present, otherwise returns None
	/// 
	/// Instead of:
	/// ```
	/// let status = libretunes::playstatus::PlayStatus::default();
	/// if let Some(audio) = status.audio_player {
	/// 	if let Some(audio) = audio.get() {
	/// 		let _ = audio.play();
	/// 	}
	/// }
	/// ```
	/// 
	/// You can do:
	/// ```
	/// let status = libretunes::playstatus::PlayStatus::default();
	/// if let Some(audio) = status.get_audio() {
	/// 	let _ = audio.play();
	/// }
	/// ```
	pub fn get_audio(&self) -> Option<HtmlElement<Audio>> {
        if let Some(audio) = &self.audio_player {
            if let Some(audio) = audio.get() {
                return Some(audio);
            }
        }

        None
    }
}

impl Default for PlayStatus {
	/// Creates a paused PlayStatus with no audio player, no progress update handle, and empty queue/history
    fn default() -> Self {
        Self {
            playing: false,
			queue_open: false,
            audio_player: None,
            history: VecDeque::new(),
            queue: VecDeque::new(),
			liked: false,
			disliked: false,
        }
    }
}
