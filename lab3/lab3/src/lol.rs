use plotters::prelude::*;

fn f(xi: f64) -> f64 {
    1. / (1. + 25. * xi * xi)
}

fn knot(i: f64, n: f64) -> f64 {
    ((2. * i) / n) - 1.
}

fn knots(degree: usize) -> Vec<f64> {
    let mut ret: Vec<f64> = Vec::new();
    for i in 0..degree {
        ret.push(knot(i as f64, degree as f64))
    }
    ret
}

fn divided_differences(size: usize, knots: &Vec<f64>, applied: &Vec<f64>) -> Vec<f64> {
    let mut ret = vec![vec![0.0; size]; size];

    for i in 0..size {
        ret[i][0] = applied[i];
    }

    for k in 1..size {
        for i in 0..(size - k) {
            ret[i][k] = ret[i + 1][k - 1] - ret[i][k - 1] / (knots[i + k] - knots[i]);
        }
    }
    ret.iter().map(|v| v[0]).collect()
}

fn poly(diffs: &Vec<f64>, knots: &Vec<f64>, knot: f64) -> f64 {
    let mut res = diffs[0];
    let mut product = 1.0;
    for i in 1..diffs.len() {
        product *= knot - knots[i];
        res += diffs[i] * product;
    }
    res
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("0.png", (1024, 1024)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("interpolation", ("sans-serif", 10).into_font())
        .margin(5)
        .x_label_area_size(10)
        .y_label_area_size(10)
        .build_cartesian_2d(-1f64..1f64, -1f64..1f64)?;

    chart.configure_mesh().draw()?;

    for i in 3..10 {
        let knots = knots(i);
        let applied = knots.iter().map(|elem: &f64| f(*elem)).collect();
        let diffs = divided_differences(knots.len(), &knots, &applied);
        chart
            .draw_series(LineSeries::new(
                (-100..100)
                    .map(|x| x as f64 / 100.0)
                    .map(|x| (x, (f(x) - poly(&diffs, &applied, x)).abs())),
                &RGBColor(20 * i as u8, 0, 0),
            ))?
            .label(format!("{}", i));
    }

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .draw()?;

    root.present()?;

    Ok(())
}
