
use midi_msg::{ChannelVoiceMsg, MidiMsg, TrackEvent};
use midi_msg::{MidiFile, Track};
use std::env;
use std::fs;


#[derive(Default, Clone, Copy)]
struct Note {
    note: u8,
    velocity: u8,
    delay: u16,
    duration: u16,
}

impl Note {
    pub fn new(note: u8, velocity: u8,delay: u16, duration: u16) -> Self{
        Self {
            note,
            velocity,
            delay,
            duration
        }
    }

    pub fn note(self, note: u8) -> Self {
        Self {
            note,
            ..self
        }
    } 

    pub fn velocity(self, velocity: u8) -> Self {
        Self {
            velocity,
            ..self
        }
    }

    pub fn delay(self, delay: u16) -> Self {
        Self {
            delay,
            ..self
        }
    }

    pub fn duration(self, duration: u16) -> Self {
        Self {
            duration,
            ..self
        }
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
                        MidiMsg::ChannelVoice { channel, msg } => match *msg {
                            ChannelVoiceMsg::NoteOn { note, velocity } => {
                                println!("track event: {:?}", m);
                                note_tmp = note_tmp.note(note).velocity(velocity).delay(m.delta_time as u16);
                            }
                            ChannelVoiceMsg::NoteOff { note, velocity } => {
                                println!("track event: {:?}", m);
                                note_tmp = note_tmp.duration(m.delta_time as u16);
                                note_list.push(note_tmp);
                                
                            }
                            _ => {
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
    println!("len: {}, size: {}Kb", note_list.len(), note_list.len()*(16*3)/1024);
}
