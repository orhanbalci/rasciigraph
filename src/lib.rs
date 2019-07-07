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

    let (min, max) = min_max(series_inner);

    let interval = (max - min).abs();
    if config.height <= 0 {
        config.height = interval as u32 * ((-interval.log10().ceil()) as u32).pow(10);
    } else {
        config.height = interval as u32;
    }

    if config.offset <= 0 {
        config.offset = 3;
    }

    "".to_string()
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

fn min_max(series: Vec<f64>) -> (f64, f64) {
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
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
