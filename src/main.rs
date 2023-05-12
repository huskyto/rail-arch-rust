use std::{fs, io};
use std::thread::sleep;
use std::time::{Duration, SystemTime};

use clap::{App, Arg};
use tui::backend::CrosstermBackend;
use tui::Terminal;

use crate::rail_assembler::{RailAssembler, RailAssemblerTrait};
use crate::rail_system::{RailSystem, RailSystemTrait};
use crate::ui::RailTerminalUI;

mod rail_system;
mod rail_assembler;
mod ui;

fn main() {
    let matches = App::new("Rail Simulator")
        .about("A simulator and assembler for the Rail Architecture written in Rust")
        .arg(Arg::with_name("input")
            .long("input")
            .short('i')
            .takes_value(true)
            .help("Input file to run or assemble."))
        .arg(Arg::with_name("output")
            .long("output")
            .short('o')
            .takes_value(true)
            .help("Output file for assembled binary."))
        .arg(Arg::with_name("bench")
            .long("bench")
            .short('b')
            .help("Run included benchmark."))
        .arg(Arg::with_name("debug")
            .long("debug")
            .short('d')
            .help("Run debug test."))
        .arg(Arg::with_name("assemble")
            .long("assemble")
            .short('a')
            .help("Assemble a source AMS file."))
        .arg(Arg::with_name("run")
            .long("run")
            .short('r')
            .help("Runs an assembled binary file."))

        .arg(Arg::with_name("ui")
            .long("with-ui")
            .short('u')
            .help("If enabled when running a file, show the terminal UI."))
        .arg(Arg::with_name("steps")
            .long("steps")
            .short('s')
            .takes_value(true)
            .default_value("256")
            .help("When running a file, sets the number of steps to run."))
        .arg(Arg::with_name("delay")
            .long("delay")
            .short('w')
            .takes_value(true)
            .default_value("250")
            .help("When running a file, sets the delay between steps in ms."))
        .arg(Arg::with_name("print-hex")
            .long("print-hex")
            .short('p')
            .help("Takes a bin file and prints a hex representation"))

        .get_matches();

    let bench_flag = matches.is_present("bench");
    let debug_flag = matches.is_present("debug");
    let assemble_flag = matches.is_present("assemble");
    let run_flag = matches.is_present("run");
    let gui_flag = matches.is_present("ui");
    let print_hex = matches.is_present("print-hex");

    if matches.is_present("help") {
        return
    }

    if bench_flag {
        run_benchmark();
    }
    else if debug_flag {
        run_debug();
    }
    else if assemble_flag {
        let input_path = matches.value_of("input").expect("Need an input file to assemble!");
        let input_text = fs::read_to_string(input_path).expect("Input file does not exist or is unreadable!");
        let output_path = matches.value_of("output").expect("Need an output file for assembled binary!");

        let rail_assembler = RailAssembler::new();
        let assembled = rail_assembler.assemble(&input_text);

        fs::write(output_path, assembled).expect("Failed to write to output file!");
    }
    else if run_flag {
        let input_path = matches.value_of("input").expect("Need an input file to assemble!");
        let input_bin = fs::read(input_path).expect("Input file does not exist or is unreadable!");

        let steps = matches.value_of("steps")
            .expect("Missing value for steps")
            .parse::<u32>().expect("Value for steps must be an integer");
        let delay = matches.value_of("delay")
            .expect("Missing value for delay")
            .parse::<u64>().expect("Value for delay must be an integer");

        let mut rail_system = RailSystem::new();
        rail_system.load_program(&input_bin[..]);
        if gui_flag {
            rail_system.set_io_print(false);
            let backend = CrosstermBackend::new(io::stdout());
            let mut terminal = Terminal::new(backend).unwrap();
            let mut ui = RailTerminalUI::new(rail_system);

            terminal.clear().expect("Error clearing terminal.");
            sleep(Duration::from_millis(100));

            for _i in 0..steps {
                ui.rail_system.step();
                terminal.draw(|f| {
                    ui.draw(f);
                }).expect("Error drawing to terminal");
                sleep(Duration::from_millis(delay));
            }

            terminal.set_cursor(0, 36).expect("Failed to set cursor when finished");
        }
        else {  // no ui
            rail_system.set_io_print(true);
            for _i in 0..steps {
                rail_system.step();
                sleep(Duration::from_millis(delay));
            }
        }
    }
    else if print_hex {
        let input_path = matches.value_of("input").expect("Need an input file to print!");
        let input_bin = fs::read(input_path).expect("Input file does not exist or is unreadable!");
        print_byte_array_in_hex(&input_bin, "");
    }
    else {
        panic!("No options selected. Add '--help' to call for all options.");
    }

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

fn print_byte_array_in_hex(byte_array: &[u8], prefix: &str) {
    let hex_chars = "0123456789ABCDEF";
    let mut result = String::with_capacity(byte_array.len() * 2);

    for (count, byte) in byte_array.iter().enumerate() {
        let high_nibble = (byte >> 4) & 0x0F;
        let low_nibble = byte & 0x0F;
        result.push_str(&format!("{}{}{} ", prefix, hex_chars.as_bytes()[high_nibble as usize] as char, hex_chars.as_bytes()[low_nibble as usize] as char));
        if count % 4 == 3 {
            result.push('\n');
        }
    }

    println!("{}", result);
}

#[allow(dead_code)]
fn run_benchmark_rng() {
    let steps = 100000000;
    let assembler = RailAssembler::new();
    let bin = assembler.assemble(r#"
                        RAN_SS+IM1 29 0 0
                        RAN_NEXT 0 0 R1
                        JMP 0 0 4"#);
    let mut system = RailSystem::new_with_program(&bin);
    system.set_io_print(false);
    println!("Running RNG benchmark with {} steps...", steps);
    let begin_time = SystemTime::now();
    for _i in 0..steps {
        system.step();
    }
    println!("Result: {}", system.get_register_value(1));
    let total_duration = SystemTime::now()
        .duration_since(begin_time)
        .expect("Timing error").as_millis();

    println!("{} millis for {} steps.", total_duration, steps);
}
