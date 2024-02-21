pub mod monte_carlo_integration;

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    thread,
};

use log::info;
use parking_lot::RwLock;

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use monte_carlo_integration::MonteCarloIntegration as MCI;
pub use monte_carlo_integration::*;
pub mod rng;
use anyhow::{anyhow, Result};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout, Margin},
    style::{Color, Style},
    symbols,
    text::{Line, Span},
    widgets::{
        Block, Borders, LineGauge, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState,
        Sparkline,
    },
    Frame, Terminal,
};
use rayon::prelude::*;
pub use rng::*;
use rng_trait::RNG;
use tracing_subscriber::{layer::SubscriberExt as _, util::SubscriberInitExt as _};

pub mod rng_trait;

/// App holds the state of the application
#[derive(Clone)]
struct App {
    messages: Arc<RwLock<Vec<(String, (String, Vec<f64>))>>>,
    message_scroll: Arc<RwLock<usize>>,
    progress_gage: Arc<RwLock<Vec<(String, RwLock<usize>)>>>,
}

impl App {
    /// Creates a new instance of the application
    fn new() -> App {
        App {
            messages: Arc::new(RwLock::new(Vec::new())),
            message_scroll: Arc::new(RwLock::new(0)),
            progress_gage: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Adds a message to the message list
    pub fn add_message(&mut self, message: String, data: (String, Vec<f64>)) -> Result<()> {
        info!("add message: {}", &message);
        {
            let mut msg = self.messages.write();
            msg.push((message, data.clone()));
        }
        Ok(())
    }

    #[allow(dead_code)]
    pub fn set_scroll(&mut self, height: usize) {
        info!("set scroll: {}", height);
        let mut scroll = self.message_scroll.write();
        *scroll = height;
    }

    pub fn get_scroll(&self) -> usize {
        *self.message_scroll.read()
    }
}

fn main() -> Result<()> {
    // init log on file
    // let stdout_log = tracing_subscriber::fmt::layer().pretty();

    // // A layer that logs events to a file.
    let file = std::fs::File::create("debug.log");
    let file = match file {
        Ok(file) => file,
        Err(error) => panic!("Error: {:?}", error),
    };
    let debug_log = tracing_subscriber::fmt::layer()
        .with_writer(Arc::new(file))
        .with_ansi(false);
    tracing_subscriber::registry().with(debug_log).init();

    terminal_event_loop()?;

    Ok(())
}

fn run_calc(app: App) {
    let mut mci: HashMap<String, Mutex<Vec<MCI>>> = HashMap::new();
    for _ in 0..100 {
        push_all(&mut mci, |(rng, name), mci| {
            let item = MCI::template_new_with_box(rng, name.clone());
            match mci.get(&name) {
                Some(s) => {
                    s.lock().expect("failed!!!!!").push(item);
                }
                None => {
                    let mut new_vec = Vec::new();
                    new_vec.push(item);
                    mci.insert(name, Mutex::new(new_vec));
                }
            }
        });
    }
    let mut mci = mci
        .into_iter()
        .map(|(k, v)| (k, v.into_inner().expect("failed!!!!!")))
        .collect::<HashMap<String, Vec<MCI>>>();
    let gate_impl = mci
        .keys()
        .map(|k| (k.clone(), RwLock::new(0)))
        .collect::<Vec<_>>();
    {
        let mut gate = app.progress_gage.write();
        *gate = gate_impl;
    }
    let start_time = std::time::Instant::now();

    let err = mci
        .par_iter_mut()
        .map(|(name, mc)| {
            let err = mc
                .par_iter_mut()
                .map(|mc| {
                    let err = mc.err(1000000);
                    let gate = app.progress_gage.read();
                    let index = (*gate)
                        .iter()
                        .position(|x| x.0 == name.clone())
                        .ok_or(anyhow!("not found"))
                        .expect("failed!!!!!");
                    {
                        *gate[index].1.write() += 1;
                    }
                    err
                })
                .collect::<Vec<_>>();
            let max = err
                .iter()
                .max_by(|a, b| a.partial_cmp(b).expect("failed!!!!!"))
                .expect("failed!!!!!")
                .clone();
            let min = err
                .iter()
                .min_by(|a, b| a.partial_cmp(b).expect("failed!!!!!"))
                .expect("failed!!!!!")
                .clone();
            let sum = err.iter().sum::<f64>();
            let len = err.len() as f64;
            let avg = sum / len;
            let tmp = (
                format!(
                    "{:<20} max: {:<10}, min: {:<10}, avg: {:<10}, time: {}",
                    name,
                    max.to_string().split_at(10).0,
                    min.to_string().split_at(10).0,
                    avg.to_string().split_at(10).0,
                    std::time::Instant::now()
                        .duration_since(start_time)
                        .as_secs_f64()
                ),
                (name.clone(), err.clone()),
            );
            app.clone()
                .add_message(tmp.0, tmp.1)
                .expect("message write failed");
            (name.clone(), (max, min, avg, err))
        })
        .collect::<Vec<_>>();

    serde_json::to_writer(
        std::fs::File::create("err.json").expect("failed!!!!!"),
        &err,
    )
    .expect("json write failed");
}

pub fn terminal_event_loop() -> Result<()> {
    let mut stdout = std::io::stdout();
    enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture).expect("failed!!!!!");
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).expect("Failed to initialize terminal");

    let app = App::new();
    let res = run_app(app, &mut terminal);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(app: App, terminal: &mut Terminal<B>) -> Result<()> {
    let app_clone = app.clone();
    thread::spawn(move || run_calc(app_clone));

    let mut press_down = false;
    let mut press_up = false;
    loop {
        terminal.draw(|f| ui(f, &app))?;
        if crossterm::event::poll(std::time::Duration::from_millis(1000))? {
            match crossterm::event::read()? {
                crossterm::event::Event::Key(key) => match key.code {
                    crossterm::event::KeyCode::Char('q') => break,
                    crossterm::event::KeyCode::Esc => break,
                    // Ctrl+C
                    crossterm::event::KeyCode::Char('c')
                        if key
                            .modifiers
                            .contains(crossterm::event::KeyModifiers::CONTROL) =>
                    {
                        break
                    }
                    // Ctrl+D
                    crossterm::event::KeyCode::Char('d')
                        if key
                            .modifiers
                            .contains(crossterm::event::KeyModifiers::CONTROL) =>
                    {
                        break
                    }
                    // Up
                    crossterm::event::KeyCode::Up => {
                        press_down = !press_down;
                        if press_down {
                            let mut scroll = app.message_scroll.write();
                            if *scroll > 0 {
                                *scroll -= 1;
                            }
                        }
                    }
                    // Down
                    crossterm::event::KeyCode::Down => {
                        let max = app.messages.read().len();
                        press_up = !press_up;
                        if press_up {
                            let mut scroll = app.message_scroll.write();
                            if *scroll < max {
                                *scroll += 1;
                            }
                        }
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }
    Ok(())
}

fn ui(f: &mut Frame, app: &App) {
    let windows = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(f.size());

    let binding = app.progress_gage.read();
    let progress_gage = (*binding)
        .iter()
        .filter(|p| *p.1.read() < 100)
        .collect::<Vec<_>>();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(0)
        .constraints(
            [
                Constraint::Percentage(100),
                Constraint::Min(progress_gage.len() as u16 + 1),
            ]
            .as_ref(),
        )
        .split(windows[1]);

    let app_messages = &app.messages;

    {
        let messages = (*app_messages.read())
            .iter()
            .flat_map(|x| x.0.split('\n').collect::<Vec<&str>>())
            .enumerate()
            .map(|(i, m)| {
                let content = Line::from(Span::raw(format!("{:>2}: {}", i, m)));
                content
            })
            .collect::<Vec<_>>();
        let vertical_scroll = app.get_scroll();
        let message_paragraph = Paragraph::new(messages.clone())
            .scroll((vertical_scroll as u16, 0))
            .block(Block::new().borders(Borders::ALL));
        let scroll_bar = Scrollbar::default()
            .orientation(ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("↑"))
            .end_symbol(Some("↓"));
        let mut scrollbar_state =
            ScrollbarState::new(messages.iter().len()).position(vertical_scroll);

        f.render_widget(message_paragraph, chunks[0]);
        f.render_stateful_widget(
            scroll_bar,
            chunks[0].inner(&Margin {
                vertical: 1,
                horizontal: 0,
            }), // using a inner vertical margin of 1 unit makes the scrollbar inside the block
            &mut scrollbar_state,
        );
    }
    {
        let constrains = progress_gage
            .iter()
            .map(|_| Constraint::Min(1))
            .collect::<Vec<_>>();
        let progress = Layout::default()
            .direction(Direction::Vertical)
            .margin(0)
            .constraints(constrains)
            .split(chunks[1]);
        for (i, (name, lock)) in progress_gage.iter().enumerate() {
            let gage = lock.read();
            let gage = *gage as u16;
            let layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Min(20), Constraint::Percentage(100)])
                .split(progress[i]);
            let mut block = Block::default()
                .border_set(symbols::border::PLAIN)
                .borders(Borders::RIGHT);
            if i == progress_gage.len() - 1 {
                block = block.borders(Borders::RIGHT | Borders::BOTTOM);
            }
            let gage = LineGauge::default()
                .block(block)
                .gauge_style(Style::default().fg(Color::Green))
                .ratio(gage as f64 / 100.0);
            f.render_widget(gage, layout[1]);
            let mut block = Block::default()
                .border_set(symbols::border::PLAIN)
                .borders(Borders::LEFT);
            if i == progress_gage.len() - 1 {
                block = block.borders(Borders::LEFT | Borders::BOTTOM);
            }
            let name = Paragraph::new(name.clone()).block(block);
            f.render_widget(name, layout[0]);
        }
    }
    // view graph
    {
        let app_messages = app_messages.read();
        if app_messages.len() == 0 {
            return;
        }
        let index = if app.get_scroll() >= app_messages.len() {
            app_messages.len() - 1
        } else {
            app.get_scroll()
        };
        let data = (*app_messages)[index].1.clone();
        let title = data.0;
        let ave = data.1.iter().sum::<f64>() / data.1.len() as f64;
        // u64に変える
        let data = data
            .1
            .iter()
            .map(|x| ((0.1 + x) * (1e+10)).round() as u64)
            .collect::<Vec<_>>();

        let sparkline = Sparkline::default()
            .block(Block::default().title(title).borders(Borders::ALL))
            .data(data.as_slice())
            .style(Style::default().fg(if ave > 1.0 { Color::Red } else { Color::Green }));

        f.render_widget(sparkline, windows[0]);
    }
}

pub fn push_all<T, U>(mci: &mut T, f: U)
where
    U: Fn((Box<dyn RNG + 'static + Send + Sync>, String), &mut T),
{
    f(wrap_with_type_name(Acorn::new()), mci);
    f(wrap_with_type_name(AesRng::new()), mci);
    f(wrap_with_type_name(Arc4::new()), mci);
    f(wrap_with_type_name(ChaCha20::new()), mci);
    f(wrap_with_type_name(Fortuna::new()), mci);
    // f(wrap_with_type_name(GjRng::new()), mci);
    f(wrap_with_type_name(Hc128Rng::new()), mci);
    f(wrap_with_type_name(IsaacRng::new()), mci);
    f(wrap_with_type_name(JitterRng::new()), mci);
    f(wrap_with_type_name(Jsf64Rng::new()), mci);
    f(wrap_with_type_name(PcgXsl64LcgRng::new()), mci);
    f(wrap_with_type_name(Lehmer::new()), mci);
    f(wrap_with_type_name(MT19937::new()), mci);
    f(wrap_with_type_name(MswsRng::new()), mci);
    f(wrap_with_type_name(MultiplyWithCarry::new()), mci);
    f(wrap_with_type_name(Pcg64::new()), mci);
    f(wrap_with_type_name(Philox::new()), mci);
    f(wrap_with_type_name(RandenRng::new()), mci);
    f(wrap_with_type_name(Ranluxpp::new()), mci);
    f(wrap_with_type_name(Romu::new()), mci);
    f(wrap_with_type_name(R30::new()), mci);
    f(wrap_with_type_name(Sapparot64Rng::new()), mci);
    f(wrap_with_type_name(Sfc64Rng::new()), mci);
    f(wrap_with_type_name(ShiShuA::new()), mci);
    f(wrap_with_type_name(SplitMix64::new()), mci);
    // f(wrap_with_type_name(Velox3bRng::new()), mci);
    f(wrap_with_type_name(WyRand::new()), mci);
    f(wrap_with_type_name(Xorshift128::new()), mci);
    f(wrap_with_type_name(Xoshiro512StarStar::new()), mci);
    f(wrap_with_type_name(Yarrow::new()), mci);
}

pub fn wrap_with_type_name<T: RNG + 'static + Send + Sync>(
    sl: T,
) -> (Box<dyn RNG + 'static + Send + Sync>, String) {
    (
        Box::new(sl),
        std::any::type_name::<T>()
            .split("::")
            .last()
            .expect("failed!!!!!")
            .into(),
    )
}
