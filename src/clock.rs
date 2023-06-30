use std::thread;
use std::time::Duration;

use crate::channel::Channel;
use crate::game_signal::GameSignal;

struct Clock {
    channel: Channel<GameSignal>,
}

impl Clock {
    const PERIOD_MILLIS: u64 = 100;

    fn new(channel: Channel<GameSignal>) -> Self {
        Clock { channel }
    }

    fn start(&self) {
        let sender = self.channel.clone();
        thread::spawn(move || {
            Self::run(sender);
        });
    }

    fn run(channel: Channel<GameSignal>) {
        loop {
            thread::sleep(Duration::from_millis(Self::PERIOD_MILLIS));
            channel.put(GameSignal::ClockTick);
        }
    }
}

