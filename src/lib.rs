use std::io::Read;
use std::vec::Vec;

pub struct Config {
    width: u32,
    height: u32,
    offset: u32,
    caption: String,
}

pub fn plot(series: Vec<f64>, mut config: Config) -> String {
    let mut series_inner = Vec::<f64>::new();
    if config.width > 0 {
        series_inner = interpolate(series, config.width);
    } else {
        series_inner = series;
    }

    let (min, max) = min_max(&series_inner);

    let interval = (max - min).abs();
    if config.height <= 0 {
        if interval as i32 <= 0 {
            config.height = (interval as i32 * 10i32.pow(-interval.log10().ceil() as u32)) as u32;
        } else {
            config.height = interval as u32;
        }
    }

    if config.offset <= 0 {
        config.offset = 3;
    }

    let ratio = if interval != 0f64 {
        config.height as f64 / interval
    } else {
        1f64
    };

    let min2 = (min * ratio).round();
    let max2 = (max * ratio).round();

    let int_min2 = min2 as i32;
    let int_max2 = max2 as i32;

    let rows = ((int_max2 - int_min2) as f64).abs() as i32;
    let width = series_inner.len() + config.offset as usize;

    let mut plot: Vec<Vec<String>> = Vec::new();

    for i in 0..rows + 1 {
        let mut line = Vec::<String>::new();
        for j in 0..width {
            line.push(" ".to_string());
        }
        plot.push(line);
    }

    let mut precision = 2;
    let mut log_maximum = f64::max(max.abs(), min.abs()).log10();
    if min == 0f64 && min == 0f64 {
        log_maximum = -1f64;
    }

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

    for y in int_min2..int_max2 + 1 {
        let magnitude = if rows > 0 {
            max - (y - int_min2) as f64 * interval / rows as f64
        } else {
            y as f64
        };
        let label = format!(
            "{number:LW$.PREC$}",
            LW = max_label_width + 1,
            PREC = precision as usize,
            number = magnitude
        );
        let w = (y - int_min2) as usize;
        let h = f64::max(config.offset as f64 - label.len() as f64, 0f64) as usize;
        plot[w][h] = label;
        if y == 0 {
            plot[w][(config.offset - 1) as usize] = "┼".to_string();
        } else {
            plot[w][(config.offset - 1) as usize] = "┤".to_string();
        };
    }

    let mut y0 = ((series_inner[0] * ratio).round() - min2) as i32;

    let mut y1: i32;
    plot[(rows - y0) as usize][(config.offset - 1) as usize] = "┼".to_string();

    for x in 0..series_inner.len() - 1 {
        y0 = ((series_inner[x + 0] * ratio).round() - int_min2 as f64) as i32;
        y1 = ((series_inner[x + 1] * ratio).round() - int_min2 as f64) as i32;

        if y0 == y1 {
            plot[(rows - y0) as usize][(x as u32 + config.offset) as usize] = "─".to_string();
        } else {
            if y0 > y1 {
                plot[(rows - y1) as usize][(x as u32 + config.offset) as usize] = "╰".to_string();
                plot[(rows - y0) as usize][(x as u32 + config.offset) as usize] = "╮".to_string();
            } else {
                plot[(rows - y1) as usize][(x as u32 + config.offset) as usize] = "╭".to_string();
                plot[(rows - y0) as usize][(x as u32 + config.offset) as usize] = "╯".to_string();
            }
        }

        let start = f64::min(y0 as f64, y1 as f64) as i32 + 1;
        let end = f64::max(y0 as f64, y1 as f64) as i32;

        for y in start..end {
            plot[(rows - y) as usize][(x as u32 + config.offset) as usize] = "│".to_string();
        }
    }

    let mut res: String = plot
        .into_iter()
        .map(|line| line.join(""))
        .collect::<Vec<String>>()
        .join("\n");
    res.pop();
    if !config.caption.is_empty() {
        res.push_str("\n");
        res.push_str(std::iter::repeat(" ").take(config.offset as usize + max_label_width + 2).collect::<String>().as_ref());
        res.push_str(config.caption.as_ref());
    }
    res
}

fn interpolate(series: Vec<f64>, count: u32) -> Vec<f64> {
    let mut result = Vec::new();
    let spring_factor = (series.len() - 1) as f64 / (count - 1) as f64;
    result.push(series[0]);
    for i in 1..count - 1 {
        let spring = i as f64 * spring_factor;
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

fn min_max(series: &Vec<f64>) -> (f64, f64) {
    let min = series
        .iter()
        .fold(std::f64::MAX, |accu, &x| if x < accu { x } else { accu });
    let max = series
        .iter()
        .fold(std::f64::MIN, |accu, &x| if x > accu { x } else { accu });
    (min, max)
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_min_max() {
        assert_eq!(
            (-2f64, 11f64),
            crate::min_max(&vec![
                2f64, 1f64, 1f64, 2f64, -2f64, 5f64, 7f64, 11f64, 3f64, 7f64, 1f64
            ])
        );
    }
    #[test]
    fn test_ones() {
        let res = crate::plot(
            vec![1f64, 1f64, 1f64, 1f64, 1f64],
            crate::Config {
                width: 0,
                height: 0,
                offset: 0,
                caption: "".to_string(),
            },
        );
        assert_eq!(res, " 1.00 ┼────");
    }

    #[test]
    fn test_zeros() {
        let res = crate::plot(
            vec![0f64, 0f64, 0f64, 0f64, 0f64],
            crate::Config {
                width: 0,
                height: 0,
                offset: 0,
                caption: "".to_string(),
            },
        );
        assert_eq!(res, " 0.00 ┼────");
    }

    #[test]
    fn third_test() {
        let res = crate::plot(
            vec![
                2f64, 1f64, 1f64, 2f64, -2f64, 5f64, 7f64, 11f64, 3f64, 7f64, 1f64,
            ],
            crate::Config {
                width: 0,
                height: 0,
                offset: 0,
                caption: "".to_string(),
            },
        );
        assert_eq!(
            res,
            " 11.00 ┤      ╭╮   
 10.00 ┤      ││   
  9.00 ┼      ││   
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
 -2.00 ┤   ╰╯     "
        );
    }

    #[test]
    fn fourth_test(){
         let res = crate::plot(
            vec![
                2f64, 1f64, 1f64, 2f64, -2f64, 5f64, 7f64, 11f64, 3f64, 7f64, 4f64, 5f64, 6f64, 9f64, 4f64, 0f64, 6f64, 1f64, 5f64, 3f64, 6f64, 2f64
            ],
            crate::Config {
                width: 0,
                height: 0,
                offset: 0,
                caption: "Plot using asciigraph.".to_string(),
            },
        );

        assert_eq!(
            res," 11.00 ┤      ╭╮              
 10.00 ┤      ││              
  9.00 ┼      ││    ╭╮        
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
          Plot using asciigraph.")
    }
}
