const VEC_NOTE: [&str; 12] = [
    "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
];

const VEC_CHORD: [&str; 13] = [
    "", "m", "sus2", "sus4", "aug", "dim", "7", "m7", "maj7", "aug7",
    "augMaj7", "dim7", "m7b5",
]; // TODO in chartgeneratorsvg a list

fn main() {
    let mut vec_note_chord: Vec<String> = Vec::new();
    for note in VEC_NOTE.iter() {
        for chord in VEC_CHORD.iter() {
            vec_note_chord.push(format!("{}{}", note, chord));
        }
    }
    for action in vec_note_chord.iter() {
        println!("{}", action);
    }
}
