use std::f64::consts::PI;

use plotters::prelude::*;

fn f(xi: f64) -> f64 {
    1. / (1. + 25. * xi * xi)
}

fn knot(i: f64, n: f64) -> f64 {
    ((2. * i) / n) - 1.
}

fn chebyshev(left: f64, right: f64, i: f64, n: f64) -> f64 {
    let cos = f64::cos(PI * (2. * i + 1.) / (2. * n + 2.));

    ((left + right) + (right - left) * cos) * 0.5
}

fn chebyshev_knots(degree: usize) -> Vec<f64> {
    let mut ret: Vec<f64> = vec![];
    let left = -1.;
    let right = 1.;
    for i in 0..degree + 1 {
        ret.push(chebyshev(left, right, i as f64, degree as f64));
    }
    ret
}

fn knots(degree: usize) -> Vec<f64> {
    let mut ret: Vec<f64> = Vec::new();
    for i in 0..degree + 1 {
        ret.push(knot(i as f64, degree as f64))
    }
    ret
}

fn divided_differences(size: usize, knots: &Vec<f64>, applied: &Vec<f64>) -> Vec<f64> {
    let mut ret = vec![vec![0.0; size]; size];

    for i in 0..size {
        ret[i][0] = applied[i];
    }

    for j in 1..size {
        for i in 0..(size - j) {
            ret[i][j] = (ret[i + 1][j - 1] - ret[i][j - 1]) / (knots[i + j] - knots[i]);
        }
    }
    ret[0].clone()
}

fn poly(diffs: &Vec<f64>, knots: &Vec<f64>, x: f64) -> f64 {
    let mut res = diffs[0];
    let mut product = 1.0;
    for i in 1..diffs.len() {
        product *= x - knots[i - 1];
        res += diffs[i] * product;
    }
    res
}

fn paint(
    knots_fn: fn(usize) -> Vec<f64>,
    file_name: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::gif(file_name, (800, 600), 1000)?.into_drawing_area();
    root.fill(&WHITE)?;
    for i in 3..14 {
        let mut chart = ChartBuilder::on(&root)
            .caption("interpolation", ("sans-serif", 50))
            .build_cartesian_2d(-1.0..1.0, -1.0..1.0)?;

        let knots = knots_fn(i);
        let applied = knots.iter().map(|&elem: &f64| f(elem)).collect();
        let diffs = divided_differences(knots.len(), &knots, &applied);
        let series = LineSeries::new(
            (-1000..1000)
                .map(|x| x as f64 / 1000.0)
                .map(|x| (x, (f(x) - poly(&diffs, &knots, x)).abs())),
            &RED,
        );

        chart.draw_series(series)?;
        root.present()?;
    }
    //
    Ok(())
}

fn paint_errors(
    knots_fn: fn(usize) -> Vec<f64>,
    file_name: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new(file_name.as_str(), (800, 600)).into_drawing_area();

    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .margin(10)
        .caption("Errors", ("sans-serif", 40))
        .set_label_area_size(LabelAreaPosition::Left, 60)
        .set_label_area_size(LabelAreaPosition::Right, 60)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .build_cartesian_2d(3..15, -100.0..100.0)?;

    chart
        .configure_mesh()
        .disable_x_mesh()
        .disable_y_mesh()
        .max_light_lines(4)
        .draw()?;

    chart.draw_series(LineSeries::new(
        (3..15).map(|x| {
            let knots = knots_fn(i);
            let applied = knots.iter().map(|&elem: &f64| f(elem)).collect();
            let diffs = divided_differences(knots.len(), &knots, &applied);
            (-1000..1000)
                .map(|x| x as f64 / 1000.0)
                .map(|x| (f(x) - poly(&diffs, &knots, x)).abs())
                .max();
        }),
        &RED,
    ));

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    paint(knots, String::from("normal.gif"))?;
    paint(chebyshev_knots, String::from("chebyshev.gif"))
}
