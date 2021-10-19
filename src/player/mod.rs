/**
 * MIT License
 *
 * termusic - Copyright (c) 2021 Larry Hao
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */
#[allow(non_camel_case_types, non_upper_case_globals, unused)]
mod vlc;
use anyhow::{bail, Result};
// use std::sync::Arc;
// use std::thread;
// use std::marker::{Send, Sync};
#[cfg(feature = "mpris")]
use crate::song::Song;
#[cfg(feature = "mpris")]
use crate::souvlaki::{
    MediaControlEvent, MediaControls, MediaMetadata, MediaPlayback, PlatformConfig,
};
#[cfg(feature = "mpris")]
use std::str::FromStr;
use std::sync::mpsc::channel;
#[cfg(feature = "mpris")]
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
use std::thread::sleep;
use std::time::Duration;
use vlc::{Instance, Media, MediaPlayer, MediaPlayerAudioEx};

pub struct Vlc {
    paused: bool,
    volume: i32,
    sender_command: Sender<VlcCommand>,
    #[cfg(feature = "mpris")]
    controls: MediaControls,
    #[cfg(feature = "mpris")]
    pub rx: Receiver<MediaControlEvent>,
}

// unsafe impl Send for GSTPlayer {}
// unsafe impl Sync for GSTPlayer {}

enum VlcCommand {
    Play(String),
    Pause,
    Resume,
    SetVolume(i32),
}

impl Vlc {
    pub fn new() -> Self {
        #[cfg(feature = "mpris")]
        let config = PlatformConfig {
            dbus_name: "termusic",
            display_name: "Termuisc in Rust",
        };

        #[cfg(feature = "mpris")]
        let mut controls = MediaControls::new(config);

        #[cfg(feature = "mpris")]
        let (tx, rx) = mpsc::sync_channel(32);
        // The closure must be Send and have a static lifetime.
        #[cfg(feature = "mpris")]
        controls
            .attach(move |event: MediaControlEvent| {
                tx.send(event).ok();
            })
            .unwrap();
        let (tx2, rx2) = channel::<VlcCommand>();

        thread::spawn(move || {
            if let Some(instance) = Instance::new() {
                if let Some(player) = MediaPlayer::new(&instance) {
                    loop {
                        if let Ok(command) = rx2.try_recv() {
                            match command {
                                VlcCommand::Play(song_str) => {
                                    let md = Media::new_path(&instance, song_str).unwrap();
                                    player.set_media(&md);
                                    player.play().ok();
                                }
                                VlcCommand::Pause => player.pause(),
                                VlcCommand::Resume => {
                                    player.play().ok();
                                }
                                VlcCommand::SetVolume(volume) => {
                                    player.set_volume(volume).ok();
                                } // _ => {}
                            }
                        }
                        sleep(Duration::from_millis(200));
                    }
                }
            }
        });

        Self {
            paused: false,
            volume: 50,
            sender_command: tx2,
            #[cfg(feature = "mpris")]
            controls,
            #[cfg(feature = "mpris")]
            rx,
        }
    }

    #[allow(clippy::cast_sign_loss)]
    pub fn duration(song_str: &str) -> Duration {
        let instance = Instance::new().unwrap();
        let md = Media::new_path(&instance, song_str).unwrap();
        match md.duration() {
            Some(d) => Duration::from_secs(d as u64),
            None => Duration::from_secs(0),
        }
    }

    pub fn add_and_play(&mut self, song_str: &str) {
        self.paused = false;
        self.sender_command
            .send(VlcCommand::Play(song_str.to_string()))
            .ok();

        #[cfg(feature = "mpris")]
        if let Ok(song) = Song::from_str(song_str) {
            self.controls.set_metadata(MediaMetadata {
                title: Some(song.title().unwrap_or("Unknown Title")),
                artist: Some(song.artist().unwrap_or("Unknown Artist")),
                album: Some(song.album().unwrap_or("")),
                ..MediaMetadata::default()
            });
        }
        #[cfg(feature = "mpris")]
        self.controls
            .set_playback(MediaPlayback::Playing { progress: None })
            .ok();
    }

    pub fn volume_up(&mut self) {
        self.volume += 5;
        if self.volume > 100 {
            self.volume = 100;
        }
        self.set_volume(self.volume);
    }

    pub fn volume_down(&mut self) {
        self.volume -= 5;
        if self.volume < 0 {
            self.volume = 0;
        }
        self.set_volume(self.volume);
    }

    pub const fn volume(&self) -> i32 {
        self.volume
    }

    pub fn set_volume(&mut self, mut volume: i32) {
        if volume > 100 {
            volume = 100;
        } else if volume < 0 {
            volume = 0;
        }
        self.volume = volume;
        self.sender_command
            .send(VlcCommand::SetVolume(self.volume))
            .ok();
    }

    pub fn pause(&mut self) {
        self.paused = true;
        self.sender_command.send(VlcCommand::Pause).ok();

        #[cfg(feature = "mpris")]
        self.controls
            .set_playback(MediaPlayback::Paused { progress: None })
            .ok();
    }

    pub fn resume(&mut self) {
        self.paused = false;
        self.sender_command.send(VlcCommand::Resume).ok();

        #[cfg(feature = "mpris")]
        self.controls
            .set_playback(MediaPlayback::Playing { progress: None })
            .ok();
    }

    pub fn is_paused(&mut self) -> bool {
        self.paused
    }

    pub fn seek(&mut self, secs: i64) -> Result<()> {
        let (_, time_pos, duration) = self.get_progress();
        let seek_pos: u64;
        if secs >= 0 {
            seek_pos = time_pos + secs.abs() as u64;
        } else if time_pos > secs.abs() as u64 {
            seek_pos = time_pos - secs.abs() as u64;
        } else {
            seek_pos = 0;
        }

        if seek_pos.cmp(&duration) == std::cmp::Ordering::Greater {
            bail! {"exceed max length"};
        }
        // self.player.set_position(seek_pos as f32);
        Ok(())
    }

    #[allow(clippy::cast_precision_loss)]
    pub fn get_progress(&mut self) -> (f64, u64, u64) {
        // let time_pos = match self.player.get_position() {
        //     Some(t) => t as u64,
        //     None => 0_u64,
        // };
        // // let duration = match self.player.get_media().unwrap().duration() {
        // // Some(d) => d as u64,
        // // None => 119_u64,
        // // };
        // let duration = 119;
        // let percent = time_pos as f64 / (duration as f64);
        // (percent, time_pos, duration)
        (0.7, 23, 129)
    }
}
