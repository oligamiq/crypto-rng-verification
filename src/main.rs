pub mod monte_carlo_integration;

use std::{
    collections::HashMap,
    sync::{Arc, Mutex, RwLock},
    thread,
};

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
    text::{Line, Span},
    widgets::{
        Block, Borders, Paragraph, Scrollbar,
        ScrollbarOrientation, ScrollbarState,
    },
    Frame, Terminal,
};
use rayon::prelude::*;
pub use rng::*;
use rng_trait::RNG;

pub mod rng_trait;

/// App holds the state of the application
#[derive(Clone)]
struct App {
    messages: Arc<RwLock<Vec<(String, (String, Vec<f64>))>>>,
    message_scroll: Arc<RwLock<usize>>,
}

impl App {
    /// Creates a new instance of the application
    fn new() -> App {
        App {
            messages: Arc::new(RwLock::new(Vec::new())),
            message_scroll: Arc::new(RwLock::new(0)),
        }
    }

    /// Adds a message to the message list
    pub fn add_message(&mut self, message: String, data: (String, Vec<f64>)) -> Result<()> {
        let mut msg = self.messages.write().map_err(|e| anyhow!("{}", e))?;
        msg.push((message, data));
        Ok(())
    }

    #[allow(dead_code)]
    pub fn set_scroll(&mut self, height: usize) {
        let mut scroll = self.message_scroll.write().unwrap();
        *scroll = height;
    }

    pub fn get_scroll(&self) -> usize {
        *self.message_scroll.read().unwrap()
    }
}

fn main() -> Result<()> {
    terminal_event_loop()?;

    Ok(())
}

fn run_calc(app: App) {
    let mut mci: HashMap<String, Mutex<Vec<MCI>>> = HashMap::new();
    for _ in 0..10 {
        push_all(&mut mci, |(rng, name), mci| {
            let item = MCI::template_new_with_box(rng, name.clone());
            match mci.get(&name) {
                Some(s) => {
                    s.lock().unwrap().push(item);
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
        .map(|(k, v)| (k, v.into_inner().unwrap()))
        .collect::<HashMap<String, Vec<MCI>>>();

    let start_time = std::time::Instant::now();

    let err = mci
        .par_iter_mut()
        .map(|(name, mc)| {
            let err = mc
                .par_iter_mut()
                .map(|mc| mc.err(1000000))
                .collect::<Vec<_>>();
            let max = err
                .iter()
                .max_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap()
                .clone();
            let min = err
                .iter()
                .min_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap()
                .clone();
            let sum = err.iter().sum::<f64>();
            let len = err.len() as f64;
            let avg = sum / len;
            app.clone()
                .add_message(format!(
                    "{:<18} max: {:<10}, min: {:<10}, avg: {:<10}, time: {}",
                    name,
                    max.to_string().split_at(10).0,
                    min.to_string().split_at(10).0,
                    avg.to_string().split_at(10).0,
                    std::time::Instant::now()
                        .duration_since(start_time)
                        .as_secs_f64()
                ),
                (name.clone(), err.clone()))
                .expect("message write failed");
            (name.clone(), (max, min, avg, err))
        })
        .collect::<Vec<_>>();

    serde_json::to_writer(
        std::fs::File::create("err.json").unwrap(),
        &err
    ).expect("json write failed");

}

pub fn terminal_event_loop() -> Result<()> {
    let mut stdout = std::io::stdout();
    enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture).unwrap();
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

    loop {
        terminal.draw(|f| ui(f, &app))?;
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
                    let mut scroll = app.message_scroll.write().unwrap();
                    if *scroll > 0 {
                        *scroll -= 1;
                    }
                }
                // Down
                crossterm::event::KeyCode::Down => {
                    let mut scroll = app.message_scroll.write().unwrap();
                    *scroll += 1;
                }
                _ => {}
            },
            _ => {}
        }
    }
    Ok(())
}

fn ui(f: &mut Frame, app: &App) {
    let windows = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(f.size());

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([Constraint::Length(90), Constraint::Percentage(10)].as_ref())
        .split(windows[0]);
    {
        let app_messages = app.messages.read().unwrap();
        let messages = (*app_messages)
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

        f.render_widget(message_paragraph, windows[1]);
        f.render_stateful_widget(
            scroll_bar,
            windows[1].inner(&Margin {
                vertical: 1,
                horizontal: 0,
            }), // using a inner vertical margin of 1 unit makes the scrollbar inside the block
            &mut scrollbar_state,
        );
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
            .unwrap()
            .into(),
    )
}
