use midi_msg::{ChannelVoiceMsg, MidiMsg, TrackEvent};
use midi_msg::{MidiFile, Track};
use std::env;
use std::fs;

// mod wirte;

#[derive(Default, Clone, Copy, Debug)]
struct Note {
    note: u8,
    delay: u8,
}

impl Note {
    pub fn new(note: u8, delay: u8) -> Self {
        Self { note, delay }
    }

    pub fn note(self, note: u8) -> Self {
        Self { note, ..self }
    }

    pub fn delay(self, delay: u8) -> Self {
        Self { delay, ..self }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        return;
    }

    let mut note_list: Vec<Note> = Vec::new();

    let midi_file = args[1].as_str();

    let midi = fs::read(midi_file).unwrap();

    let midi_msgs = MidiFile::from_midi(&midi).unwrap();
    let header = midi_msgs.header;
    println!("{:?}", header);
    let tracks = midi_msgs.tracks;
    println!("track count: {}", tracks.len());

    let mut note_tmp = Note::default();
    for track in &tracks {
        // println!("{:?}", track)

        match track {
            Track::Midi(midi) => {
                for m in midi {
                    let event = &m.event;

                    match event {
                        MidiMsg::ChannelVoice { channel: _, msg } => match *msg {
                            ChannelVoiceMsg::NoteOn { note, velocity: _ } => {
                                println!("track event: {:?}", m);
                                note_tmp = note_tmp.note(note).delay(m.delta_time as u8);
                                note_list.push(note_tmp);
                            }
                            ChannelVoiceMsg::NoteOff { note, velocity: _ } => {
                                println!("track event: {:?}", m);
                                note_tmp = note_tmp.note(note).delay(0);
                                note_list.push(note_tmp);
                            }
                            _ => {
                                println!("track event: {:?}", m);
                                continue;
                            }
                        },
                        _ => {}
                    }

                    // println!("track event: {:?}", m);
                }
            }
            Track::AlienChunk(alien_chunk) => {
                for a in alien_chunk {
                    println!("alien chunk: {}", a);
                }
            }
        }
    }

    // println!("size: {}KB", size_of_val(&MIDI_CONTENT) / 1025);
    println!(
        "len: {}, size: {}Kb",
        note_list.len(),
        note_list.len() * 2 / 1024
    );

    let mut midi_rs_content = String::new();
    midi_rs_content.push_str(&format!(
        "pub struct Note {{
        note: u8,
        delay: u8,
    }}
    pub const MIDI_CONTENT: [Note; {}] = [\n\t",
        note_list.len()
    ));
    for note in note_list {
        midi_rs_content.push_str(&format!(
            "Note {{note: {},delay: {}}},\n",
            note.note, note.delay
        ))
    }
    midi_rs_content.push_str(&format!("];"));
    let _ = fs::write("src/wirte.rs", midi_rs_content).unwrap();
}

const NOTE_LIST: [Note; 2] = [
    Note {
        note: 60,
        delay: 10,
    },
    Note {
        note: 64,
        delay: 10,
    },
];
