use clap::Parser;
use midi_msg::{ChannelVoiceMsg, MidiMsg};
use midi_msg::{MidiFile, Track};
use std::fs;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    midi_file: String,
    output_name: String,
}

#[derive(Default, Clone, Copy, Debug)]
struct Note {
    note: u8,
    delay: u16,
}

impl Note {
    pub fn new(note: u8, delay: u16) -> Self {
        Self { note, delay }
    }

    pub fn note(self, note: u8) -> Self {
        Self { note, ..self }
    }

    pub fn delay(self, delay: u16) -> Self {
        Self { delay, ..self }
    }
}

fn main() {
    let args = Args::parse();

    let midi_file = args.midi_file.as_str();
    let midi = fs::read(midi_file).unwrap();

    let mut note_channel_list: Vec<Vec<Note>> = Vec::new();
    let mut note_list: Vec<Note> = Vec::new();

    let midi_msgs = MidiFile::from_midi(&midi).unwrap();
    let header = midi_msgs.header;
    println!("{:?}", header);
    let tracks = midi_msgs.tracks;
    println!("track count: {}", tracks.len());

    let mut note_tmp = Note::default();
    for track in &tracks {
        match track {
            Track::Midi(midi) => {
                for m in midi {
                    let event = &m.event;
                    println!("track event: {:?}", m);
                    match event {
                        MidiMsg::ChannelVoice { channel: _, msg } => match *msg {
                            ChannelVoiceMsg::NoteOn { note, velocity } => {
                                let note_tmp = if velocity == 0 {
                                    note_tmp.note(0).delay(m.delta_time as u16)
                                } else {
                                    note_tmp.note(note).delay(m.delta_time as u16)
                                };
                                note_list.push(note_tmp);
                            }
                            ChannelVoiceMsg::NoteOff {
                                note: _,
                                velocity: _,
                            } => {
                                // 关闭声音时，需要将note设置为0，否则会一直播放
                                note_tmp = note_tmp.note(0).delay(m.delta_time as u16);
                                note_list.push(note_tmp);
                            }
                            _ => {
                                continue;
                            }
                        },
                        _ => {}
                    }
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
        note_list.len() * 3 / 1024
    );

    let mut midi_rs_content = String::new();
    midi_rs_content.push_str(&format!(
        "pub struct Note {{
            note: u8,
            delay: u16,
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
    let _ = fs::write(format!("{}.rs", args.output_name), midi_rs_content).unwrap();

    println!("note size: {}", std::mem::size_of::<Note>());
}

enum Channel {
    Channel1(Note),
    Channel2(Note),
    Channel3(Note),
    Channel4(Note),
    Channel5(Note),
    Channel6(Note),
    Channel7(Note),
    Channel8(Note),
    Channel9(Note),
    Channel10(Note),
    Channel11(Note),
    Channel12(Note),
    Channel13(Note),
    Channel14(Note),
    Channel15(Note),
    Channel16(Note),
    Channel17(Note),
    Channel18(Note),
    Channel19(Note),
    Channel20(Note),
    Channel21(Note),
    Channel22(Note),
    Channel23(Note),
    Channel24(Note),
    Channel25(Note),
    Channel26(Note),
    Channel27(Note),
    Channel28(Note),
    Channel29(Note),
    Channel30(Note),
    Channel31(Note),
    Channel32(Note),
}

impl Channel {
    pub fn new(channel: u8, note: Note) -> Self {
        match channel {
            1 => Self::Channel1(note),
            2 => Self::Channel2(note),
            3 => Self::Channel3(note),
            4 => Self::Channel4(note),
            5 => Self::Channel5(note),
            6 => Self::Channel6(note),
            7 => Self::Channel7(note),
            8 => Self::Channel8(note),
            9 => Self::Channel9(note),
            10 => Self::Channel10(note),
            11 => Self::Channel11(note),
            12 => Self::Channel12(note),
            13 => Self::Channel13(note),
            14 => Self::Channel14(note),
            15 => Self::Channel15(note),
            16 => Self::Channel16(note),
            17 => Self::Channel17(note),
            18 => Self::Channel18(note),
            19 => Self::Channel19(note),
            20 => Self::Channel20(note),
            21 => Self::Channel21(note),
            22 => Self::Channel22(note),
            23 => Self::Channel23(note),
            24 => Self::Channel24(note),
            25 => Self::Channel25(note),
            26 => Self::Channel26(note),
            27 => Self::Channel27(note),
            28 => Self::Channel28(note),
            29 => Self::Channel29(note),
            30 => Self::Channel30(note),
            31 => Self::Channel31(note),
            32 => Self::Channel32(note),
            _ => Self::Channel1(note),
        }
    }
}
