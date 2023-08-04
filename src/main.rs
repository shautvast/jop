use std::env::args;
use std::io::BufReader;
use std::io::prelude::*;
use std::process::{Command, Stdio};
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use std::thread::sleep;
use std::time::Duration;

use textplots::{Chart, ColorPlot, Shape};

const RED: rgb::RGB8 = rgb::RGB8::new(0xFF, 0x00, 0x00);
const GREEN: rgb::RGB8 = rgb::RGB8::new(0x00, 0xFF, 0x00);


fn main() {
    let filename = args().nth(1).expect("requires filename");

    let should_run = Arc::new(AtomicBool::new(true));
    let should_run_ctrlc_ref = should_run.clone();

    let term = console::Term::stdout();
    term.hide_cursor().unwrap();
    term.clear_screen().unwrap();

    ctrlc::set_handler(move || {
        should_run_ctrlc_ref
            .as_ref()
            .store(false, Ordering::Relaxed);
    }).unwrap();

    let mut cmd = Command::new("tail")
        .args(&["-f", "-n", "+1", &filename])
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .unwrap();
    let mut reader = BufReader::new(cmd.stdout.take().unwrap());
    let mut line = String::new();

    let mut points_before: Vec<(f32, f32)> = vec![];
    let mut points_after: Vec<(f32, f32)> = vec![];

    while should_run.as_ref().load(Ordering::Acquire) {
        reader.read_line(&mut line).unwrap();
        let parsed_line = jop::parse(&line);
        if let Some(l) = parsed_line {
            points_after.push((l.ts.clone(), l.used_after as f32));
            points_before.push((l.ts, l.used_before as f32));
            let max_ts = points_after.last().unwrap().0.clone();

            term.move_cursor_to(0, 0).unwrap();
            Chart::new_with_y_range(200, 100, 0., max_ts, 0.0, l.total_memory as f32)
                .linecolorplot(&Shape::Lines(&points_before), GREEN)
                .linecolorplot(&Shape::Lines(&points_after), RED)
                .display();
        }

        sleep(Duration::from_millis(100)); // pause for nice animation
        line.clear();
    }

    let term = console::Term::stdout();
    term.show_cursor().unwrap();
}