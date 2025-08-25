use std::{thread::sleep, time::Duration};

use progress_bar::ProgressBar;

fn main() {
    let p = ProgressBar::new(500);
    for _ in p {
        sleep(Duration::from_secs_f64(0.01))
    }
}
