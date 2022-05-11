use crate::logic::find_intersection::find_system_intersections;
use crate::logic::find_optimum::find_optimum;
use crate::models::equation::Equation;
use crate::ui::app::App;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    error::Error,
    io,
    time::{Duration, Instant},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    Frame, Terminal,
};

use crate::ui::chart::{get_chart, get_chart_data};
use crate::ui::list::get_list;
use crate::ui::result::get_result;

pub fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let tick_rate = Duration::from_millis(250);
    let mut app = App::new();
    // test data
    app.equation_system = vec![
        Equation(12.0, 4.0, 480.0),
        Equation(8.0, 8.0, 640.0),
        Equation(1.0, -1.0, 0.0),
        Equation(-1.0, 0.0, 0.0),
        Equation(0.0, -1.0, 0.0),
    ];
    app.score_fn = Equation(50.0, 10.0, 0.0);
    app.highlighted_points
        .push(find_optimum(&app.equation_system, &app.score_fn).0);

    let res = run_app(&mut terminal, app, tick_rate);

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

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
    tick_rate: Duration,
) -> io::Result<()> {
    let mut last_tick = Instant::now();
    loop {
        terminal.draw(|f| ui(f, &app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));
        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if let KeyCode::Char('q') = key.code {
                    return Ok(());
                }
            }
        }
        if last_tick.elapsed() >= tick_rate {
            app.on_tick();
            last_tick = Instant::now();
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
    let size = f.size();
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Ratio(4, 5), Constraint::Ratio(1, 5)].as_ref())
        .split(size);

    let max_x = 150.0;
    let max_y = 150.0;
    let chart_data = get_chart_data(&app.equation_system, max_x, max_y);
    let chart = get_chart(&chart_data, max_x, max_y);
    f.render_widget(chart, chunks[0]);

    let sidebar = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(60),
                Constraint::Percentage(20),
                Constraint::Percentage(20),
            ]
            .as_ref(),
        )
        .split(chunks[1]);

    let list = get_list(&app.equation_system);
    f.render_widget(list, sidebar[0]);

    let result = get_result(&app.equation_system, &app.score_fn);
    f.render_widget(result, sidebar[1]);
}
