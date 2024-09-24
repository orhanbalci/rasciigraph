use std::cmp::{max, min};

use crate::consts::colors::{RESET, WHITE};

// Define the Config struct
#[derive(Clone)]
pub struct Config {
    color: String,
}

const SYMBOLS: &[&str] = &["┼", "┤", "╶", "╴", "─", "╰", "╭", "╮", "╯", "│"];

impl Default for Config {
    fn default() -> Self {
        Self {
            color: WHITE.to_string(),
        }
    }
}

impl Config {
    pub fn color(mut self, color: &str) -> Self {
        self.color = color.to_string();
        self
    }
}

pub struct Series {
    config: Config,
    data: Vec<f64>,
}

impl Series {
    pub fn new(data: Vec<f64>, config: Config) -> Self {
        Self { data, config }
    }
}

// Function to apply color to a string (symbol)
pub fn colored(s: &str, color: &str) -> String {
    format!("\x1b[{}m{}\x1b[{}m", color, s, RESET)
}

// Main plotting function
pub fn plot(series_input: Vec<Series>) -> String {
    let series = series_input;

    // Handle the case where a single series (Vec<f64>) is provided
    if series.is_empty() || series[0].data.is_empty() {
        return String::new();
    }

    // Initialize min and max
    let mut min_value = series[0].data[0];
    let mut max_value = series[0].data[0];

    // Find min and max values
    for serie in &series {
        for value in serie.data.clone() {
            min_value = min_value.min(value);
            max_value = max_value.max(value);
        }
    }

    let range = (max_value - min_value).abs();
    let height = range;
    let ratio = if range != 0.0 { height / range } else { 1.0 };
    let miny = (min_value * ratio).round() as isize;
    let maxy = (max_value * ratio).round() as isize;
    let rows = (maxy - miny).unsigned_abs();

    let labels = (miny..=maxy)
        .map(|y| max_value - ((y - miny) as f64) * range / rows as f64)
        .map(|x| format!("{:.2}", x));

    let label_maxlen = labels.clone().map(|x| x.len()).max().unwrap_or(0);

    let offset = label_maxlen + 1;
    // Determine the width of the plot
    let width = series.iter().map(|s| s.data.len()).max().unwrap_or(0) + offset;

    let labels: Vec<_> = labels.collect();

    // Initialize result grid with spaces
    let mut result = vec![vec![" ".to_string(); width]; rows + 1];

    // Y-Axis and labels
    for y in miny..=maxy {
        let y_index = (y - miny) as usize;
        let label = labels[y_index].clone();
        let label_chars: Vec<char> = label.chars().collect();

        for (i, &ch) in label_chars.iter().enumerate() {
            result[y_index][i] = ch.to_string();
        }
        result[y_index][offset - 1] = SYMBOLS[0].to_string();
    }

    // Plot the series
    for serie in series {
        let current_color = serie.config.color.clone();
        // Handle first value
        let y0 = (serie.data[0] * ratio).round() as isize - miny;
        let row0 = rows - y0 as usize;
        if row0 < result.len() && offset - 1 < width {
            result[row0][offset - 1] = colored(SYMBOLS[0], &current_color);
        }

        // Plot the line
        for x in 0..serie.data.len() - 1 {
            let y0 = (serie.data[x] * ratio).round() as isize - miny;
            let y1 = (serie.data[x + 1] * ratio).round() as isize - miny;

            if y0 == y1 {
                let row = rows - y0 as usize;
                if row < result.len() && x + offset < width {
                    result[row][x + offset] = colored(SYMBOLS[4], &current_color);
                }
            } else {
                let row0 = rows - y0 as usize;
                let row1 = rows - y1 as usize;

                // Determine the symbols based on the slope
                let symbol1 = if y0 > y1 { SYMBOLS[5] } else { SYMBOLS[6] };
                let symbol2 = if y0 > y1 { SYMBOLS[7] } else { SYMBOLS[8] };

                if row1 < result.len() && x + offset < width {
                    result[row1][x + offset] = colored(symbol1, &current_color);
                }
                if row0 < result.len() && x + offset < width {
                    result[row0][x + offset] = colored(symbol2, &current_color);
                }

                // Fill in the vertical lines between points
                for y in min(y0, y1) + 1..max(y0, y1) {
                    let row = rows - y as usize;
                    if row < result.len() && x + offset < width {
                        result[row][x + offset] = colored(SYMBOLS[9], &current_color);
                    }
                }
            }
        }
    }

    // Convert result grid to string
    result
        .into_iter()
        .map(|row| row.join(""))
        .collect::<Vec<String>>()
        .join("\n")
}
