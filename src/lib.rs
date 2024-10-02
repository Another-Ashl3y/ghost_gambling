use rand::prelude::*;

struct Log {
    base: f64,
    cut: f64,
    pool: f64,
}

impl Log {
    fn new(base: f64, cut: f64, pool: f64) -> Self {
        Self { base, cut, pool }
    }
}

pub struct Player {
    pub money: f64,
    pub rebet: f64,
    pot: f64,
    pub believer: bool,
    pub history: Vec<Log>,
}

impl Player {
    pub fn new(rng: &mut ThreadRng) -> Self {
        Self {
            money: 50.0,
            rebet: rng.gen(),
            pot: 0.0,
            believer: rng.gen(),
            history: Vec::new(),
        }
    }
    pub fn set_pot(&mut self) {
        self.pot = self.money * self.rebet;
        self.money -= self.pot;
    }
    pub fn cash_in(&mut self, cut: f64, money: f64) {
        self.money += self.pot + cut * money;
        self.history.push(
            println!(
            "Bet: {:.3}%\nIn: {:.0} - Out: {:.0} + {:.3}% of {:.0}\n \\ => +{}",
            
        
    }
    pub fn log_loss(&mut self) {
        self.history.push(Log::new(base, cut, pool));
    }
}
