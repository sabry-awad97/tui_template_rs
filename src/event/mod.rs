use std::{
    thread,
    time::{Duration, Instant},
};

use crossbeam_channel::{unbounded, Receiver, Sender};
use termimad::crossterm;

use crossterm::event::{self, Event as CrosstermEvent, KeyEvent, MouseEvent};

mod handler;

#[derive(Clone, Copy, Debug)]
pub enum Event {
    Tick,
    Key(KeyEvent),
    Mouse(MouseEvent),
    Resize(u16, u16),
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct EventHandler {
    sender: Sender<Event>,
    receiver: Receiver<Event>,
    handler: thread::JoinHandle<()>,
}

impl Default for EventHandler {
    fn default() -> Self {
        let tick_rate = Duration::from_millis(250);
        let (sender, receiver) = unbounded();
        let sender_clone = sender.clone();
        let handler = thread::spawn(move || {
            let sender = sender_clone;
            let mut last_tick = Instant::now();
            loop {
                let timeout = tick_rate
                    .checked_sub(last_tick.elapsed())
                    .unwrap_or(tick_rate);

                if event::poll(timeout).expect("no events available") {
                    match event::read().expect("unable to read event") {
                        CrosstermEvent::Key(e) => sender.send(Event::Key(e)),
                        CrosstermEvent::Mouse(e) => sender.send(Event::Mouse(e)),
                        CrosstermEvent::Resize(w, h) => sender.send(Event::Resize(w, h)),
                    }
                    .expect("failed to send terminal event")
                }

                if last_tick.elapsed() >= tick_rate {
                    sender.send(Event::Tick).expect("failed to send tick event");
                    last_tick = Instant::now();
                }
            }
        });
        Self {
            sender,
            receiver,
            handler,
        }
    }
}

impl EventHandler {
    pub fn new() -> Self {
        Self::default()
    }
}
