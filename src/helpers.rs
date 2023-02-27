/// Plot the frequency space of the microphone input using plotters.
#[cfg(feature = "plot")]
pub fn plot_frequency_space(frequency_space: &[(f32, f32)], title: &str, file_name: &str, x_min: f32, x_max: f32) {
    use plotters::prelude::*;

    let max = frequency_space.iter().map(|(_, d)| d).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    //let normalized_frequency_space = frequency_space.iter().map(|(f, m)| (f, m / max)).collect::<Vec<_>>();

    let file_name = format!("{}.png", file_name);
    let root = BitMapBackend::new(&file_name, (1920, 1080)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root)
        .caption(title, ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(x_min..x_max, 0f32..*max)
        .unwrap();

    chart.configure_mesh().draw().unwrap();

    chart.draw_series(LineSeries::new(frequency_space.iter().map(|(x, y)| (*x, *y)), RED)).unwrap();
}
