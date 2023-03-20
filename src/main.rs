use std::thread::sleep;
// use std::thread::sleep;
use std::time::{Duration, SystemTime};
use crate::rail_system::{RailSystem, RailSystemTrait};
use clap::{App, Arg};

mod rail_system;

fn main() {
    let matches = App::new("Rail Simulator")
        // .arg(Arg::with_name("input")
        //     .help("Input file")
        //     .required(true)
        //     .index(1))
        // .arg(Arg::with_name("test")
        //     .long("test")
        //     .help("Run tests"))
        .arg(Arg::with_name("bench")
            .long("bench")
            .help("Run benchmark"))
        .arg(Arg::with_name("debug")
            .long("debug")
            .help("Run debug test"))
        .get_matches();

    // let file = matches.value_of("file").unwrap();
    // let test_flag = matches.is_present("test");
    let bench_flag = matches.is_present("bench");
    let debug_flag = matches.is_present("debug");

    if bench_flag {
        run_benchmark();
    }
    if debug_flag {
        run_debug();
    }
    // for _i in 0..64 {
    //     rs.step();
    //     // console.draw()
    //     // Thread.sleep(150);
    //     sleep(Duration::from_millis(50));
    // }

    // debug();
    // run_benchmark();
}

fn run_benchmark() {
    let steps = 100000000;
    let mut system = RailSystem::new_with_program(&FIBONACCI_ASM);
    system.set_io_print(false);
    println!("Running benchmark with {} steps...", steps);
    let begin_time = SystemTime::now();
    for _i in 0..steps {
        system.step();
    }
    let total_duration = SystemTime::now()
        .duration_since(begin_time)
        .expect("Timing error").as_millis();

    println!("{} millis for {} steps.", total_duration, steps);
}

fn run_debug() {
    let mut system = RailSystem::new_with_program(&FIBONACCI_ASM);
    system.set_io_print(true);
    for _i in 0..64 {
        system.step();
        sleep(Duration::from_millis(150));
    }
}

const FIBONACCI_ASM: [u8; 36] = [
    0x40, 0x00, 0x01, 0x01,
    0x40, 0x0B, 0x01, 0x0B,
    0x40, 0x02, 0x00, 0x0A,
    0x00, 0x01, 0x02, 0x02,
    0x40, 0x0A, 0x00, 0x01,
    0x40, 0x02, 0x00, 0x0F,
    0x91, 0x02, 0x04, 0x00,
    0x00, 0x0B, 0x04, 0x04,
    0x26, 0x00, 0x00, 0x08,
];
