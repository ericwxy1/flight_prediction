mod data_preparation;
mod analysis;
mod model_planning;
use std::io::{self, Write};

const ANALYSIS_SYNONYMS: &[&str] = &["analysis", "stats", "statistics", "insight", "report"];
const AIRPORT_SYNONYMS: &[&str] = &["airport", "apt", "airfield"];
const CARRIER_SYNONYMS: &[&str] = &["carrier", "airline", "airlines"];
const PREDICTION_SYNONYMS: &[&str] = &["prediction", "future", "forecast", "projection"];

#[derive(Debug, PartialEq)]
enum Intent {
    AnalysisAirport(String),
    AnalysisCarrier(String),
    PredictFuture(String),
    Unknown,
}

fn main(){
    let records = match data_preparation::load_data("International_Report_Departures.csv"){
        Ok(r) => r,
        Err(e) => {
            eprintln!("Error loading data: {}", e);
            std::process::exit(1);
        }
    };
    let cleaned = data_preparation::clean_data(records);
    println!("Loaded and cleaned {} records.", cleaned.len());

    println!("Please enter your request, using IATA code. For example:");
    println!("- 'I want to see analysis for JFK airport'");
    println!("- 'Show me a report of carrier AA'");
    println!("- 'I want to see airline DL future prediction'");
    println!("- 'Enter 'stop' to stop");
    loop{
        print!("> ");
        io::stdout().flush().unwrap();

        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).expect("Failed to read input");
        let user_input = user_input.trim().to_string();

        if user_input.to_lowercase() == "stop" {
            println!("Stopping the program as requested.");
            break;
        }

        handle_query(&user_input, &cleaned);
    }
}

fn handle_query(query: &str, records: &[data_preparation::FlightRecord]){
    match recognize_intent(query){
        Intent::AnalysisAirport(airport_code) => {
            println!("Analysis for {} airport", airport_code);
            let filtered = filter_records(records, |r|
                r.usg_apt.eq_ignore_ascii_case(&airport_code)
            );
            show_filtered_analysis(&format!("{} airport", airport_code), &filtered);
        },
        Intent::AnalysisCarrier(airline_code) => {
            println!("Analysis for carrier {}", airline_code);
            let filtered = filter_records(records, |r|
                r.carrier.eq_ignore_ascii_case(&airline_code)
            );
            show_filtered_analysis(&format!("airline {}", airline_code), &filtered);
        },
        Intent::PredictFuture(airline_code) => {
            println!("Predicting future trends for carrier {}", airline_code);
            let filtered = filter_records(records, |r|
                r.carrier.eq_ignore_ascii_case(&airline_code)
            );
            let historical = extract_monthly_aggregates(&filtered);
            model_planning::predict_future_trends(&historical);
        },
        Intent::Unknown => {
            println!("Sorry, I couldn't understand your request. Please try rephrasing.");
        },
    }
}

fn recognize_intent(query: &str) -> Intent{
    let query_lower = query.to_lowercase();
    let tokens: Vec<&str> = query_lower.split_whitespace()
        .map(|word| word.trim_matches(|c: char| !c.is_alphanumeric() && c != '-'))
        .collect();

    let has_analysis = tokens.iter().any(|word| ANALYSIS_SYNONYMS.contains(word));
    let has_prediction = tokens.iter().any(|word| PREDICTION_SYNONYMS.contains(word));

    if has_prediction {
        for i in 0..tokens.len(){
            if CARRIER_SYNONYMS.contains(&tokens[i]) && i + 1 < tokens.len(){
                let carrier_code = tokens[i + 1].to_uppercase();
                if (carrier_code.len() == 2 || carrier_code.len() == 3) && carrier_code.chars().all(|c| c.is_alphanumeric()) {
                    return Intent::PredictFuture(carrier_code);
                }
            }
        }
    }

    if !has_analysis && !has_prediction{
        return Intent::Unknown;
    }

    let airport_code = find_first_code(&tokens, AIRPORT_SYNONYMS, |code| {
        code.len() == 3 && code.chars().all(|c| c.is_alphabetic())
    });

    let carrier_code = find_first_code(&tokens, CARRIER_SYNONYMS, |code| {
        (code.len() == 2 || code.len() == 3) && code.chars().all(|c| c.is_alphanumeric())
    });

    if let Some(code) = airport_code{
        Intent::AnalysisAirport(code)
    }
    else if let Some(code) = carrier_code{
        Intent::AnalysisCarrier(code)
    }
    else{
        Intent::Unknown
    }
}

fn find_first_code(tokens: &[&str], synonyms: &[&str], validation: impl Fn(&str) -> bool) -> Option<String>{
    for (i, &word) in tokens.iter().enumerate(){
        if synonyms.contains(&word) {
            if let Some(code) = extract_code(tokens, i, &validation) {
                return Some(code);
            }
        }
    }
    None
}

fn extract_code(tokens: &[&str], synonym_index: usize, validation: impl Fn(&str) -> bool) -> Option<String>{
    if synonym_index + 1 < tokens.len(){
        let potential_code = tokens[synonym_index + 1];
        if validation(potential_code) {
            return Some(potential_code.to_uppercase());
        }
    }
    if synonym_index >= 1{
        let potential_code = tokens[synonym_index - 1];
        if validation(potential_code) {
            return Some(potential_code.to_uppercase());
        }
    }
    None
}

fn extract_monthly_aggregates(filtered: &[&data_preparation::FlightRecord]) -> Vec<analysis::MonthlyAggregate> {
    let owned_records: Vec<data_preparation::FlightRecord> = filtered.iter().map(|r| (*r).clone()).collect();
    analysis::aggregate_by_month(&owned_records)
}


fn filter_records<'a, F>(records: &'a [data_preparation::FlightRecord], predicate: F) -> Vec<&'a data_preparation::FlightRecord>
where
    F: Fn(&data_preparation::FlightRecord) -> bool,
{
    records.iter().filter(|r| predicate(r)).collect()
}

fn show_filtered_analysis(label: &str, filtered: &[&data_preparation::FlightRecord]) {
    if filtered.is_empty(){
        println!("No data found for {}.", label);
        return;
    }
    let owned: Vec<data_preparation::FlightRecord> = filtered.iter().map(|r| (*r).clone()).collect();
    let monthly_aggregates = analysis::aggregate_by_month(&owned);
    analysis::plot_monthly_aggregation(&monthly_aggregates);
    println!("(Top foreign airports related to {}):", label);
    analysis::find_top_foreign_airports(&owned, 5);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recognize_intent_analysis_airport_after_synonym() {
        let query = "I want to see analysis for JFK airport";
        let intent = recognize_intent(query);
        assert_eq!(intent, Intent::AnalysisAirport("JFK".to_string()));
    }

    #[test]
    fn test_recognize_intent_analysis_airport_before_synonym() {
        let query = "Show me the report of LAX apt";
        let intent = recognize_intent(query);
        assert_eq!(intent, Intent::AnalysisAirport("LAX".to_string()));
    }

    #[test]
    fn test_recognize_intent_analysis_carrier_before_synonym() {
        let query = "Report of airline DL";
        let intent = recognize_intent(query);
        assert_eq!(intent, Intent::AnalysisCarrier("DL".to_string()));
    }

    #[test]
    fn test_recognize_intent_analysis_carrier_after_synonym() {
        let query = "Show me a report of carrier AA";
        let intent = recognize_intent(query);
        assert_eq!(intent, Intent::AnalysisCarrier("AA".to_string()));
    }

    #[test]
    fn test_recognize_intent_airport_code_with_non_alpha() {
        let query = "Provide statistics for JFK1 airport";
        let intent = recognize_intent(query);
        assert_eq!(intent, Intent::Unknown);
    }

    #[test]
    fn test_recognize_intent_lowercase_input() {
        let query = "analysis for jfk airport";
        let intent = recognize_intent(query);
        assert_eq!(intent, Intent::AnalysisAirport("JFK".to_string()));
    }

    #[test]
    fn test_recognize_intent_predict_future_standard() {
        let query = "I want to see airline AA future prediction";
        let intent = recognize_intent(query);
        assert_eq!(intent, Intent::PredictFuture("AA".to_string()));
    }

    #[test]
    fn test_recognize_intent_predict_future_missing_airline_code() {
        let query = "I want to see future prediction";
        let intent = recognize_intent(query);
        assert_eq!(intent, Intent::Unknown);
    }
}
