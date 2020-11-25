use chartgeneratorsvg::generate::DtoNote2 as DtoNote;
use chartgeneratorsvg::generate::Generate2;
use chartgeneratorsvg::generate::GenerateDto;
use std::str::FromStr;
use ukulele_midi::SoundBytes;
use ukulele_midi::Variant;

const VEC_NOTE: [&str; 12] = [
    "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
];

const VEC_CHORD: [&str; 13] = [
    "", "m", "sus2", "sus4", "aug", "dim", "7", "m7", "maj7", "aug7",
    "augMaj7", "dim7", "m7b5",
]; // TODO in chartgeneratorsvg a list

const VEC_FRET: [usize; 5] = [0, 5, 7, 10, 12];

fn main() {
    for note in VEC_NOTE.iter() {
        for chord in VEC_CHORD.iter() {}
    }
}

fn generate_wav(
    variant: &str,
    semitones: &[u8],
    sample_ukulele: Box<[u8]>,
) -> Vec<u8> {
    let mut sb: SoundBytes = SoundBytes {
        semitones_midi: semitones,
        midi: &mut Vec::new(),
        wav: &mut Vec::new(),
    };
    let v = Variant::from_str(variant).unwrap();
    sb.generate_from_local_asset(v).unwrap();
    sb.get_wav().to_vec()
}
