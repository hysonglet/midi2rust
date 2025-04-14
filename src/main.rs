use clap::Parser;
use midi_msg::{ChannelVoiceMsg, Meta, MidiMsg};
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
    channel: u8,
    note: u8,
    delay: u16,
}

impl Note {
    pub fn new(channel: u8, note: u8, delay: u16) -> Self {
        Self {
            channel,
            note,
            delay,
        }
    }
}

fn main() {
    let args = Args::parse();

    let midi_file = args.midi_file.as_str();
    let midi = fs::read(midi_file).unwrap();

    let mut note_list: Vec<Note> = Vec::new();

    let midi_msgs = MidiFile::from_midi(&midi).unwrap();
    let header = midi_msgs.header;
    println!("{:?}", header);
    let tracks = midi_msgs.tracks;
    println!("track count: {}", tracks.len());

    let tpqn: u32 = header.division.beat_or_frame_to_tick(1.0);
    let mut bpm_ms = 1;
    let mut one_tick_ms = 1.0;
    println!(
        "tttt: {} {}",
        tpqn,
        header.division.ticks_to_beats_or_frames(1),
    );

    for track in &tracks {
        match track {
            Track::Midi(midi) => {
                for m in midi {
                    let event = &m.event;
                    let tick_ms: u16 = (m.delta_time as f32 * one_tick_ms) as u16;
                    println!("track event: {:?}", m);
                    match event {
                        MidiMsg::Meta { msg } => match *msg {
                            Meta::SetTempo(tempo) => {
                                let bpm_ms = tempo as f32 / 1000.0;
                                one_tick_ms = bpm_ms / tpqn as f32;
                                println!("tempo: {} bpm, one tick ms: {}", bpm_ms, one_tick_ms);
                            }
                            _ => {}
                        },
                        MidiMsg::ChannelVoice { channel, msg } => match *msg {
                            ChannelVoiceMsg::NoteOn { note, velocity } => {
                                note_list.push(if velocity == 0 {
                                    Note::new(*channel as u8, 0, tick_ms)
                                } else {
                                    Note::new(*channel as u8, note, tick_ms)
                                });
                            }
                            ChannelVoiceMsg::NoteOff {
                                note: _,
                                velocity: _,
                            } => {
                                // 关闭声音时，需要将note设置为0，否则会一直播放
                                note_list.push(Note::new(*channel as u8, 1, tick_ms));
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
        note_list.len() * 4 / 1024
    );

    let mut midi_rs_content = String::new();
    midi_rs_content.push_str(&format!(
        "struct Note {{
    channel: u8,
    note: u8,
    delay: u16,
    }}

    pub const MIDI_CONTENT: [Note; {}] = [\n\t",
        note_list.len()
    ));
    for note in note_list {
        midi_rs_content.push_str(&format!(
            "Note {{channel: {}, note: {},delay: {}}},\n",
            note.channel, note.note, note.delay
        ))
    }
    midi_rs_content.push_str(&format!("];"));

    let _ = fs::write(format!("{}.rs", args.output_name), midi_rs_content).unwrap();

    println!("note size: {}", std::mem::size_of::<Note>());
    println!("Channel size: {}", std::mem::size_of::<Note>());
}
