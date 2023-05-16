extern crate tui;

use std::io;

use tui::Frame;
use tui::backend::CrosstermBackend;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::{Color, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, Paragraph};

use crate::rail_system::{RailSystem, RailSystemTrait};

pub struct RailTerminalUI {
    pub(crate) rail_system: RailSystem
}

impl RailTerminalUI {

    const MEM_LEFT_PAD: usize = 0;
    const MEM_MIDDLE_PAD: usize = 0;
    const MEM_RIGHT_PAD: usize = 2;
    const MEM_WIDTH: u16 = (Self::MEM_LEFT_PAD + 6 + 22 + Self::MEM_MIDDLE_PAD + Self::MEM_RIGHT_PAD + 2) as u16;
    const S_MEM_WIDHT: u16 = (Self::MEM_LEFT_PAD + 2 + 11 + Self::MEM_RIGHT_PAD + 2) as u16;

    const REG_WIDTH: u16 = 11;

    pub fn new(rail_system: RailSystem) -> RailTerminalUI {
        RailTerminalUI {
            rail_system
        }
    }

    pub fn draw(&self, frame: &mut Frame<CrosstermBackend<io::Stdout>>) {
        let rows = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Length(6),
                    Constraint::Percentage(70)
                ].as_ref()
            ).split(frame.size());

        let top_cols = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Percentage(30),
                    Constraint::Percentage(40),
                    Constraint::Percentage(30)
                ].as_ref()
            ).split(rows[0]);

        self.draw_level(frame, top_cols[0]);
        self.draw_header(frame, top_cols[1]);
        self.draw_7seg_widgets(frame, top_cols[2]);
        self.draw_content(frame, rows[1]);
    }

    fn draw_header(&self, frame: &mut Frame<CrosstermBackend<io::Stdout>>, area: Rect) {
        let spans = vec![
            Spans::from(Span::styled("    ___       _ __  ___           __ ", Style::default().fg(Color::Yellow))),
            Spans::from(Span::styled("   / _ \\___ _(_) / / _ | ________/ / ", Style::default().fg(Color::Yellow))),
            Spans::from(Span::styled("  / , _/ _ `/ / / / __ |/ __/ __/ _ \\", Style::default().fg(Color::Yellow))),
            Spans::from(Span::styled(" /_/|_|\\_,_/_/_/ /_/ |_/_/  \\__/_//_/", Style::default().fg(Color::Yellow))),
            Spans::from(""),
        ];
        let block = Paragraph::new(spans)
            .alignment(Alignment::Center)
            .block(Block::default()
                .border_style(Style::default().fg(Color::Yellow))
                .borders(Borders::BOTTOM | Borders::TOP));

        frame.render_widget(block, area);
    }

    fn draw_7seg_widgets(&self, frame: &mut Frame<CrosstermBackend<io::Stdout>>, area: Rect) {
        let display_columns = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Percentage(25),
                    Constraint::Percentage(25),
                    Constraint::Percentage(25),
                    Constraint::Percentage(25)
                ].as_ref()
            ).split(area);

        for d_reg_num in 10..14 {
            self.draw_7_seg(frame, display_columns[d_reg_num - 10],
                            self.rail_system.get_register_value(d_reg_num as u8));
        }
    }

    fn draw_7_seg(&self, frame: &mut Frame<CrosstermBackend<io::Stdout>>, area: Rect, value: u8) {
        let segments = Self::get_segs_for_byte(value);

        let spans = vec![
            Spans::from(""),
            Spans::from(""),
            Spans::from(Span::styled(segments[0].clone(), Style::default().fg(Color::Red))),
            Spans::from(Span::styled(segments[1].clone(), Style::default().fg(Color::Red))),
            Spans::from(Span::styled(segments[2].clone(), Style::default().fg(Color::Red)))
        ];
        let block = Paragraph::new(spans)
            .alignment(Alignment::Center)
            .block(Block::default());

        frame.render_widget(block, area);
    }

    fn get_segs_for_byte(value: u8) -> Vec<String> {
        let high_nibble = (value >> 4) & 0x0F;
        let low_nibble = value & 0x0F;
        let high_segs = Self::get_segs_for_nibble(high_nibble);
        let low_segs = Self::get_segs_for_nibble(low_nibble);
        vec![
                format!("{} {}", high_segs[0], low_segs[0]),
                format!("{} {}", high_segs[1], low_segs[1]),
                format!("{} {}", high_segs[2], low_segs[2])
            ]
    }

    const SEGS: [&'static str; 3] = [
        " _     _  _     _  _  _  _  _  _     _     _  _    ",
        "| |  | _| _||_||_ |_   ||_||_||_||_ |   _||_ |_    ",
        "|_|  ||_  _|  | _||_|  ||_| _|| ||_||_ |_||_ |     "
    ];

    const CIRC_OFF: &'static str = "○";
    const CIRC_ON: &'static str = "●";
    const CIRC_SEPARATOR: &'static str = "  ";

    fn get_segs_for_nibble<'a>(nibble: u8) -> Vec<&'a str> {
        let offset = (std::cmp::min(16, nibble) * 3) as usize;
        vec![
                &Self::SEGS[0][offset..offset + 3],
                &Self::SEGS[1][offset..offset + 3],
                &Self::SEGS[2][offset..offset + 3]
            ]
    }

    fn draw_level(&self, frame: &mut Frame<CrosstermBackend<io::Stdout>>, area: Rect) {
        let mut spans = Vec::new();
        let reg_val = self.rail_system.get_register_value(9);
        for pos in 0..8 {
            if Self::is_bit_pos_on(reg_val, pos as usize) {
                spans.push(Span::styled(Self::CIRC_ON,
                                Style::default().fg(Color::LightGreen)));
                spans.push(Span::raw(Self::CIRC_SEPARATOR));
            }
            else {
                spans.push(Span::styled(Self::CIRC_OFF,
                                                    Style::default().fg(Color::DarkGray)));
                spans.push(Span::raw(Self::CIRC_SEPARATOR));
            }
        }

        let spans = vec![
            Spans::from(""),
            Spans::from(""),
            Spans::from(""),
            Spans::from(spans),
            Spans::from("")
        ];

        let block = Paragraph::new(spans)
            .alignment(Alignment::Center)
            .block(Block::default());

        frame.render_widget(block, area);
    }

    fn is_bit_pos_on(value: u8, pos: usize) -> bool {
        let mask = 1 << (7 - pos);
        value & mask > 0
    }

    fn draw_content(&self, frame: &mut Frame<CrosstermBackend<io::Stdout>>, area: Rect) {
            // TODO check if content will not fit and display somehow
        let main_columns = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Length(Self::REG_WIDTH),
                    Constraint::Min(10),
                ].as_ref()
            ).split(area);
        // min heigh for display : 34

        let mem_columns = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Length(Self::MEM_WIDTH),
                    Constraint::Length(Self::MEM_WIDTH),
                    Constraint::Length(Self::S_MEM_WIDHT),
                    // Constraint::Percentage(50),
                    Constraint::Percentage(50)
                ].as_ref()
            ).split(main_columns[1]);

        let registers = self.get_registers();
        let block = Paragraph::new(registers)
            .block(Block::default().borders(Borders::ALL).title("Registers"));
        frame.render_widget(block, main_columns[0]);

        let program = self.get_program();
        let block = Paragraph::new(program)
            .block(Block::default().borders(Borders::ALL).title("Program"));
        frame.render_widget(block, mem_columns[0]);

        let ram = self.get_ram();
        let block = Paragraph::new(ram)
            .block(Block::default().borders(Borders::ALL).title("RAM"));
        frame.render_widget(block, mem_columns[1]);

        let call_stack = self.get_call_stack();
        let block = Paragraph::new(call_stack)
            .block(Block::default().borders(Borders::ALL).title("Call Stack"));
        frame.render_widget(block, mem_columns[2]);

        let gen_stack = self.get_gen_stack();
        let block = Paragraph::new(gen_stack)
            .block(Block::default().borders(Borders::ALL).title("Gen Stack"));
        frame.render_widget(block, mem_columns[3]);
    }

    fn get_registers(&self) -> Vec<Spans> {
        let mut spans = Vec::new();
        spans.push(Spans::from(vec! [Span::raw("")]));

        for i in 0..16 {
            let value = self.rail_system.get_register_value(i);
            let span_vec = match i {
                0..=7 =>
                    vec![
                        Span::raw(format!(" R{}:  ", i)),
                        Span::styled(Self::hex_str(value), Style::default().fg(Color::Blue)),
                    ],
                8 => vec![
                    Span::raw(" BZ0: "),
                    Span::styled(Self::hex_str(value), Style::default().fg(Color::Blue)),
                ],
                9 => vec![
                    Span::raw(" LV0: "),
                    Span::styled(Self::hex_str(value), Style::default().fg(Color::Blue)),
                ],
                10..=13 => vec![
                    Span::raw(format!(" D{} : ", i - 10)),
                    Span::styled(Self::hex_str(value), Style::default().fg(Color::Blue)),
                ],
                14 => vec![
                    Span::raw(" CNT: "),
                    Span::styled(Self::hex_str(value), Style::default().fg(Color::Green)),
                ],
                15 => vec![
                    Span::raw(" IO:  "),
                    Span::styled(Self::hex_str(value), Style::default().fg(Color::LightYellow)),
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
            line_vec.push(Span::raw(" ".repeat(Self::MEM_LEFT_PAD)));   // left pad

            line_vec.push(Self::make_span(is_exec_line, slice_left));
            line_vec.push(Span::raw(" ".repeat(Self::MEM_MIDDLE_PAD)));   // middle pad

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
            let line_vec = vec![
                Span::raw(" ".repeat(Self::MEM_LEFT_PAD)),  // left pad
                Self::make_span(false, slice_left),
                Span::raw(" ".repeat(Self::MEM_MIDDLE_PAD)),   // middle pad
                Self::make_span(false, slice_right)];

            spans.push(Spans::from(line_vec));
        }

        spans
    }

    fn get_call_stack(&self) -> Vec<Spans> {
        let mut spans = Vec::new();
        let ptr = self.rail_system.get_call_stack_ptr();
        for line in 0..32 {
            let slice = self.rail_system.get_call_stack_slice(line * 4, (line * 4) + 3);

            let local_ptr = if ptr / 4 == line { ptr - (line * 4) }
            else { 5 };

            let mut line_vec = Vec::new();
            line_vec.push(Span::raw(" ".repeat(2)));
            line_vec.push(Span::raw(" ".repeat(Self::MEM_LEFT_PAD)));
            let mut val_spans = Self::make_stack_span(local_ptr, slice);
            line_vec.append(&mut val_spans);

            spans.push(Spans::from(line_vec));
        }

        spans
    }

    fn get_gen_stack(&self) -> Vec<Spans> {
        let mut spans = Vec::new();
        let ptr = self.rail_system.get_gen_stack_ptr();
        for line in 0..32 {
            let slice = self.rail_system.get_gen_stack_slice(line * 4, (line * 4) + 3);

            let local_ptr = if ptr / 4 == line { ptr - (line * 4) }
            else { 5 };

            let mut line_vec = Vec::new();
            line_vec.push(Span::raw(" ".repeat(2)));
            line_vec.push(Span::raw(" ".repeat(Self::MEM_LEFT_PAD)));
            let mut val_spans = Self::make_stack_span(local_ptr, slice);
            line_vec.append(&mut val_spans);

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

    fn make_stack_span(local_ptr: u8, slice: &[u8]) -> Vec<Span> {
        let mut res = Vec::new();
        for i in 0..4 {
            if i == local_ptr {
                res.push(Span::styled(format!("{} ", Self::hex_str(slice[i as usize])), Style::default().fg(Color::Green)));
            }
            else {
                res.push(Span::raw(format!("{} ", Self::hex_str(slice[i as usize]))));
            }
        }

        res
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
