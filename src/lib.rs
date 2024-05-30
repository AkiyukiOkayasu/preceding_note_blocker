use nih_plug::prelude::*;
use std::sync::Arc;

struct PrecedingNoteBlocker {
    params: Arc<PrecedingNoteBlockerParams>,
    midi_note_states: [[bool; 128]; 16],
}

#[derive(Default, Params)]
struct PrecedingNoteBlockerParams {}

impl Default for PrecedingNoteBlocker {
    fn default() -> Self {
        Self {
            params: Arc::new(PrecedingNoteBlockerParams::default()),
            midi_note_states: [[false; 128]; 16],
        }
    }
}

impl Plugin for PrecedingNoteBlocker {
    const NAME: &'static str = "Preceding Note Blocker";
    const VENDOR: &'static str = "Akiyuki Okayasu";
    const URL: &'static str = env!("CARGO_PKG_HOMEPAGE");
    const EMAIL: &'static str = "akiyuki.okayasu@gmail.com";

    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    // This plugin doesn't have any audio I/O
    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[];

    const MIDI_INPUT: MidiConfig = MidiConfig::Basic;
    const MIDI_OUTPUT: MidiConfig = MidiConfig::Basic;
    const SAMPLE_ACCURATE_AUTOMATION: bool = true;

    // If the plugin can send or receive SysEx messages, it can define a type to wrap around those
    // messages here. The type implements the `SysExMessage` trait, which allows conversion to and
    // from plain byte buffers.
    type SysExMessage = ();
    // More advanced plugins can use this to run expensive background tasks. See the field's
    // documentation for more information. `()` means that the plugin does not have any background
    // tasks.
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn initialize(
        &mut self,
        _audio_io_layout: &AudioIOLayout,
        _buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>,
    ) -> bool {
        // Resize buffers and perform other potentially expensive initialization operations here.
        // The `reset()` function is always called right after this function. You can remove this
        // function if you do not need it.
        true
    }

    fn reset(&mut self) {
        // Reset buffers and envelopes here. This can be called from the audio thread and may not
        // allocate. You can remove this function if you do not need it.

        // Reset all note states
        for channel in self.midi_note_states.iter_mut() {
            for note_state in channel.iter_mut() {
                *note_state = false;
            }
        }
    }

    fn process(
        &mut self,
        _buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        while let Some(event) = context.next_event() {
            match event {
                NoteEvent::NoteOn {
                    timing,
                    voice_id,
                    channel,
                    note,
                    velocity,
                } => {
                    if velocity > 0.0f32 {
                        // note-on
                        if check_set_note_state(channel, note, &mut self.midi_note_states) {
                            context.send_event(NoteEvent::NoteOn {
                                timing,
                                voice_id,
                                channel,
                                note,
                                velocity,
                            })
                        }
                    } else {
                        // note-off
                        off_note_state(channel, note, &mut self.midi_note_states);
                        context.send_event(NoteEvent::NoteOff {
                            timing,
                            voice_id,
                            channel,
                            note,
                            velocity,
                        })
                    }
                }
                NoteEvent::NoteOff {
                    timing,
                    voice_id,
                    channel,
                    note,
                    velocity,
                } => {
                    off_note_state(channel, note, &mut self.midi_note_states);
                    context.send_event(NoteEvent::NoteOff {
                        timing,
                        voice_id,
                        channel,
                        note,
                        velocity,
                    })
                }
                _ => (),
            }
        }
        ProcessStatus::Normal
    }
}

/// ノート管理の状態をチェックし、ノートオンを許可するかどうかを返す
///
/// # Arguments
/// * `channel` - MIDIチャンネル番号
/// * `note` - MIDIノート番号
/// * `note_state` - ノート管理の状態
///
/// # Returns
/// * `bool` - ノートオンを許可する場合はtrue、それ以外はfalse
fn check_set_note_state(channel: u8, note: u8, note_state: &mut [[bool; 128]; 16]) -> bool {
    let state = note_state[channel as usize][note as usize];
    note_state[channel as usize][note as usize] = true;

    // すでにtrueがセットされているときはノートonを許可しない
    !state
}

/// ノート管理の状態をリセットする
///
/// # Arguments
/// * `channel` - MIDIチャンネル番号
/// * `note` - MIDIノート番号
/// * `note_state` - ノート管理の状態
fn off_note_state(channel: u8, note: u8, note_state: &mut [[bool; 128]; 16]) {
    note_state[channel as usize][note as usize] = false;
}

impl ClapPlugin for PrecedingNoteBlocker {
    const CLAP_ID: &'static str = "com.groundless-electronics.prece-blocker";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("Do not allow retriggering when a note-on of the same MIDI note comes while playing a MIDI note.");
    const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
    const CLAP_SUPPORT_URL: Option<&'static str> = None;

    // Don't forget to change these features
    const CLAP_FEATURES: &'static [ClapFeature] = &[ClapFeature::NoteEffect, ClapFeature::Utility];
}

impl Vst3Plugin for PrecedingNoteBlocker {
    const VST3_CLASS_ID: [u8; 16] = *b"precedingnoteblo";

    // And also don't forget to change these categories
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] =
        &[Vst3SubCategory::Instrument, Vst3SubCategory::Tools];
}

nih_export_clap!(PrecedingNoteBlocker);
nih_export_vst3!(PrecedingNoteBlocker);
