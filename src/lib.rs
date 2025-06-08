use std::vec::Vec;

#[cfg(feature = "color")]
use colored::Color;
#[cfg(feature = "color")]
use colored::ColoredString;
#[cfg(feature = "color")]
use colored::Colorize;

pub struct Config {
    width: u32,
    height: u32,
    offset: u32,
    caption: String,
    #[cfg(feature = "color")]
    caption_color: Color,
    #[cfg(feature = "color")]
    axis_color: Color,
    #[cfg(feature = "color")]
    label_color: Color,
    #[cfg(feature = "color")]
    series_colors: Vec<Color>,
    #[cfg(feature = "color")]
    series_legends: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            width: 0,
            height: 0,
            offset: 0,
            caption: String::new(),
            #[cfg(feature = "color")]
            caption_color: Color::White,
            #[cfg(feature = "color")]
            axis_color: Color::White,
            #[cfg(feature = "color")]
            label_color: Color::White,
            #[cfg(feature = "color")]
            series_colors: vec![],
            #[cfg(feature = "color")]
            series_legends: Vec::new(),
        }
    }
}

impl Config {
    pub fn with_caption(mut self, caption: String) -> Self {
        self.caption = caption;
        self
    }

    pub fn with_height(mut self, height: u32) -> Self {
        self.height = height;
        self
    }

    pub fn with_width(mut self, width: u32) -> Self {
        self.width = width;
        self
    }

    pub fn with_offset(mut self, offset: u32) -> Self {
        self.offset = offset;
        self
    }

    #[cfg(feature = "color")]
    pub fn with_caption_color(mut self, color: Color) -> Self {
        self.caption_color = color;
        self
    }

    #[cfg(feature = "color")]
    pub fn with_axis_color(mut self, color: Color) -> Self {
        self.axis_color = color;
        self
    }

    #[cfg(feature = "color")]
    pub fn with_label_color(mut self, color: Color) -> Self {
        self.label_color = color;
        self
    }

    #[cfg(feature = "color")]
    pub fn with_series_colors(mut self, colors: Vec<Color>) -> Self {
        self.series_colors = colors;
        self
    }

    #[cfg(feature = "color")]
    pub fn with_series_legends(mut self, legends: Vec<String>) -> Self {
        self.series_legends = legends;
        self
    }
}

pub fn plot(series: Vec<f64>, config: Config) -> String {
    plot_many(vec![series], config)
}

pub fn plot_many(mut series: Vec<Vec<f64>>, mut config: Config) -> String {
    let mut len_max = series.iter().map(|s| s.len()).max().unwrap_or(0);
    if config.width > 0 {
        series.iter_mut().for_each(|s| {
            if s.len() < len_max {
                s.extend(vec![f64::NAN].repeat(len_max - s.len()))
            }
            *s = interpolate(s, config.width);
        });
        len_max = config.width as usize;
    }

    let mut min = f64::MAX;
    let mut max = f64::MIN;

    (min, max) = series.iter().map(|s| min_max(s)).fold(
        (min, max),
        |(current_min, current_max), (next_min, next_max)| {
            (
                f64::min(next_min, current_min),
                f64::max(next_max, current_max),
            )
        },
    );

    let interval = (max - min).abs();
    if config.height == 0 {
        if interval == 0f64 {
            config.height = 3;
        } else if interval <= 1f64 {
            config.height =
                (interval * f64::from(10i32.pow((-interval.log10()).ceil() as u32))) as u32;
        } else {
            config.height = interval as u32;
        }
    }

    if config.offset == 0 {
        config.offset = 3;
    }

    let ratio = if interval != 0f64 {
        f64::from(config.height) / interval
    } else {
        1f64
    };

    let min2 = (min * ratio).round();
    let max2 = (max * ratio).round();

    let int_min2 = min2 as i32;
    let int_max2 = max2 as i32;

    let rows = f64::from(int_max2 - int_min2).abs() as i32;
    let width = len_max as u32 + config.offset;

    let mut plot: Vec<Vec<String>> = Vec::new();

    for _i in 0..=rows {
        let mut line = Vec::<String>::new();
        for _j in 0..width {
            line.push(" ".to_string());
        }
        plot.push(line);
    }

    let mut precision = 2;
    let log_maximum = if min == 0f64 && max == 0f64 {
        -1f64
    } else {
        f64::max(max.abs(), min.abs()).log10()
    };

    if log_maximum < 0f64 {
        if log_maximum % 1f64 != 0f64 {
            precision += log_maximum.abs() as i32;
        } else {
            precision += (log_maximum.abs() - 1f64) as i32;
        }
    } else if log_maximum > 2f64 {
        precision = 0;
    }

    let max_number_label_length = format!("{:.*}", precision as usize, max).len();
    let min_number_label_length = format!("{:.*}", precision as usize, min).len();

    let max_label_width = usize::max(max_number_label_length, min_number_label_length);

    for y in int_min2..=int_max2 {
        let magnitude = if rows > 0 {
            max - f64::from(y - int_min2) * interval / f64::from(rows)
        } else {
            f64::from(y)
        };
        let label = format!(
            "{number:LW$.PREC$}",
            LW = max_label_width + 1,
            PREC = precision as usize,
            number = magnitude
        );
        let w = (y - int_min2) as usize;
        let h = f64::max(f64::from(config.offset) - label.len() as f64, 0f64) as usize;
        plot[w][h] = label;
        plot[w][(config.offset - 1) as usize] = "┤".to_string();
    }

    for series_inner in series {
        let mut y0;
        let mut y1;
        if !series_inner[0].is_nan() {
            y0 = ((series_inner[0] * ratio).round() - min2) as i32;
            plot[(rows - y0) as usize][(config.offset - 1) as usize] = "┼".to_string();
        }

        for x in 0..series_inner.len() - 1 {
            if series_inner[x].is_nan() && series_inner[x + 1].is_nan() {
                continue;
            }
            if series_inner[x + 1].is_nan() && !series_inner[x].is_nan() {
                y0 = ((series_inner[x] * ratio).round() - f64::from(int_min2)) as i32;
                plot[(rows - y0) as usize][(x as u32 + config.offset) as usize] = "─".to_string();
                continue;
            }
            if series_inner[x].is_nan() && !series_inner[x + 1].is_nan() {
                y1 = ((series_inner[x + 1] * ratio).round() - f64::from(int_min2)) as i32;
                plot[(rows - y1) as usize][(x as u32 + config.offset) as usize] = "─".to_string();
                continue;
            }
            y0 = ((series_inner[x] * ratio).round() - f64::from(int_min2)) as i32;
            y1 = ((series_inner[x + 1] * ratio).round() - f64::from(int_min2)) as i32;

            if y0 == y1 {
                plot[(rows - y0) as usize][(x as u32 + config.offset) as usize] = "─".to_string();
            } else {
                if y0 > y1 {
                    plot[(rows - y1) as usize][(x as u32 + config.offset) as usize] =
                        "╰".to_string();
                    plot[(rows - y0) as usize][(x as u32 + config.offset) as usize] =
                        "╮".to_string();
                } else {
                    plot[(rows - y1) as usize][(x as u32 + config.offset) as usize] =
                        "╭".to_string();
                    plot[(rows - y0) as usize][(x as u32 + config.offset) as usize] =
                        "╯".to_string();
                }

                let start = f64::min(f64::from(y0), f64::from(y1)) as i32 + 1;
                let end = f64::max(f64::from(y0), f64::from(y1)) as i32;

                for y in start..end {
                    plot[(rows - y) as usize][(x as u32 + config.offset) as usize] =
                        "│".to_string();
                }
            }
        }
    }

    let mut res: String = plot
        .into_iter()
        .map(|line| line.join(""))
        .collect::<Vec<String>>()
        .join("\n");
    res.pop();
    if !config.caption.is_empty() {
        res.push('\n');
        res.push_str(
            std::iter::repeat(" ")
                .take(config.offset as usize + max_label_width as usize)
                .collect::<String>()
                .as_ref(),
        );
        if config.caption.len() < len_max {
            res.push_str(
                std::iter::repeat(" ")
                    .take((len_max - config.caption.len()) / 2)
                    .collect::<String>()
                    .as_ref(),
            );
        }
        res.push_str(config.caption.as_ref());
    }
    res
}

#[cfg(feature = "color")]
pub fn plot_colored(series: Vec<f64>, config: Config) -> ColoredString {
    plot_many_colored(vec![series], config)
}

#[cfg(feature = "color")]
pub fn plot_many_colored(mut series: Vec<Vec<f64>>, mut config: Config) -> ColoredString {
    let mut len_max = series.iter().map(|s| s.len()).max().unwrap_or(0);
    if config.width > 0 {
        series.iter_mut().for_each(|s| {
            if s.len() < len_max {
                s.extend(vec![f64::NAN].repeat(len_max - s.len()))
            }
            *s = interpolate(s, config.width);
        });
        len_max = config.width as usize;
    }

    let mut min = f64::MAX;
    let mut max = f64::MIN;

    (min, max) = series.iter().map(|s| min_max(s)).fold(
        (min, max),
        |(current_min, current_max), (next_min, next_max)| {
            (
                f64::min(next_min, current_min),
                f64::max(next_max, current_max),
            )
        },
    );

    let interval = (max - min).abs();
    if config.height == 0 {
        if interval == 0f64 {
            config.height = 3;
        } else if interval <= 1f64 {
            config.height =
                (interval * f64::from(10i32.pow((-interval.log10()).ceil() as u32))) as u32;
        } else {
            config.height = interval as u32;
        }
    }

    if config.offset == 0 {
        config.offset = 3;
    }

    let ratio = if interval != 0f64 {
        f64::from(config.height) / interval
    } else {
        1f64
    };

    let min2 = (min * ratio).round();
    let max2 = (max * ratio).round();

    let int_min2 = min2 as i32;
    let int_max2 = max2 as i32;

    let rows = f64::from(int_max2 - int_min2).abs() as i32;
    let width = len_max as u32 + config.offset;

    let mut plot: Vec<Vec<ColoredString>> = Vec::new();

    for _i in 0..=rows {
        let mut line = Vec::<ColoredString>::new();
        for _j in 0..width {
            line.push(" ".to_string().into());
        }
        plot.push(line);
    }

    let mut precision = 2;
    let log_maximum = if min == 0f64 && max == 0f64 {
        -1f64
    } else {
        f64::max(max.abs(), min.abs()).log10()
    };

    if log_maximum < 0f64 {
        if log_maximum % 1f64 != 0f64 {
            precision += log_maximum.abs() as i32;
        } else {
            precision += (log_maximum.abs() - 1f64) as i32;
        }
    } else if log_maximum > 2f64 {
        precision = 0;
    }

    let max_number_label_length = format!("{:.*}", precision as usize, max).len();
    let min_number_label_length = format!("{:.*}", precision as usize, min).len();

    let max_label_width = usize::max(max_number_label_length, min_number_label_length);

    for y in int_min2..=int_max2 {
        let magnitude = if rows > 0 {
            max - f64::from(y - int_min2) * interval / f64::from(rows)
        } else {
            f64::from(y)
        };
        let label = format!(
            "{number:LW$.PREC$}",
            LW = max_label_width + 1,
            PREC = precision as usize,
            number = magnitude
        );
        let w = (y - int_min2) as usize;
        let h = f64::max(f64::from(config.offset) - label.len() as f64, 0f64) as usize;
        plot[w][h] = label.color(config.axis_color);
        plot[w][(config.offset - 1) as usize] = "┤".to_string().color(config.axis_color);
    }

    for (i, series_inner) in series.iter().enumerate() {
        let mut y0;
        let mut y1;
        if !series_inner[0].is_nan() {
            y0 = ((series_inner[0] * ratio).round() - min2) as i32;
            plot[(rows - y0) as usize][(config.offset - 1) as usize] =
                "┼".to_string().color(config.axis_color);
        }

        for x in 0..series_inner.len() - 1 {
            if series_inner[x].is_nan() && series_inner[x + 1].is_nan() {
                continue;
            }
            if series_inner[x + 1].is_nan() && !series_inner[x].is_nan() {
                y0 = ((series_inner[x] * ratio).round() - f64::from(int_min2)) as i32;
                plot[(rows - y0) as usize][(x as u32 + config.offset) as usize] =
                    "─".to_string().color(config.series_colors[i]);
                continue;
            }
            if series_inner[x].is_nan() && !series_inner[x + 1].is_nan() {
                y1 = ((series_inner[x + 1] * ratio).round() - f64::from(int_min2)) as i32;
                plot[(rows - y1) as usize][(x as u32 + config.offset) as usize] =
                    "─".to_string().color(config.series_colors[i]);
                continue;
            }
            y0 = ((series_inner[x] * ratio).round() - f64::from(int_min2)) as i32;
            y1 = ((series_inner[x + 1] * ratio).round() - f64::from(int_min2)) as i32;

            if y0 == y1 {
                plot[(rows - y0) as usize][(x as u32 + config.offset) as usize] =
                    "─".to_string().color(config.series_colors[i]);
            } else {
                if y0 > y1 {
                    plot[(rows - y1) as usize][(x as u32 + config.offset) as usize] =
                        "╰".to_string().color(config.series_colors[i]);
                    plot[(rows - y0) as usize][(x as u32 + config.offset) as usize] =
                        "╮".to_string().color(config.series_colors[i]);
                } else {
                    plot[(rows - y1) as usize][(x as u32 + config.offset) as usize] =
                        "╭".to_string().color(config.series_colors[i]);
                    plot[(rows - y0) as usize][(x as u32 + config.offset) as usize] =
                        "╯".to_string().color(config.series_colors[i]);
                }

                let start = f64::min(f64::from(y0), f64::from(y1)) as i32 + 1;
                let end = f64::max(f64::from(y0), f64::from(y1)) as i32;

                for y in start..end {
                    plot[(rows - y) as usize][(x as u32 + config.offset) as usize] =
                        "│".to_string().color(config.series_colors[i]);
                }
            }
        }
    }

    let mut res: String = plot
        .into_iter()
        .map(|line| line.into_iter().map(|s| s.to_string()).collect::<String>())
        .collect::<Vec<String>>()
        .join("\n");

    //res.pop();

    let mut caption = String::new();

    if !config.caption.is_empty() {
        caption.push('\n');
        caption.push_str(
            std::iter::repeat(" ")
                .take(config.offset as usize + max_label_width as usize)
                .collect::<String>()
                .as_ref(),
        );
        if config.caption.len() < len_max {
            caption.push_str(
                std::iter::repeat(" ")
                    .take((len_max - config.caption.len()) / 2)
                    .collect::<String>()
                    .as_ref(),
            );
        }
        caption.push_str(
            config
                .caption
                .color(config.caption_color)
                .to_string()
                .as_ref(),
        );
    }
    res.push_str(caption.as_ref());
    res.into()
}

fn interpolate(series: &[f64], count: u32) -> Vec<f64> {
    let mut result = Vec::new();
    let spring_factor = (series.len() - 1) as f64 / f64::from(count - 1);
    result.push(series[0]);
    for i in 1..count - 1 {
        let spring = f64::from(i) * spring_factor;
        let before = spring.floor();
        let after = spring.ceil();
        let at_point = spring - before;
        result.push(linear_interpolate(
            series[before as usize],
            series[after as usize],
            at_point,
        ))
    }
    result.push(series[series.len() - 1]);
    result
}

fn linear_interpolate(before: f64, after: f64, at_point: f64) -> f64 {
    before + (after - before) * at_point
}

fn min_max(series: &[f64]) -> (f64, f64) {
    let min = series
        .iter()
        .fold(std::f64::MAX, |accu, &x| if x < accu { x } else { accu });
    let max = series
        .iter()
        .fold(std::f64::MIN, |accu, &x| if x > accu { x } else { accu });
    (min, max)
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {

    macro_rules! graph_eq {
        ($fname:ident ? [$($series:expr),*]  => $rhs:expr) => {
            #[test]
            fn $fname(){
              let res = crate::plot(vec![$(f64::from($series),)*], crate::Config::default());
              assert_eq!(res, $rhs);
        }};
        ($fname:ident ? [$($series:expr),*]  ? $config:expr => $rhs:expr) => {
            #[test]
            fn $fname(){
              let res = crate::plot(vec![$(f64::from($series),)*], $config);
              assert_eq!(res, $rhs);
        }};
    }

    graph_eq!(test_ones  ? [1, 1, 1, 1, 1] => " 1.00 ┼────");
    graph_eq!(test_zeros ? [0, 0, 0, 0, 0] => " 0.00 ┼────");
    graph_eq!(test_three ? [2, 1, 1, 2, -2, 5, 7, 11, 3, 7, 1] => " 11.00 ┤      ╭╮   
 10.00 ┤      ││   
  9.00 ┤      ││   
  8.00 ┤      ││   
  7.00 ┤     ╭╯│╭╮ 
  6.00 ┤     │ │││ 
  5.00 ┤    ╭╯ │││ 
  4.00 ┤    │  │││ 
  3.00 ┤    │  ╰╯│ 
  2.00 ┼╮ ╭╮│    │ 
  1.00 ┤╰─╯││    ╰ 
  0.00 ┤   ││      
 -1.00 ┤   ││      
 -2.00 ┤   ╰╯     ");

    graph_eq!(test_four ? [2, 1, 1, 2, -2, 5, 7, 11, 3, 7, 4, 5, 6, 9, 4, 0, 6, 1, 5, 3, 6, 2] ? 
    crate::Config::default().with_caption("Plot using asciigraph.".to_string()) 
    => " 11.00 ┤      ╭╮              
 10.00 ┤      ││              
  9.00 ┤      ││    ╭╮        
  8.00 ┤      ││    ││        
  7.00 ┤     ╭╯│╭╮  ││        
  6.00 ┤     │ │││ ╭╯│ ╭╮  ╭╮ 
  5.00 ┤    ╭╯ │││╭╯ │ ││╭╮││ 
  4.00 ┤    │  ││╰╯  ╰╮││││││ 
  3.00 ┤    │  ╰╯     ││││╰╯│ 
  2.00 ┼╮ ╭╮│         ││││  ╰ 
  1.00 ┤╰─╯││         ││╰╯    
  0.00 ┤   ││         ╰╯      
 -1.00 ┤   ││                 
 -2.00 ┤   ╰╯                
        Plot using asciigraph.");

    graph_eq!(test_five ? [ 2, 1, 1, 2, -2, 5, 7, 11, 3, 7, 4, 5, 6, 9, 4, 0, 6, 1, 5, 3, 6, 2] ? 
                crate::Config::default().with_caption("Plot using asciigraph.".to_string()) 
     => " 11.00 ┤      ╭╮              
 10.00 ┤      ││              
  9.00 ┤      ││    ╭╮        
  8.00 ┤      ││    ││        
  7.00 ┤     ╭╯│╭╮  ││        
  6.00 ┤     │ │││ ╭╯│ ╭╮  ╭╮ 
  5.00 ┤    ╭╯ │││╭╯ │ ││╭╮││ 
  4.00 ┤    │  ││╰╯  ╰╮││││││ 
  3.00 ┤    │  ╰╯     ││││╰╯│ 
  2.00 ┼╮ ╭╮│         ││││  ╰ 
  1.00 ┤╰─╯││         ││╰╯    
  0.00 ┤   ││         ╰╯      
 -1.00 ┤   ││                 
 -2.00 ┤   ╰╯                
        Plot using asciigraph." );

    graph_eq!(test_six ? [0.2, 0.1, 0.2, 2, -0.9, 0.7, 0.91, 0.3, 0.7, 0.4, 0.5] ? 
    crate::Config::default().with_caption("Plot using asciigraph.".to_string())
    => "  2.00 ┤  ╭╮ ╭╮    
  0.55 ┼──╯│╭╯╰─── 
 -0.90 ┤   ╰╯     
        Plot using asciigraph." );

    graph_eq!(test_seven ? [2, 1, 1, 2, -2, 5, 7, 11, 3, 7, 1] ? 
    crate::Config::default().with_height(4).with_offset(3)
    => " 11.00 ┤      ╭╮   
  7.75 ┤    ╭─╯│╭╮ 
  4.50 ┼╮ ╭╮│  ╰╯│ 
  1.25 ┤╰─╯││    ╰ 
 -2.00 ┤   ╰╯     "
    );

    graph_eq!(test_eight ? [0.453, 0.141, 0.951, 0.251, 0.223, 0.581, 0.771, 0.191, 0.393, 0.617, 0.478]
    => " 0.95 ┤ ╭╮        
 0.85 ┤ ││  ╭╮    
 0.75 ┤ ││  ││    
 0.65 ┤ ││ ╭╯│ ╭╮ 
 0.55 ┤ ││ │ │ │╰ 
 0.44 ┼╮││ │ │╭╯  
 0.34 ┤│││ │ ││   
 0.24 ┤││╰─╯ ╰╯   
 0.14 ┤╰╯        ");

    graph_eq!(test_nine ? [0.01, 0.004, 0.003, 0.0042, 0.0083, 0.0033, 0.0079] 
    => " 0.010 ┼╮      
 0.009 ┤│      
 0.008 ┤│  ╭╮╭ 
 0.007 ┤│  │││ 
 0.006 ┤│  │││ 
 0.005 ┤│  │││ 
 0.004 ┤╰╮╭╯││ 
 0.003 ┤ ╰╯ ╰╯"
    );

    graph_eq!(test_ten ? [192, 431, 112, 449, -122, 375, 782, 123, 911, 1711, 172] ? crate::Config::default().with_height(10)
    => " 1711 ┤        ╭╮ 
 1528 ┤        ││ 
 1344 ┤        ││ 
 1161 ┤        ││ 
  978 ┤       ╭╯│ 
  794 ┤     ╭╮│ │ 
  611 ┤     │││ │ 
  428 ┤╭╮╭╮╭╯││ │ 
  245 ┼╯╰╯││ ╰╯ ╰ 
   61 ┤   ││      
 -122 ┤   ╰╯     ");

    graph_eq!(test_eleven ? [0.3189989805, 0.149949026, 0.30142492354, 0.195129182935, 0.3142492354, 
    0.1674974513, 0.3142492354, 0.1474974513, 0.3047974513] ?
    crate::Config::default().with_width(30).with_height(5).with_caption("Plot with custom height & width.".to_string())
        => " 0.32 ┼╮            ╭─╮     ╭╮     ╭ 
 0.29 ┤╰╮    ╭─╮   ╭╯ │    ╭╯│     │ 
 0.26 ┤ │   ╭╯ ╰╮ ╭╯  ╰╮  ╭╯ ╰╮   ╭╯ 
 0.23 ┤ ╰╮ ╭╯   ╰╮│    ╰╮╭╯   ╰╮ ╭╯  
 0.20 ┤  ╰╮│     ╰╯     ╰╯     │╭╯   
 0.16 ┤   ╰╯                   ╰╯   
       Plot with custom height & width."
    );

    graph_eq!(test_twelve ? [0, 0, 0, 0, 1.5, 0, 0, -0.5, 9, -3, 0, 0, 1, 2, 1, 0, 0, 0, 0,
				0, 0, 0, 0, 1.5, 0, 0, -0.5, 8, -3, 0, 0, 1, 2, 1, 0, 0, 0, 0,
				0, 0, 0, 0, 1.5, 0, 0, -0.5, 10, -3, 0, 0, 1, 2, 1, 0, 0, 0, 0] ? 
                crate::Config::default().with_offset(10).with_height(10).with_caption("I'm a doctor, not an engineer.".to_string())
    => "     10.00    ┤                                             ╭╮          
      8.70    ┤       ╭╮                                    ││          
      7.40    ┤       ││                 ╭╮                 ││          
      6.10    ┤       ││                 ││                 ││          
      4.80    ┤       ││                 ││                 ││          
      3.50    ┤       ││                 ││                 ││          
      2.20    ┤       ││   ╭╮            ││   ╭╮            ││   ╭╮     
      0.90    ┤   ╭╮  ││  ╭╯╰╮       ╭╮  ││  ╭╯╰╮       ╭╮  ││  ╭╯╰╮    
     -0.40    ┼───╯╰──╯│╭─╯  ╰───────╯╰──╯│╭─╯  ╰───────╯╰──╯│╭─╯  ╰─── 
     -1.70    ┤        ││                 ││                 ││         
     -3.00    ┤        ╰╯                 ╰╯                 ╰╯        
                            I'm a doctor, not an engineer.");

    graph_eq!(test_thirteen ? [-5, -2, -3, -4, 0, -5, -6, -7, -8, 0, -9, -3, -5, -2, -9, -3, -1]
    => "  0.00 ┤   ╭╮   ╭╮       
 -1.00 ┤   ││   ││     ╭ 
 -2.00 ┤╭╮ ││   ││  ╭╮ │ 
 -3.00 ┤│╰╮││   ││╭╮││╭╯ 
 -4.00 ┤│ ╰╯│   │││││││  
 -5.00 ┼╯   ╰╮  │││╰╯││  
 -6.00 ┤     ╰╮ │││  ││  
 -7.00 ┤      ╰╮│││  ││  
 -8.00 ┤       ╰╯││  ││  
 -9.00 ┤         ╰╯  ╰╯ ");

    graph_eq!(test_fourteen ? [-0.000018527, -0.021, -0.00123, 0.00000021312, 
    -0.0434321234, -0.032413241234, 0.0000234234] ?
        crate::Config::default().with_height(5).with_width(45)
        => "  0.000 ┼─╮           ╭────────╮                    ╭ 
 -0.008 ┤ ╰──╮     ╭──╯        ╰─╮                ╭─╯ 
 -0.017 ┤    ╰─────╯             ╰╮             ╭─╯   
 -0.025 ┤                         ╰─╮         ╭─╯     
 -0.034 ┤                           ╰╮   ╭────╯       
 -0.042 ┤                            ╰───╯           "
    );

    graph_eq!(test_fifteen ? [57.76, 54.04, 56.31, 57.02, 59.5, 52.63, 52.97, 56.44, 56.75, 52.96, 55.54, 
    55.09, 58.22, 56.85, 60.61, 59.62, 59.73, 59.93, 56.3, 54.69, 55.32, 54.03, 50.98, 50.48, 54.55, 47.49, 
    55.3, 46.74, 46, 45.8, 49.6, 48.83, 47.64, 46.61, 54.72, 42.77, 50.3, 42.79, 41.84, 44.19, 43.36, 45.62, 
    45.09, 44.95, 50.36, 47.21, 47.77, 52.04, 47.46, 44.19, 47.22, 45.55, 40.65, 39.64, 37.26, 40.71, 42.15, 
    36.45, 39.14, 36.62]
   => " 60.61 ┤             ╭╮ ╭╮                                          
 59.60 ┤   ╭╮        │╰─╯│                                          
 58.60 ┤   ││      ╭╮│   │                                          
 57.59 ┼╮ ╭╯│      │││   │                                          
 56.58 ┤│╭╯ │ ╭─╮  │╰╯   ╰╮                                         
 55.58 ┤││  │ │ │╭─╯      │╭╮    ╭╮                                 
 54.57 ┤╰╯  │ │ ││        ╰╯╰╮ ╭╮││      ╭╮                         
 53.56 ┤    │╭╯ ╰╯           │ ││││      ││                         
 52.56 ┤    ╰╯               │ ││││      ││           ╭╮            
 51.55 ┤                     ╰╮││││      ││           ││            
 50.54 ┤                      ╰╯│││      ││╭╮      ╭╮ ││            
 49.54 ┤                        │││  ╭─╮ ││││      ││ ││            
 48.53 ┤                        │││  │ │ ││││      ││ ││            
 47.52 ┤                        ╰╯│  │ ╰╮││││      │╰─╯╰╮╭╮         
 46.52 ┤                          ╰─╮│  ╰╯│││      │    │││         
 45.51 ┤                            ╰╯    │││   ╭──╯    ││╰╮        
 44.50 ┤                                  │││ ╭╮│       ╰╯ │        
 43.50 ┤                                  ││╰╮│╰╯          │        
 42.49 ┤                                  ╰╯ ╰╯            │   ╭╮   
 41.48 ┤                                                   │   ││   
 40.48 ┤                                                   ╰╮ ╭╯│   
 39.47 ┤                                                    ╰╮│ │╭╮ 
 38.46 ┤                                                     ││ │││ 
 37.46 ┤                                                     ╰╯ │││ 
 36.45 ┤                                                        ╰╯╰"
    );

    #[test]
    fn test_min_max() {
        assert_eq!(
            (-2f64, 11f64),
            crate::min_max(&[2f64, 1f64, 1f64, 2f64, -2f64, 5f64, 7f64, 11f64, 3f64, 7f64, 1f64])
        );
    }

    #[test]
    fn test_plot_many(){
        let res = super::plot_many(vec![vec![0f64,1.0,2.0,3.0,3.0,3.0,2.0,0.0], vec![5.0,4.0,2.0,1.0, 4.0, 6.0,6.0]], super::Config::default());
        let exp = " 6.00 ┤    ╭─  
 5.00 ┼╮   │   
 4.00 ┤╰╮ ╭╯   
 3.00 ┤ │╭│─╮  
 2.00 ┤ ╰╮│ ╰╮ 
 1.00 ┤╭╯╰╯  │ 
 0.00 ┼╯     ╰";
        assert_eq!(res, exp);
    }

    #[test]
    fn test_plot_many_two(){
        let res = super::plot_many(vec![vec![0.0f64, 1.0, 0.0], vec![2.0, 3.0, 4.0, 3.0, 2.0], vec![4.0, 5.0, 6.0, 7.0, 6.0, 5.0,4.0]], super::Config::default().with_width(21));
        let exp = " 7.00 ┤        ╭──╮         
 6.00 ┤    ╭───╯  ╰───╮     
 5.00 ┤ ╭──╯          ╰──╮  
 4.00 ┼─╯  ╭───╮         ╰─ 
 3.00 ┤ ╭──╯   ╰──╮         
 2.00 ┼─╯         ╰──       
 1.00 ┤ ╭───╮               
 0.00 ┼─╯   ╰─             ";
        assert_eq!(res, exp);
        // assert_eq!(res.as_bytes(), exp.as_bytes());
    }
    
    #[test]
    fn test_plot_many_three(){
        let res = super::plot_many(vec![vec![0.0f64, 0.0, 2.0, 2.0, f64::NAN], vec![1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0], vec![f64::NAN, f64::NAN, f64::NAN, 0.0, 0.0, 2.0, 2.0]], super::Config::default());
        let exp = " 2.00 ┤ ╭──╭─ 
 1.00 ┼────│─ 
 0.00 ┼─╯──╯ "; 
        assert_eq!(res, exp);
    }

    #[test]
    fn test_plot_many_four(){
        
        let res = super::plot_many(vec![vec![0.1f64, 0.2, 0.3, f64::NAN, 0.5, 0.6, 0.7, f64::NAN, f64::NAN, 0.9, 1.0]], super::Config::default());
        let exp = " 1.00 ┤         ╭ 
 0.90 ┤        ─╯ 
 0.80 ┤           
 0.70 ┤     ╭─    
 0.60 ┤    ╭╯     
 0.50 ┤   ─╯      
 0.40 ┤           
 0.30 ┤ ╭─        
 0.20 ┤╭╯         
 0.10 ┼╯         "; 
        assert_eq!(res, exp);
    }

}
