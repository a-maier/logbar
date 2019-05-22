use std::{sync,cmp,default};
use std::default::Default;

static DEFAULT_WIDTH: usize = 50;
static DEFAULT_TICK: char = '|';
static DEFAULT_BAR: char = '=';
static DEFAULT_INDICATOR: char = '#';
static SEGMENTS: [usize; 4] = [10, 5, 4, 2];

/// Progress bar style
#[derive(Clone,Debug,Eq,PartialEq,Ord,PartialOrd,Hash)]
pub struct Style {
    width: usize,
    labels: bool,
    tick: char,
    bar: char,
    indicator: char,
}

impl Style {
    /// Default progress bar style
    ///
    /// # Example
    ///
    /// Create a progress bar, explicitly asking for the default style:
    /// ```rust
    /// let style = logbar::Style::new();
    /// let max_progress = 100;
    /// let bar = logbar::ProgressBar::with_style(max_progress, style);
    /// ```
    pub fn new() -> Self {
        Style::default()
    }

    /// Set the progress bar width in characters
    ///
    /// # Example
    ///
    /// Create a progress bar with a width of 80 characters:
    /// ```rust
    /// let style = logbar::Style::new().width(80);
    /// let max_progress = 100;
    /// let bar = logbar::ProgressBar::with_style(max_progress, style);
    /// ```
    pub fn width(mut self, width: usize) -> Self {
        self.width = width;
        self
    }

    /// Toggle progress bar labels of the form XX%
    ///
    /// # Example
    ///
    /// Create a progress bar without labels:
    /// ```rust
    /// let style = logbar::Style::new().labels(false);
    /// let max_progress = 100;
    /// let bar = logbar::ProgressBar::with_style(max_progress, style);
    /// ```
    pub fn labels(mut self, labels: bool) -> Self {
        self.labels = labels;
        self
    }

    /// Choose a "tick" character separating the progress bar segments
    ///
    /// # Example
    ///
    /// Create a progress bar with '↓' as tick character:
    /// ```rust
    /// let style = logbar::Style::new().tick('↓');
    /// let max_progress = 100;
    /// let bar = logbar::ProgressBar::with_style(max_progress, style);
    /// ```
    pub fn tick(mut self, tick: char) -> Self {
        self.tick = tick;
        self
    }

    /// Choose a character for the progress bar segments
    ///
    /// # Example
    ///
    /// Create a progress bar made out of '-' characters, separated
    /// by the default "tick" character.
    /// ```rust
    /// let style = logbar::Style::new().bar('-');
    /// let max_progress = 100;
    /// let bar = logbar::ProgressBar::with_style(max_progress, style);
    /// ```
    pub fn bar(mut self, bar: char) -> Self {
        self.bar = bar;
        self
    }

    /// Choose a progress indicator
    ///
    /// # Example
    ///
    /// Create a progress bar where the progress is indicated by the
    /// number of '█' characters.
    /// ```rust
    /// let style = logbar::Style::new().indicator('█');
    /// let max_progress = 100;
    /// let bar = logbar::ProgressBar::with_style(max_progress, style);
    /// ```
    pub fn indicator(mut self, indicator: char) -> Self {
        self.indicator = indicator;
        self
    }
}

impl default::Default for Style {
    /// Default progress bar style
    ///
    /// # Example
    ///
    /// Create a progress bar, explicitly asking for the default style:
    /// ```rust
    /// let style = logbar::Style::default();
    /// let max_progress = 100;
    /// let bar = logbar::ProgressBar::with_style(max_progress, style);
    /// ```
    fn default() -> Self {
        Style{
            width: DEFAULT_WIDTH,
            labels: true,
            tick: DEFAULT_TICK,
            bar: DEFAULT_BAR,
            indicator: DEFAULT_INDICATOR,
        }
    }
}

#[derive(Clone,Default,Debug,Eq,PartialEq,Ord,PartialOrd,Hash)]
struct Counter {
    count: usize,
    progress: usize,
    finished: bool,
}

/// A log-friendly progress bar
#[derive(Debug)]
pub struct ProgressBar {
    counter: sync::Arc<sync::Mutex<Counter>>,
    max_progress: usize,
    style: Style,
}

fn num_segments(width: usize) -> usize {
    for s in SEGMENTS.iter() {
        // the number of segments must divide the total width
        // and each segment must be large enough for labels
        if width % s == 0 && width/s  > 3 {
            return *s;
        }
    }
    1
}

fn draw_labels(width: usize, segments: usize) {
    debug_assert_eq!(width % segments, 0);
    eprint!("0% ");
    let seg_width = width/segments;
    for p in 1..=segments {
        for _ in 0..(seg_width-3) {
            eprint!(" ");
        }
        eprint!("{}%", p*100/segments)
    }
    eprintln!("")
}

fn draw_tickbar(style: &Style, segments: usize) {
    let width = style.width;
    debug_assert_eq!(width % segments, 0);
    eprint!("{}", style.tick);
    let seg_width = width/segments;
    for _ in 1..=segments {
        for _ in 0..(seg_width-1) {
            eprint!("{}", style.bar);
        }
        eprint!("{}", style.tick)
    }
    eprintln!("")
}

fn draw_bar(style: &Style) {
    let width = style.width;
    let segments = num_segments(width);
    if width > 3 && style.labels {
        draw_labels(width, segments)
    }
    if width > 1 {
        draw_tickbar(style, segments)
    }
    else if width == 1 {
        eprintln!("{}", style.tick)
    }
}

impl ProgressBar {
    /// Create a new progress bar with default style
    ///
    /// # Example
    ///
    /// ```rust
    /// let max_progress = 100;
    /// let bar = logbar::ProgressBar::new(max_progress);
    /// ```
    pub fn new(max_progress: usize) -> Self {
        ProgressBar::with_style(max_progress, Style::default())
    }

    /// Create a new progress bar with custom style
    ///
    /// # Example
    ///
    /// Create a progress bar with a width of 80 characters:
    /// ```rust
    /// let style = logbar::Style::new().width(80);
    /// let max_progress = 100;
    /// let bar = logbar::ProgressBar::with_style(max_progress, style);
    /// ```
    pub fn with_style(max_progress: usize, style: Style) -> Self {
        let counter = sync::Arc::new(sync::Mutex::new(Counter::default()));
        draw_bar(&style);
        ProgressBar{counter, max_progress, style}
    }

    /// Get the style of the current progress bar
    ///
    /// # Example
    ///
    /// ```rust
    /// let max_progress = 100;
    /// let bar = logbar::ProgressBar::new(max_progress);
    /// assert_eq!(bar.style(), &logbar::Style::default())
    /// ```
    pub fn style(&self) -> &Style {
        &self.style
    }

    /// Increment the progress
    ///
    /// This method increments the internal progress counter, up to the
    /// maximum defined during the construction of the progress bar. It
    /// then updates the progress display.
    ///
    /// # Example
    ///
    /// ```rust
    /// let max_progress = 50;
    /// let bar = logbar::ProgressBar::new(max_progress);
    /// bar.inc(10); // Increment progress to 10 out of 50.
    /// // The progress bar is at 20% now
    /// bar.inc(10); // Increment progress to 20 out of 50.
    /// // The progress bar is at 40% now
    /// bar.inc(100); // Increment progress to 50 out of 50.
    /// // The progress bar is at 100% now
    /// ```
    pub fn inc(&self, i: usize) {
        let new_progress = {
            let mut c = self.counter.lock().unwrap();
            let new_count = cmp::min(c.count + i, self.max_progress);
            let new_progress = new_count*self.style.width/self.max_progress;
            let diff = new_progress - c.progress;
            *c = Counter{count: new_count, progress: new_progress, finished: false};
            diff
        };
        for _ in 0..new_progress {
            eprint!("{}", self.style.indicator);
        }
    }

    /// Finish the progress bar
    ///
    /// This method sets the progress to 100% and moves to the next line
    /// after the progress bar
    ///
    /// # Example
    ///
    /// ```rust
    /// let max_progress = 50;
    /// let bar = logbar::ProgressBar::new(max_progress);
    /// bar.finish();
    /// // The progress bar is at 100% now
    /// ```
    pub fn finish(&self) {
        self.inc(self.max_progress);
        let mut c = self.counter.lock().unwrap();
        if c.finished == false {
            eprintln!("");
            c.finished = true;
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    //TODO: capture stderr and check
    // for the time being, run
    // cargo test -- --nocapture --test_threads=1
    // and check the output manually

    #[test]
    fn construct() {
        eprintln!("");
        let max_progress = 1000;
        {
            let bar = ProgressBar::new(max_progress);
            assert_eq!(bar.style().width, DEFAULT_WIDTH);
        }

        {
            let width = 80;
            let mut style = Style::default();
            style.width = width;
            let bar = ProgressBar::with_style(max_progress, style);
            assert_eq!(bar.style().width, width);
        }

    }

    #[test]
    fn inc() {
        eprintln!("");
        let max_progress = 20;
        let ten_millis = std::time::Duration::from_millis(10);

        let bar = ProgressBar::new(max_progress);
        for _ in 0..max_progress {
            std::thread::sleep(ten_millis);
            bar.inc(1);
        }
        eprintln!("");

        let bar = ProgressBar::new(max_progress);
        bar.inc(2*max_progress);
        eprintln!("");

        let _bar = ProgressBar::new(max_progress);
        eprintln!("");
    }

    #[test]
    fn finish() {
        eprintln!("");
        let max_progress = 200;
        let bar = ProgressBar::new(max_progress);
        bar.finish();
    }

    #[test]
    fn abort() {
        eprintln!("");
        let max_progress = 200;
        let bar = ProgressBar::new(max_progress);
        bar.inc(50);
    }

    #[test]
    fn alt_styles() {
        eprintln!("");
        let max_progress = 200;
        eprintln!("indicator █:");
        let style = Style::new().indicator('█');
        let bar = ProgressBar::with_style(max_progress, style);
        bar.finish();

        eprintln!("\ntick ↓:");
        let style = Style::new().tick('↓');
        let bar = ProgressBar::with_style(max_progress, style);
        bar.finish();

        eprintln!("\nbar -:");
        let style = Style::new().bar('-');
        let bar = ProgressBar::with_style(max_progress, style);
        bar.finish();

        eprintln!("\nno labels:");
        let style = Style::new().labels(false);
        let bar = ProgressBar::with_style(max_progress, style);
        bar.finish();

        eprintln!("\nall of the above:");
        let style = Style::new().indicator('█').labels(false).tick('↓').bar('-');
        let bar = ProgressBar::with_style(max_progress, style);
        bar.finish();

        for w in 0..=20 {
            eprintln!("\nwidth {}:", w);
            let style = Style::new().width(w);
            let bar = ProgressBar::with_style(max_progress, style);
            bar.finish();
        }
        let w = 40;
        eprintln!("\nwidth {}:", w);
        let style = Style::new().width(w);
        let bar = ProgressBar::with_style(max_progress, style.clone());
        bar.finish();
    }
}
