// Based on: https://thenathannator.github.io/GuitarGame_ChartFormats/Chart-File-Formats/mid-format/Technical-Details/
type MidiTick = i64;
type MidiDataByte = u8; // typically 0-127, but can go higher
type MidiPitch = MidiDataByte;
type MidiChannel = i8; // 0-15

pub enum SMPTENumber {
    Fps24,
    Fps25,
    Fps29_97,
    Fps30,
}

impl SMPTENumber {
    pub fn fps(&self) -> f64 {
        match self {
            Self::Fps24 => 24.0,
            Self::Fps25 => 25.0,
            Self::Fps29_97 => 29.97,
            Self::Fps30 => 30.0,
        }
    }
}

impl SMPTENumber {
    fn try_from_bits(value: u8) -> Option<Self> {
        match value {
            0b00 => Some(Self::Fps24),
            0b01 => Some(Self::Fps25),
            0b10 => Some(Self::Fps29_97),
            0b11 => Some(Self::Fps30),
            _ => None,
        }
    }
}

pub enum SMPTENumberOrOther {
    SMPTENumber(SMPTENumber),
    Other(i8),
}

impl SMPTENumberOrOther {
    pub fn fps(&self) -> f64 {
        match self {
            Self::SMPTENumber(x) => x.fps(),
            Self::Other(a) => -(*a as f64), // best guess
        }
    }
}

impl SMPTENumberOrOther {
    fn from_twos_complement(value: i8) -> Self {
        match value {
            -24 => Self::SMPTENumber(SMPTENumber::Fps24),
            -25 => Self::SMPTENumber(SMPTENumber::Fps25),
            -29 => Self::SMPTENumber(SMPTENumber::Fps29_97),
            -30 => Self::SMPTENumber(SMPTENumber::Fps30),
            other => Self::Other(other),
        }
    }
}

pub enum MidiTimingType {
    TicksPerQuarterNote { resolution: u16 },
    TicksPerSMPTEFrame { resolution: u8, smpte: SMPTENumber },
}

pub struct RecentTempoChange {
    tick: MidiTick,
    position_seconds: f64,
    microseconds_per_beat: f64,
}

impl MidiTimingType {
    pub fn beats_from_ticks(&self, tick: MidiTick) -> f64 {
        match self {
            Self::TicksPerQuarterNote { resolution } => (tick as f64) / (*resolution as f64),
            _ => 0.0,
        }
    }

    pub fn seconds_from_ticks(&self, tick: MidiTick, most_recent_tempo_change: RecentTempoChange) -> f64 {
        match self {
            Self::TicksPerQuarterNote { .. } => {
                let beats_since_tempo_change = self.beats_from_ticks(most_recent_tempo_change.tick - tick);
                let seconds_since_tempo_change =
                    most_recent_tempo_change.microseconds_per_beat * beats_since_tempo_change;
                let seconds_total = most_recent_tempo_change.position_seconds + seconds_since_tempo_change;
                seconds_total
            }
            Self::TicksPerSMPTEFrame { resolution, smpte } => {
                let frames_passed = (tick as f64) / (*resolution as f64);
                let seconds_per_frame = 1.0 / smpte.fps();
                let seconds_total = frames_passed * seconds_per_frame;
                seconds_total
            }
        }
    }
}

pub struct MidiHeader {
    format_type: u16,
    number_of_tracks_metadata: u16,
    delta_time_type: MidiTimingType,
}

pub enum MidiText {
    String(String),
    Bytes(Vec<u8>),
}

pub enum MetaEvent {
    // 0xFF 0x00
    SequenceNumber {
        num: u16,
    },
    // 0xFF 0x01
    TextEvent {
        text: MidiText,
    },
    // 0xFF 0x02
    CopyrightNotice {
        text: MidiText,
    },
    // 0xFF 0x03
    TrackName {
        text: MidiText,
    },
    // 0xFF 0x04
    InstrumentName {
        text: MidiText,
    },
    // 0xFF 0x05
    Lyric {
        text: MidiText,
    },
    // 0xFF 0x06
    Marker {
        text: MidiText,
    },
    // 0xFF 0x07
    CuePoint {
        text: MidiText,
    },
    // 0xFF 0x20
    MIDIChannelPrefix {
        channel: MidiChannel,
    },
    // 0xFF 0x2F
    EndOfTrack,
    // 0xFF 0x51
    SetTempo {
        microseconds_per_beat: u32, // 24-bit (3-byte) number
    },
    // 0xFF 0x54
    SMPTEOffset {
        format: SMPTENumber, // embedded within the hour byte; the struct has 5 bytes in total
        hour: u8,
        minute: u8,
        second: u8,
        frame: u8,
        centiframe: u8, // 1/100th of a frame
    },
    // 0xFF 0x58
    TimeSignature {
        numerator: u8,
        denominator: u8,
        clocks: u8,
        base: u8,
    },
    // 0xFF 0x59
    KeySignature {
        sharp_count: i8, // +7 = 7 sharps; 0 = no sharps/flats; -7 = 7 flats
        minor: bool,
    },
    // 0xFF 0x7F
    VendorDefined {
        data: Vec<u8>,
    },
}

pub enum MidiEvent {
    // Channel events
    // 0x8n
    NoteOff {
        channel: MidiChannel,
        pitch: MidiPitch,
        velocity: MidiDataByte,
    },
    // 0x9n
    NoteOn {
        channel: MidiChannel,
        pitch: MidiPitch,
        velocity: MidiDataByte,
    },
    // 0xAn
    PolyphonicKeyPressure {
        channel: MidiChannel,
        pitch: MidiPitch,
        pressure: MidiDataByte,
    },
    // 0xBn
    ControlChange {
        // CC
        channel: MidiChannel,
        controller_number: MidiDataByte,
        value: MidiDataByte,
    },
    // 0xCn
    ProgramChange {
        channel: MidiChannel,
        program_number: MidiDataByte,
    },
    // 0xDn
    ChannelPressure {
        pressure: MidiDataByte,
    },
    // 0xEn
    PitchBend {
        bend: u16,
    },

    // System events
    // starts with 0xF0, ends with 0xF7
    SysEx {
        bytes: Vec<u8>,
    },
    // starts with 0xF7, no ending marker
    SysExNoEnd {
        bytes: Vec<u8>,
    },

    // Meta events
    // 0xFF
    Meta(MetaEvent),
}

pub struct MidiTrack {
    events: Vec<(MidiTick, MidiEvent)>,
}

pub enum MidiChunk {
    Header(MidiHeader),
    Track(MidiTrack),
}

pub struct MidiFile {
    chunks: Vec<MidiChunk>,
}
