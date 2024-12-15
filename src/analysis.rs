use std::collections::HashMap;
use plotters::prelude::*;
use crate::data_preparation::FlightRecord;

pub struct MonthlyAggregate {
    pub year: u16,
    pub month: u8,
    pub total_flights: u32,
}

pub fn aggregate_by_month(records: &[FlightRecord]) -> Vec<MonthlyAggregate> {
    let mut map: HashMap<(u16, u8), u32> = HashMap::new();
    for r in records {
        *map.entry((r.year, r.month)).or_insert(0) += r.total;
    }

    let mut results: Vec<MonthlyAggregate> = map.into_iter()
        .map(|((year, month), total_flights)| MonthlyAggregate { year, month, total_flights }).collect();
    results.sort_by(|a, b| a.year.cmp(&b.year).then(a.month.cmp(&b.month)));
    results
}

pub fn plot_monthly_aggregation(aggregates: &[MonthlyAggregate]) -> Result<(), Box<dyn std::error::Error>> {
    if aggregates.is_empty() {
        println!("No data to plot.");
        return Ok(());
    }
    let month_labels: Vec<String> = aggregates.iter()
        .map(|a| format!("{}-{:02}", a.year, a.month)).collect();

    let root = BitMapBackend::new("monthly_aggregation.png", (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let max_flights = aggregates.iter().map(|a| a.total_flights).max().unwrap_or(0);
    let mut chart = ChartBuilder::on(&root)
        .caption("Monthly Aggregation of Flights", ("sans-serif", 20))
        .margin(10)
        .x_label_area_size(50)
        .y_label_area_size(50)
        .build_cartesian_2d(
            0..(aggregates.len() as i32 - 1),
            0..(max_flights as i32 + 10),
        )?;

    chart.configure_mesh()
        .x_desc("Month")
        .y_desc("Total Flights")
        .x_label_formatter(&|x| {
            let idx = *x as usize;
            if idx < month_labels.len() {
                month_labels[idx].clone()
            } else {
                "".to_string()
            }
        })
        .draw()?;

    let series = aggregates.iter().enumerate().map(|(i, a)| (i as i32, a.total_flights as i32));
    chart.draw_series(LineSeries::new(series, &RED))?
         .label("Flights")
         .legend(|(x, y)| PathElement::new(vec![(x, y), (x+20, y)], &RED));

    chart.configure_series_labels()
         .border_style(&BLACK)
         .draw()?;

    println!("Chart generated: monthly_aggregation.png");
    Ok(())
}

pub fn show_monthly_aggregation_chart(aggregates: &[MonthlyAggregate]) {
    match plot_monthly_aggregation(aggregates) {
        Ok(_) => println!("Chart generated: monthly_aggregation.png"),
        Err(e) => eprintln!("Error generating chart: {}", e),
    }
}

pub fn find_top_foreign_airports(records: &[FlightRecord], top_n: usize) {
    let mut map: HashMap<&str, u32> = HashMap::new();
    for r in records {
        *map.entry(&r.fg_apt).or_insert(0) += r.total;
    }

    let mut counts: Vec<(&str, u32)> = map.into_iter().collect();
    counts.sort_by(|a, b| b.1.cmp(&a.1));
    println!("Top {} foreign airports:", top_n);
    for (apt, count) in counts.iter().take(top_n) {
        println!("{}: {}", apt, count);
    }
}
