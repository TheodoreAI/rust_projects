use plotters::prelude::*;
use std::error::Error;
use csv::ReaderBuilder;

pub fn plot_csv_data(_x_data: &[f64], _y_data: &[f64]) -> Result<(), Box<dyn Error>>{
    // Read the CSV file (assuming it has two columns, x and y)
    let csv_path = "fibonacci.csv";
    let mut rdr = ReaderBuilder::new().has_headers(false).from_path(csv_path)?;

    // Collect the data from CSV into two Vecs: _x_data and _y_data
    let (_x_data, _y_data): (Vec<f64>, Vec<f64>) = rdr.records()
        .filter_map(|result| {
            let record = result.ok()?;
            let x: f64 = record[0].parse().ok()?;
            let y: f64 = record[1].parse().ok()?;
            Some((x, y))
        })
        .unzip();

    // Create the plot and draw the data on it
    let root = BitMapBackend::new("plot.png", (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;
    let root = root.margin(10, 10, 10, 10);

    let mut chart = ChartBuilder::on(&root)
        .caption("CSV Data Plot", ("sans-serif", 30))
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(0f64..10f64, 0f64..100f64)?;

    chart.configure_mesh().draw()?;

    chart.draw_series(LineSeries::new(_x_data.iter().zip(_y_data.iter()).map(|(x, y)| (*x, *y)), &BLUE))?;

    Ok(())
}
