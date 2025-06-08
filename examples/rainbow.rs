#[cfg(feature = "color")]
use colored::Color;
#[cfg(feature = "color")]
use rasciigraph::{plot_many_colored, Config};

#[cfg(feature = "color")]
fn main() {
    // Generate concentric semi-circles
    let mut data = vec![vec![]; 6];
    for i in 0i64..6 {
        for x in -40i64..=40 {
            let v = if x >= -(40 - i) && x <= (40 - i) {
                Some(((40 - i) as f64).powi(2) - (x as f64).powi(2))
                    .map(|v| v.sqrt() / 2.0)
                    .unwrap_or(f64::NAN)
            } else {
                f64::NAN
            };
            data[i as usize].push(v);
        }
    }

    // Define the configuration
    let config = Config::default()
        .with_height(20)
        .with_width(80)
        .with_caption("Concentric Semi-Circles".to_string())
        .with_caption_color(Color::Blue)
        .with_axis_color(Color::Green)
        .with_series_colors(vec![
            Color::Red,
            Color::Yellow,
            Color::Green,
            Color::Blue,
            Color::Magenta,
            Color::Cyan,
        ]);

    // Generate the plot
    let result = plot_many_colored(data, config);

    // Print the result
    println!("{}", result);
}
