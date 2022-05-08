use crate::models::equation::Equation;

pub struct App {
    pub signal1: SinSignal,
    pub equation_system: Vec<Equation>,
    pub signal2: SinSignal,
    pub window: [f64; 2],
    pub score_fn: Equation,
}

impl App {
    pub fn new() -> App {
        let signal1 = SinSignal::new(0.2, 3.0, 18.0);
        let signal2 = SinSignal::new(0.1, 2.0, 10.0);
        App {
            signal1,
            signal2,
            equation_system: vec![],
            window: [0.0, 20.0],
            score_fn: Equation(0.0, 0.0, 0.0),
        }
    }

    pub fn on_tick(&mut self) {
        /* for _ in 0..5 {
            self.data.remove(0);
        }
        self.data1.extend(self.signal1.by_ref().take(5));
        for _ in 0..10 {
            self.data2.remove(0);
        }
        self.data2.extend(self.signal2.by_ref().take(10));
        self.window[0] += 1.0;
        self.window[1] += 1.0; */
    }
}

#[derive(Clone)]
pub struct SinSignal {
    x: f64,
    interval: f64,
    period: f64,
    scale: f64,
}

impl SinSignal {
    pub fn new(interval: f64, period: f64, scale: f64) -> SinSignal {
        SinSignal {
            x: 0.0,
            interval,
            period,
            scale,
        }
    }
}

impl Iterator for SinSignal {
    type Item = (f64, f64);
    fn next(&mut self) -> Option<Self::Item> {
        let point = (self.x, (self.x * 1.0 / self.period).sin() * self.scale);
        self.x += self.interval;
        Some(point)
    }
}
