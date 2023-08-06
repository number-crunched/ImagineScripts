
use std::io::Write;

const STAGE_1: f32 = 60.0 / 460.0;
const STAGE_2: f32 = 60.0 / 620.68;
const STAGE_3: f32 = 60.0 / 704.34;

#[derive(Debug, PartialEq)]
enum FiringState {
    Stage1,
    Stage2,
    Stage3,
}

struct ThunderlordWrapper {
    shots: u16,
    mag: u16,
    lightning_strikes: u16,
    state: FiringState,
    time_seconds: f32,
    last_strike: f32,
}
impl ThunderlordWrapper {
    fn new() -> Self {
        Self {
            shots: 0,
            mag: 62,
            lightning_strikes: 0,
            state: FiringState::Stage1,
            time_seconds: 0.0,
            last_strike: 0.0, // we should use option here because we technically havent
                              // had a lightning strike at 0.0 but :shrug:
        }
    }

    fn empty(&self) -> bool {
        self.mag <= 0
    }

    fn strike_time(&mut self) {
        if self.last_strike + 1.0 <= self.time_seconds {
            self.lightning_strikes += 1;
            self.last_strike = self.time_seconds;
            self.mag += 7;
        }
    }

    fn shoot(&mut self) {
        self.mag -= 1;
        self.shots += 1;
        self.strike_time();
        self.time_seconds += match self.state {
            FiringState::Stage1 => STAGE_1,
            FiringState::Stage2 => STAGE_2,
            FiringState::Stage3 => STAGE_3,
        };
        self.update();
    }

    fn update(&mut self) {
        if self.time_seconds >= 3.22 && self.state != FiringState::Stage3 {
            self.state = FiringState::Stage3
        } else if self.time_seconds >= 0.96 && self.state != FiringState::Stage2 {
            self.state = FiringState::Stage2
        }
    }
}

fn thunderlord() -> ThunderlordWrapper {
    let mut thunderlord = ThunderlordWrapper::new();

    loop {
        if thunderlord.empty() {
            break thunderlord;
        }

        thunderlord.shoot();
    }
}

fn main() -> std::io::Result<()> {
    let info = thunderlord();
    std::io::stdout().write_all(
        format!(
            "\nshots fired: {}\nlightning strikes: {}\ntime: {}\n",
            info.shots, info.lightning_strikes, info.time_seconds,
        )
        .as_bytes(),
    )?;
    Ok(())
}
