extern crate tui;

use std::io;

use tui::Frame;
use tui::backend::CrosstermBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, Paragraph};

use crate::rail_system::{RailSystem, RailSystemTrait};

pub struct RailTerminalUI {
    pub(crate) rail_system: RailSystem
}

impl RailTerminalUI {

    pub fn new(rail_system: RailSystem) -> RailTerminalUI {
        RailTerminalUI {
            rail_system
        }
    }

    pub fn draw(&self, frame: &mut Frame<CrosstermBackend<io::Stdout>>) {
        let columns = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Percentage(20),
                    Constraint::Percentage(40),
                    Constraint::Percentage(40)
                ].as_ref()
            ).split(frame.size());

        let registers = self.get_registers();
        let block = Paragraph::new(registers)
            .block(Block::default().borders(Borders::ALL).title("Registers"));
        frame.render_widget(block, columns[0]);

        let program = self.get_program();
        let block = Paragraph::new(program)
            .block(Block::default().borders(Borders::ALL).title("Program"));
        frame.render_widget(block, columns[1]);

        let ram = self.get_ram();
        let block = Paragraph::new(ram)
            .block(Block::default().borders(Borders::ALL).title("RAM"));
        frame.render_widget(block, columns[2]);
    }

    fn get_registers(&self) -> Vec<Spans> {
        let mut spans = Vec::new();
        for i in 0..16 {
            let value = self.rail_system.get_register_value(i);
            let span_vec = match i {
                0..=7 =>
                    vec![
                        Span::raw(format!("R{}:  ", i)),
                        Span::styled(format!("{}", Self::hex_str(value)), Style::default().fg(Color::Blue)),
                    ],
                8 => vec![
                    Span::raw("BZ0: "),
                    Span::styled(format!("{}", Self::hex_str(value)), Style::default().fg(Color::Blue)),
                ],
                9 => vec![
                    Span::raw("LV0: "),
                    Span::styled(format!("{}", Self::hex_str(value)), Style::default().fg(Color::Blue)),
                ],
                10..=13 => vec![
                    Span::raw(format!("D{}: ", i)),
                    Span::styled(format!("{}", Self::hex_str(value)), Style::default().fg(Color::Blue)),
                ],
                14 => vec![
                    Span::raw("CNT: "),
                    Span::styled(format!("{}", Self::hex_str(value)), Style::default().fg(Color::Green)),
                ],
                15 => vec![
                    Span::raw("IO:  "),
                    Span::styled(format!("{}", Self::hex_str(value)), Style::default().fg(Color::Yellow)),
                ],
                _ => vec! [Span::raw("")],
            };
            spans.push(Spans::from(span_vec));
        }

        spans
    }

    fn get_program(&self) -> Vec<Spans> {
        let mut spans = Vec::new();
        let cnt = self.rail_system.get_cnt_register_value();
        for line in 0..32 {
            let slice_left = self.rail_system.get_program_slice(line * 4, (line * 4) + 3);
            let slice_right = self.rail_system.get_program_slice((line * 4) + 128, (line * 4) + 3 + 128);
            let mut is_exec_line = cnt / 4 == line;
            let mut line_vec = Vec::new();
            line_vec.push(Span::raw(" ".repeat(4)));   // left pad

            line_vec.push(Self::make_span(is_exec_line, slice_left));
            line_vec.push(Span::raw(" ".repeat(3)));   // middle pad

            is_exec_line = cnt / 4 == line + 32;
            line_vec.push(Self::make_span(is_exec_line, slice_right));

            spans.push(Spans::from(line_vec));
        }

        spans
    }

    fn get_ram(&self) -> Vec<Spans> {
        let mut spans = Vec::new();
        for line in 0..32 {
            let slice_left = self.rail_system.get_ram_slice(line * 4, (line * 4) + 3);
            let slice_right = self.rail_system.get_ram_slice((line * 4) + 128, (line * 4) + 3 + 128);
            let mut line_vec = Vec::new();
            line_vec.push(Span::raw(" ".repeat(4)));   // left pad
            line_vec.push(Self::make_span(false, slice_left));
            line_vec.push(Span::raw(" ".repeat(3)));   // middle pad
            line_vec.push(Self::make_span(false, slice_right));

            spans.push(Spans::from(line_vec));
        }

        spans
    }

    fn make_span(is_executing: bool, slice: &[u8]) -> Span {
            // this feels so wrong. There has to be a better way!
        let values = format!("{} {} {} {}", Self::hex_str(slice[0]), Self::hex_str(slice[1]),
                             Self::hex_str(slice[2]), Self::hex_str(slice[3]));
        if is_executing {
            Span::styled(format!(" > {}", values), Style::default().fg(Color::Green))
        }
        else {
            Span::raw(format!("   {}", values))
        }
    }

    fn hex_str(byte: u8) -> String {
        Self::get_byte_in_hex(byte, "")
    }

    fn get_byte_in_hex(byte: u8, prefix: &str) -> String {
        let hex_chars = "0123456789ABCDEF";

        let high_nibble = (byte as u16 >> 4) & 0x0F;
        let low_nibble = byte as u16 & 0x0F;

        format!("{}{}{}", prefix, hex_chars.chars().nth(high_nibble as usize).unwrap(), hex_chars.chars().nth(low_nibble as usize).unwrap())
    }

}
