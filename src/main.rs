use chartgeneratorsvg::generate::DtoNote;
use chartgeneratorsvg::interface::InterfaceRustToolAudioExport;
use chartgeneratorsvg::interface::InterfaceRustToolAudioExportChord;
use std::fs::File;
use std::io::prelude::*;
use std::str::FromStr;
use ukulele_midi::SoundBytes;
use ukulele_midi::Variant;

const VEC_NOTE: [&str; 12] = [
    "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
];

const VEC_CHORD: [&str; 13] = [
    "", "m", "sus2", "sus4", "aug", "dim", "7", "m7", "maj7", "aug7",
    "augMaj7", "dim7", "m7b5",
]; // TODO better

const VEC_FRET: [u8; 5] = [0, 5, 7, 10, 12]; // TODO better

const VARIATION: [&str; 3] = ["chord", "arp8", "arp4"]; // TODO better

const TUNING: [&str; 3] = ["C", "D", "G"];

fn main() -> std::io::Result<()> {
    for tuning in TUNING.iter() {
        for fret_position in VEC_FRET.iter() {
            for note in VEC_NOTE.iter() {
                for chord in VEC_CHORD.iter() {
                    let mut dto: Vec<DtoNote> = Vec::new();
                    let interface: InterfaceRustToolAudioExport =
                        InterfaceRustToolAudioExport::new();
                    dto.append(&mut interface.chord_list(
                        note,
                        chord,
                        *fret_position,
                        tuning,
                    ));
                    for v in VARIATION.iter() {
                        for d in dto.iter() {
                            for c in d.chord.iter() {
                                for (counter, data) in c.data.iter().enumerate()
                                {
                                    let vec_wav = generate_wav(
                                        v,
                                        &data.semitones[..] as &[u8],
                                    );
                                    let mut buffer = File::create(format!(
                                        "temp/{}-{}-{}-{}-{}-{}.wav",
                                        tuning,
                                        fret_position,
                                        chord,
                                        note,
                                        v,
                                        counter
                                    ))?; // can be done better... but this is a simple tool
                                    buffer.write_all(&vec_wav[..])?;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

fn generate_wav(variant: &str, semitones: &[u8]) -> Vec<u8> {
    let mut sb: SoundBytes = SoundBytes {
        semitones_midi: semitones,
        midi: &mut Vec::new(),
        wav: &mut Vec::new(),
    };
    let v = Variant::from_str(variant).unwrap();
    sb.generate_from_local_asset(v).unwrap();
    sb.get_wav().to_vec()
}
/*
// In scale the program freeze 1 sec on my iphone on selectbox
pub fn main() -> std::io::Result<()> {
    for fret_position in VEC_FRET.iter() {
        for note in VEC_NOTE.iter() {
            for chord in VEC_CHORD.iter() {
                let mut dto: Vec<DtoNote> = Vec::new();
                let interface: InterfaceRustToolAudioExport =
                    InterfaceRustToolAudioExport::new();
                dto.append(&mut interface.chord_list(
                    note,
                    chord,
                    *fret_position,
                ));
                for d in dto.iter() {
                    for c in d.chord.iter() {
                        for (counter, data) in c.data.iter().enumerate() {
                            let mut file = File::create(format!(
                                "temp_svg/{}-{}-{}-_-{}.svg",
                                fret_position, chord, note, counter
                            ))?; // can be done better... but this is a simple tool
                            file.write_all(data.svg.as_bytes())?;
                        }
                    }
                }
            }
        }
    }
    Ok(())
}*/
