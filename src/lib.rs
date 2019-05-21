use std::{sync,cmp,ops,default};
use std::default::Default;

static DEFAULT_WIDTH: usize = 50;
static DEFAULT_TICK: char = '|';
static DEFAULT_BAR: char = '=';
static DEFAULT_INDICATOR: char = '#';
static SEGMENTS: [usize; 4] = [10, 5, 4, 2];


#[derive(Clone,Debug,Eq,PartialEq,Ord,PartialOrd,Hash)]
pub struct Style {
    width: usize,
    labels: bool,
    tick: char,
    bar: char,
    indicator: char,
}

impl Style {
    pub fn new() -> Self {
        Style::default()
    }

    pub fn width(mut self, width: usize) -> Self {
        self.width = width;
        self
    }

    pub fn labels(mut self, labels: bool) -> Self {
        self.labels = labels;
        self
    }

    pub fn tick(mut self, tick: char) -> Self {
        self.tick = tick;
        self
    }

    pub fn bar(mut self, bar: char) -> Self {
        self.bar = bar;
        self
    }

    pub fn indicator(mut self, indicator: char) -> Self {
        self.indicator = indicator;
        self
    }
}

impl default::Default for Style {
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

#[derive(Debug)]
pub struct ProgressBar {
    counter: sync::Arc<sync::Mutex<Counter>>,
    max_count: usize,
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
    pub fn new(max_count: usize) -> Self {
        ProgressBar::with_style(max_count, Style::default())
    }

    pub fn with_style(max_count: usize, style: Style) -> Self {
        let counter = sync::Arc::new(sync::Mutex::new(Counter::default()));
        draw_bar(&style);
        ProgressBar{counter, max_count, style}
    }

    pub fn style(&self) -> &Style {
        &self.style
    }

    pub fn inc(&self, i: usize) {
        let new_progress = {
            let mut c = self.counter.lock().unwrap();
            let new_count = cmp::min(c.count + i, self.max_count);
            let new_progress = new_count*self.style.width/self.max_count;
            let diff = new_progress - c.progress;
            *c = Counter{count: new_count, progress: new_progress, finished: false};
            diff
        };
        for _ in 0..new_progress {
            eprint!("{}", self.style.indicator);
        }
    }

    pub fn finish(&self) {
        self.inc(self.max_count);
        let mut c = self.counter.lock().unwrap();
        if c.finished == false {
            eprintln!("");
            c.finished = true;
        }
    }

    pub fn abort(&self) {
        let mut c = self.counter.lock().unwrap();
        *c = Counter{
            count: self.max_count,
            progress: self.style.width,
            finished: true
        };
    }

}

impl ops::Drop for ProgressBar {
    fn drop(&mut self) {
        self.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    //TODO: capture stderr and check
    // for the time being,

    #[test]
    fn construct() {
        eprintln!("");
        let max_count = 1000;
        {
            let bar = ProgressBar::new(max_count);
            assert_eq!(bar.style().width, DEFAULT_WIDTH);
        }

        {
            let width = 80;
            let mut style = Style::default();
            style.width = width;
            let bar = ProgressBar::with_style(max_count, style);
            assert_eq!(bar.style().width, width);
        }

    }

    #[test]
    fn inc() {
        eprintln!("");
        let max_count = 20;
        let ten_millis = std::time::Duration::from_millis(10);

        {
            let bar = ProgressBar::new(max_count);
            for _ in 0..max_count {
                std::thread::sleep(ten_millis);
                bar.inc(1);
            }
        }

        {
            let bar = ProgressBar::new(max_count);
            bar.inc(2*max_count);
        }

        let _bar = ProgressBar::new(max_count);
    }

    #[test]
    fn finish() {
        eprintln!("");
        let max_count = 200;
        let bar = ProgressBar::new(max_count);
        bar.finish();
    }

    #[test]
    fn abort() {
        eprintln!("");
        let max_count = 200;
        let bar = ProgressBar::new(max_count);
        bar.inc(50);
        bar.abort();
    }

    #[test]
    fn alt_styles() {
        eprintln!("");
        let max_count = 200;
        eprintln!("indicator █:");
        let style = Style::new().indicator('█');
        let bar = ProgressBar::with_style(max_count, style);
        bar.finish();

        eprintln!("\ntick ↓:");
        let style = Style::new().tick('↓');
        let bar = ProgressBar::with_style(max_count, style);
        bar.finish();

        eprintln!("\nbar -:");
        let style = Style::new().bar('-');
        let bar = ProgressBar::with_style(max_count, style);
        bar.finish();

        eprintln!("\nno labels:");
        let style = Style::new().labels(false);
        let bar = ProgressBar::with_style(max_count, style);
        bar.finish();

        eprintln!("\nall of the above:");
        let style = Style::new().indicator('█').labels(false).tick('↓').bar('-');
        let bar = ProgressBar::with_style(max_count, style);
        bar.finish();

        for w in 0..=20 {
            eprintln!("\nwidth {}:", w);
            let style = Style::new().width(w);
            let _bar = ProgressBar::with_style(max_count, style);
        }
        let w = 40;
        eprintln!("\nwidth {}:", w);
        let style = Style::new().width(w);
        let _bar = ProgressBar::with_style(max_count, style.clone());

    }
}
