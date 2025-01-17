//! Constants and definitions surrounding MIDI support.

pub use midi_consts::channel_event::control_change;

/// Determines which note events a plugin receives.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum MidiConfig {
    /// The plugin will not have a note input port and will thus not receive any not events.
    None,
    /// The plugin receives note on/off events, pressure, and potentially a couple standardized
    /// expression types depending on the plugin standard and host.
    Basic,
    /// The plugin receives full MIDI CCs as well as pitch bend information. For VST3 plugins this
    /// involves adding 130*16 parameters to bind to the the 128 MIDI CCs, pitch bend, and channel
    /// pressure.
    MidiCCs,
}

/// Event for (incoming) notes. The set of supported note events depends on the value of
/// [`Plugin::MIDI_INPUT`][crate::prelude::Plugin::MIDI_INPUT]. Also check out the
/// [`util`][crate::util] module for convenient conversion functions.
///
/// All of the timings are sample offsets withing the current buffer. All sample, channel and note
/// numbers are zero-indexed.
#[derive(Debug, Clone, Copy, PartialEq)]
#[non_exhaustive]
pub enum NoteEvent {
    /// A note on event, available on [`MidiConfig::Basic`] and up.
    NoteOn {
        timing: u32,
        /// The note's channel, from 0 to 16.
        channel: u8,
        /// The note's MIDI key number, from 0 to 127.
        note: u8,
        /// The note's velocity, from 0 to 1. Some plugin APIs may allow higher precision than the
        /// 127 levels available in MIDI.
        velocity: f32,
    },
    /// A note off event, available on [`MidiConfig::Basic`] and up.
    NoteOff {
        timing: u32,
        /// The note's channel, from 0 to 16.
        channel: u8,
        /// The note's MIDI key number, from 0 to 127.
        note: u8,
        /// The note's velocity, from 0 to 1. Some plugin APIs may allow higher precision than the
        /// 127 levels available in MIDI.
        velocity: f32,
    },
    /// A polyphonic note pressure/aftertouch event, available on [`MidiConfig::Basic`] and up. Not
    /// all hosts may support polyphonic aftertouch.
    ///
    /// # Note
    ///
    /// When implementing MPE support you should use MIDI channel pressure instead as polyphonic key
    /// pressure + MPE is undefined as per the MPE specification. Or as a more generic catch all,
    /// you may manually combine the polyphonic key pressure and MPE channel pressure.
    PolyPressure {
        timing: u32,
        /// The note's channel, from 0 to 16.
        channel: u8,
        /// The note's MIDI key number, from 0 to 127.
        note: u8,
        /// The note's pressure, from 0 to 1.
        pressure: f32,
    },
    /// A volume expression event, available on [`MidiConfig::Basic`] and up. Not all hosts may
    /// support these expressions.
    PolyVolume {
        timing: u32,
        /// The note's channel, from 0 to 16.
        channel: u8,
        /// The note's MIDI key number, from 0 to 127.
        note: u8,
        /// The note's voltage gain ratio, where 1.0 is unity gain.
        gain: f32,
    },
    /// A panning expression event, available on [`MidiConfig::Basic`] and up. Not all hosts may
    /// support these expressions.
    PolyPan {
        timing: u32,
        /// The note's channel, from 0 to 16.
        channel: u8,
        /// The note's MIDI key number, from 0 to 127.
        note: u8,
        /// The note's panning from, from -1 to 1, with -1 being panned hard left, and 1 being
        /// panned hard right.
        pan: f32,
    },
    /// A tuning expression event, available on [`MidiConfig::Basic`] and up. Not all hosts may support
    /// these expressions.
    PolyTuning {
        timing: u32,
        /// The note's channel, from 0 to 16.
        channel: u8,
        /// The note's MIDI key number, from 0 to 127.
        note: u8,
        /// The note's tuning in semitones, from -120 to 120.
        tuning: f32,
    },
    /// A vibrato expression event, available on [`MidiConfig::Basic`] and up. Not all hosts may support
    /// these expressions.
    PolyVibrato {
        timing: u32,
        /// The note's channel, from 0 to 16.
        channel: u8,
        /// The note's MIDI key number, from 0 to 127.
        note: u8,
        /// The note's vibrato amount, from 0 to 1.
        vibrato: f32,
    },
    /// A expression expression (yes, expression expression) event, available on
    /// [`MidiConfig::Basic`] and up. Not all hosts may support these expressions.
    PolyExpression {
        timing: u32,
        /// The note's channel, from 0 to 16.
        channel: u8,
        /// The note's MIDI key number, from 0 to 127.
        note: u8,
        /// The note's expression amount, from 0 to 1.
        expression: f32,
    },
    /// A brightness expression event, available on [`MidiConfig::Basic`] and up. Not all hosts may support
    /// these expressions.
    PolyBrightness {
        timing: u32,
        /// The note's channel, from 0 to 16.
        channel: u8,
        /// The note's MIDI key number, from 0 to 127.
        note: u8,
        /// The note's brightness amount, from 0 to 1.
        brightness: f32,
    },
    /// A MIDI channel pressure event, available on [`MidiConfig::MidiCCs`] and up.
    MidiChannelPressure {
        timing: u32,
        /// The affected channel, from 0 to 16.
        channel: u8,
        /// The pressure, normalized to `[0, 1]` to match the poly pressure event.
        pressure: f32,
    },
    /// A MIDI pitch bend, available on [`MidiConfig::MidiCCs`] and up.
    MidiPitchBend {
        timing: u32,
        /// The affected channel, from 0 to 16.
        channel: u8,
        /// The pressure, normalized to `[0, 1]`. `0.5` means no pitch bend.
        value: f32,
    },
    /// A MIDI control change event, available on [`MidiConfig::MidiCCs`] and up.
    ///
    /// # Note
    ///
    /// The wrapper does not perform any special handling for two message 14-bit CCs (where the CC
    /// number is in the range `[0, 31]`, and the next CC is that number plus 32) or for four
    /// message RPN messages. For now you will need to handle these CCs yourself.
    MidiCC {
        timing: u32,
        /// The affected channel, from 0 to 16.
        channel: u8,
        /// The control change number. See [`control_change`] for a list of CC numbers.
        cc: u8,
        /// The CC's value, normalized to `[0, 1]`. Multiply by 127 to get the original raw value.
        value: f32,
    },
}

impl NoteEvent {
    /// Returns the sample within the current buffer this event belongs to.
    pub fn timing(&self) -> u32 {
        match &self {
            NoteEvent::NoteOn { timing, .. } => *timing,
            NoteEvent::NoteOff { timing, .. } => *timing,
            NoteEvent::PolyPressure { timing, .. } => *timing,
            NoteEvent::PolyVolume { timing, .. } => *timing,
            NoteEvent::PolyPan { timing, .. } => *timing,
            NoteEvent::PolyTuning { timing, .. } => *timing,
            NoteEvent::PolyVibrato { timing, .. } => *timing,
            NoteEvent::PolyExpression { timing, .. } => *timing,
            NoteEvent::PolyBrightness { timing, .. } => *timing,
            NoteEvent::MidiChannelPressure { timing, .. } => *timing,
            NoteEvent::MidiPitchBend { timing, .. } => *timing,
            NoteEvent::MidiCC { timing, .. } => *timing,
        }
    }

    /// Subtract a sample offset from this event's timing, needed to compensate for the block
    /// splitting in the VST3 wrapper implementation because all events have to be read upfront.
    pub(crate) fn subtract_timing(&mut self, samples: u32) {
        match self {
            NoteEvent::NoteOn { timing, .. } => *timing -= samples,
            NoteEvent::NoteOff { timing, .. } => *timing -= samples,
            NoteEvent::PolyPressure { timing, .. } => *timing -= samples,
            NoteEvent::PolyVolume { timing, .. } => *timing -= samples,
            NoteEvent::PolyPan { timing, .. } => *timing -= samples,
            NoteEvent::PolyTuning { timing, .. } => *timing -= samples,
            NoteEvent::PolyVibrato { timing, .. } => *timing -= samples,
            NoteEvent::PolyExpression { timing, .. } => *timing -= samples,
            NoteEvent::PolyBrightness { timing, .. } => *timing -= samples,
            NoteEvent::MidiChannelPressure { timing, .. } => *timing -= samples,
            NoteEvent::MidiPitchBend { timing, .. } => *timing -= samples,
            NoteEvent::MidiCC { timing, .. } => *timing -= samples,
        }
    }
}
