use ghost_gambling::*;
use rand::prelude::*;

fn main() {
    let mut rng = thread_rng();
    let mut manager = Manager::new(20, &mut rng);
    (0..1000).for_each(|_| manager.step(&mut rng, false));
    println!("{}", manager.get_best().money);
    (0..2000).for_each(|_| manager.improve_step(&mut rng));
    (0..1000).for_each(|_| manager.step(&mut rng, true));
    let best_player = manager.get_best();
    best_player.display_logs();
    println!(
        "{}\nRebet: {}\nBeliever: {}",
        best_player.money, best_player.rebet, best_player.believer
    );
}
