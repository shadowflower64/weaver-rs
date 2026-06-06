pub enum DrumsTrackContentType {
    FourLaneDrums,
    FiveLaneDrums,
    ProDrums,
}

pub enum TrackContentType {
    Guitar5F,
    Guitar6F,
    ProGuitar17F,
    ProGuitar22F,
    Drums(DrumsTrackContentType),
    EliteDrums,
    Vocals,
    Harm1,
    Harm2,
    Harm3,
    Keys,
    ProKeysX,
    ProKeysLD,
    ProKeysAnim,
    Events,
    Beat,
    Venue,
}

pub enum StandardPartTrackName {
    PartGuitar,
    PartGuitarGHL,
    PartRealGuitar,
    PartRealGuitar22,
    // PartRealGuitarBonus, // weird
    PartBass,
    PartBassGHL,
    PartRealBass,
    PartRealBass22,
    PartRhythmGuitar,
    PartRhythmGuitarGHL,
    PartRealRhythmGuitar,   // Not in spec yet
    PartRealRhythmGuitar22, // Not in spec yet
    PartGuitarCoop,
    PartGuitarCoopGHL,
    PartRealGuitarCoop,   // Not in spec yet
    PartRealGuitarCoop22, // Not in spec yet
    PartDrums,
    PartEliteDrums, // Not in spec yet
    PartVocals,
    Harm1,
    Harm2,
    Harm3,
    PartKeys,
    // PartKeysGHL, // weird
    PartRealKeysX,
    PartRealKeysH,
    PartRealKeysM,
    PartRealKeysE,
}

pub enum StandardMiscTrackName {
    PartKeysAnimRH,
    PartKeysAnimLH,
    Events,
    Beat,
    Venue,
}

pub enum StandardTrackName {
    Part(StandardPartTrackName),
    Misc(StandardMiscTrackName),
}

pub enum MidiChartTrackName {
    Standard(StandardTrackName),
    Unrecognized(String),
}

impl From<&str> for MidiChartTrackName {
    fn from(value: &str) -> Self {
        use MidiChartTrackName::Standard as S;
        use StandardMiscTrackName as M;
        use StandardPartTrackName as P;
        use StandardTrackName::Misc;
        use StandardTrackName::Part;
        match value {
            "PART GUITAR" | "T1 GEMS" => S(Part(P::PartGuitar)),
            "PART GUITAR GHL" => S(Part(P::PartGuitarGHL)),
            "PART REAL_GUITAR" => S(Part(P::PartRealGuitar)),
            "PART REAL_GUITAR_22" => S(Part(P::PartRealGuitar22)),
            // "PART REAL_GUITAR_BONUS" => S(Part(Part::PartRealGuitarBonus)),
            "PART BASS" => S(Part(P::PartBass)),
            "PART BASS GHL" => S(Part(P::PartBassGHL)),
            "PART REAL_BASS" => S(Part(P::PartRealBass)),
            "PART REAL_BASS_22" => S(Part(P::PartRealBass22)),
            "PART RHYTHM" => S(Part(P::PartRhythmGuitar)),
            "PART RHYTHM GHL" => S(Part(P::PartRhythmGuitarGHL)),
            "PART REAL_RHYTHM" => S(Part(P::PartRealRhythmGuitar)),
            "PART REAL_RHYTHM_22" => S(Part(P::PartRealRhythmGuitar22)),
            "PART GUITAR COOP" => S(Part(P::PartGuitarCoop)),
            "PART GUITAR COOP GHL" => S(Part(P::PartGuitarCoopGHL)),
            "PART REAL_GUITAR COOP" => S(Part(P::PartRealGuitarCoop)),
            "PART REAL_GUITAR COOP_22" => S(Part(P::PartRealGuitarCoop22)),
            "PART DRUMS" | "PART DRUM" | "PART DRUMS_2X" | "PART REAL_DRUMS_PS" => S(Part(P::PartDrums)),
            "PART VOCALS" => S(Part(P::PartVocals)),
            "HARM1" | "PART HARM1" => S(Part(P::Harm1)),
            "HARM2" | "PART HARM2" => S(Part(P::Harm2)),
            "HARM3" | "PART HARM3" => S(Part(P::Harm3)),
            "PART KEYS" => S(Part(P::PartKeys)),
            "PART REAL_KEYS_X" => S(Part(P::PartRealKeysX)),
            "PART REAL_KEYS_H" => S(Part(P::PartRealKeysH)),
            "PART REAL_KEYS_M" => S(Part(P::PartRealKeysM)),
            "PART REAL_KEYS_E" => S(Part(P::PartRealKeysE)),
            // "PART KEYS GHL" => S(Part(P::PartKeysGHL)),
            "PART KEYS_ANIM_RH" => S(Misc(M::PartKeysAnimRH)),
            "PART KEYS_ANIM_LH" => S(Misc(M::PartKeysAnimLH)),
            "EVENTS" => S(Misc(M::Events)),
            "BEAT" => S(Misc(M::Beat)),
            "VENUE" => S(Misc(M::Venue)),
            unrecognized => MidiChartTrackName::Unrecognized(unrecognized.to_owned()),
        }
    }
}

pub struct ChartMidi {}
