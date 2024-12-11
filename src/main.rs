mod data_preparation;
mod analysis;

fn main() {
    let records = match data_preparation::load_data("International_Report_Departures.csv") {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Error loading data: {}", e);
            std::process::exit(1);
        }
    };
    let cleaned = data_preparation::clean_data(records);
    println!("Loaded and cleaned {} records.", cleaned.len());
    let monthly_aggregates = analysis::aggregate_by_month(&cleaned);
    analysis::print_monthly_aggregation(&monthly_aggregates);
    analysis::find_top_foreign_airports(&cleaned, 5);
}
