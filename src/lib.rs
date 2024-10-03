use rand::prelude::*;
use std::fmt::Display;

#[derive(Clone, Copy)]
pub struct Log {
    base: f64,
    cut: f64,
    pool: f64,
    result: f64,
}

impl Log {
    fn new(base: f64, cut: f64, pool: f64, result: f64) -> Self {
        //let payout = base + cut * pool;
        //println!("{} + {}", result - payout, payout);
        Self {
            base,
            cut,
            pool,
            result,
        }
    }
}

impl Display for Log {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "In: {:.2} - Out: {:.2} + {:.3}% of {:.0}\n \\ => +{}\n",
            self.base, self.base, self.cut, self.pool, self.result
        )
    }
}

#[derive(Clone)]
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
        let log = Log::new(self.pot, cut, money, self.money);
        self.history.push(log);
        // println!("Rebet: {}\n{}", self.rebet, log); // Logging
    }
    pub fn log_loss(&mut self) {
        self.history.push(Log::new(-self.pot, 0.0, 0.0, self.money));
    }
    pub fn mutate(&mut self, rng: &mut ThreadRng) {
        self.rebet += rng.gen_range(-0.1..0.1) * 0.1;
        self.rebet = self.rebet.clamp(0.0, 1.0);
        self.believer = rng.gen();
    }
    pub fn display_logs(&self) {
        self.history.iter().for_each(|l| println!("{}", l));
    }
}

#[derive(Clone, Copy)]
pub struct Pool {
    pub left: f64,
    pub right: f64,
}

impl Pool {
    pub fn new(left: f64, right: f64) -> Self {
        Self { left, right }
    }
    pub fn total(&self) -> f64 {
        self.left + self.right
    }
}

pub struct Manager {
    pub players: Vec<Player>,
    pub pool: Pool,
}

impl Manager {
    pub fn new(size: usize, rng: &mut ThreadRng) -> Self {
        Self {
            players: (0..size).map(|_| Player::new(rng)).collect(),
            pool: Pool::new(1600000.0, 12000000.0),
        }
    }
    pub fn step(&mut self, rng: &mut ThreadRng) {
        let ghost: u8 = rng.gen_range(0..24);
        let believer_win = ghost == 0 || ghost == 1;

        //println!("Believer Win: {}", believer_win);

        for player in self.players.iter_mut() {
            player.set_pot();
            if player.believer && believer_win {
                player.cash_in(player.pot / self.pool.left, self.pool.right);
            } else if !believer_win && !player.believer {
                player.cash_in(player.pot / self.pool.right, self.pool.left);
            } else {
                player.log_loss();
            }
        }
    }
    pub fn reset_player_money(&mut self) {
        self.players
            .iter_mut()
            .for_each(|player| player.money = 50.0);
    }
    pub fn get_best(&mut self) -> Player {
        self.sort_players();
        self.players[0].clone()
    }
    pub fn sort_players(&mut self) {
        self.players
            .sort_by(|a, b| a.money.partial_cmp(&b.money).unwrap());
        self.players.reverse();
    }
    pub fn improve_step(&mut self, rng: &mut ThreadRng) {
        self.reset_player_money();
        (0..100).for_each(|_| self.step(rng));
        self.sort_players();
        let mut players_copy = self.players.clone();

        self.players = (0..self.players.len())
            .map(|_| players_copy[0].clone())
            .collect();

        self.players
            .iter_mut()
            .for_each(|player| player.mutate(rng));

        self.reset_player_money();
        (0..100).for_each(|_| self.step(rng));
        players_copy.sort_by(|a, b| a.money.partial_cmp(&b.money).unwrap());
        players_copy.reverse();

        println!("{} -> {}", players_copy[0].money, self.players[0].money);
        if self.players[0].money < players_copy[0].money {
            self.players = players_copy;
        }
        self.reset_player_money();
    }
}
