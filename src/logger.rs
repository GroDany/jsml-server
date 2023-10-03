use std::{
    fmt::{self, Display, Formatter},
    time::{Duration, Instant},
};

use actix_web::http::StatusCode;

pub trait LogEntry: Display + Send + Sync {
    fn update(&mut self, code: StatusCode);
}

pub struct RouteEntry {
    timer: Instant,
    elapsed: Duration,
    path: String,
    code: Option<StatusCode>,
}

impl RouteEntry {
    pub fn new(path: &str) -> Self {
        Self {
            timer: Instant::now(),
            elapsed: Duration::from_millis(0),
            path: path.to_string(),
            code: None,
        }
    }
}

impl Display for RouteEntry {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut result = "".to_string();
        if let Some(code) = self.code {
            result = format!("{result}{code}");
        }
        let num = self.elapsed.as_micros();
        let (n, r) = (num / 1000, num % 1000);
        result = format!("{result}\t- {}\t-\t{}.{:03} ms", &self.path, n, r);
        write!(f, "{result}")
    }
}

impl LogEntry for RouteEntry {
    fn update(&mut self, code: StatusCode) {
        self.code = Some(code);
        self.elapsed = self.timer.elapsed();
        println!("{self}");
    }
}
