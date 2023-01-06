fn main() {
    let res = rasciigraph::plot_many(
        vec![
            vec![0.0f64, 1.0, 0.0],
            vec![2.0, 3.0, 4.0, 3.0, 2.0],
            vec![4.0, 5.0, 6.0, 7.0, 6.0, 5.0, 4.0],
        ],
        rasciigraph::Config::default().with_width(21),
    );
    print!("{}", res);
}
