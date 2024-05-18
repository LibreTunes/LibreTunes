use symphonia::core::codecs::CodecType;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;
use std::fs::File;

/// Extract the codec and duration of an audio file
/// This is combined into one function because the file object will be consumed
pub fn extract_metadata(file: File) -> Result<(CodecType, u64), Box<dyn std::error::Error>> {
	let source_stream = MediaSourceStream::new(Box::new(file), Default::default());

	let hint = Hint::new();
	let format_opts = FormatOptions::default();
	let metadata_opts = MetadataOptions::default();

	let probe = symphonia::default::get_probe().format(&hint, source_stream, &format_opts, &metadata_opts)?;
	let reader = probe.format;

	if reader.tracks().len() != 1 {
		return Err(format!("Expected 1 track, found {}", reader.tracks().len()).into())
	}

	let track = &reader.tracks()[0];

	let time_base = track.codec_params.time_base.ok_or("Missing time base")?;
	let duration = track.codec_params.n_frames
		.map(|frames| track.codec_params.start_ts + frames)
		.ok_or("Missing number of frames")?;

	let duration = duration
		.checked_mul(time_base.numer as u64)
		.and_then(|v| v.checked_div(time_base.denom as u64))
		.ok_or("Overflow while computing duration")?;

	Ok((track.codec_params.codec, duration))
}
