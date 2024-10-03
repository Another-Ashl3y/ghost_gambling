use ghost_gambling::*;
use indicatif::ProgressBar;
use rand::prelude::*;

fn main() {
    let mut rng = thread_rng();
    let mut manager = Manager::new(200, &mut rng);
    (0..16).for_each(|i| manager.step(&mut rng, i, false));
    println!("{}", manager.get_best().money);
    let runtime = 200000;
    let bar = ProgressBar::new(runtime);
    (0..runtime).for_each(|_| {
        manager.improve_step(&mut rng);
        bar.inc(1);
    });
    (0..20).for_each(|i| manager.step(&mut rng, i, true));
    let best_player = manager.get_best();
    best_player.display_logs();
    println!(
        "{}\nRebet: {}\nBeliever: {}",
        best_player.money, best_player.rebet, best_player.believer
    );
}
