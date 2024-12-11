use std::collections::HashMap;
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

    let mut results: Vec<MonthlyAggregate> = map
        .into_iter()
        .map(|((year, month), total_flights)| MonthlyAggregate { year, month, total_flights })
        .collect();
    results.sort_by(|a, b| a.year.cmp(&b.year).then(a.month.cmp(&b.month)));
    results
}

pub fn print_monthly_aggregation(aggregates: &[MonthlyAggregate]) {
    println!("Year,Month,Total_Flights");
    for a in aggregates {
        println!("{},{},{}", a.year, a.month, a.total_flights);
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
