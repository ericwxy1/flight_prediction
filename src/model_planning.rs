use crate::analysis::MonthlyAggregate;
use linfa::prelude::*;
use linfa_linear::LinearRegression;
use ndarray::Array;


pub fn predict_future_trends(historical: &[MonthlyAggregate]) {
    if historical.is_empty() {
        println!("No historical data available for predictions.");
        return;
    }

    let start_year = historical.first().unwrap().year;
    let start_month = historical.first().unwrap().month;
    let months_from_start = |year: u16, month: u8| -> f64 {
        (year as f64 - start_year as f64) * 12.0 + (month as f64 - start_month as f64)
    };

    let xs: Vec<f64> = historical.iter().map(|r| months_from_start(r.year, r.month)).collect();
    let ys: Vec<f64> = historical.iter().map(|r| r.total_flights as f64).collect();
    
    let n = xs.len(); // store length first
    let x_array = Array::from_iter(xs).into_shape((n, 1)).unwrap();
    let y_array = Array::from_iter(ys);    
    let dataset = linfa::Dataset::new(x_array, y_array);

    let model = LinearRegression::new().fit(&dataset).expect("Could not fit linear model");

    println!("Model fitted. Intercept: {}, Coefficients: {:?}", model.intercept(), model.params());

    let last = historical.last().unwrap();
    let last_idx = months_from_start(last.year, last.month);
    let last_actual = last.total_flights as f64;

    let future_horizon = 6;
    println!("Predicting the next {} months...", future_horizon);

    let mut future_predictions = Vec::new();
    for i in 1..=future_horizon {
        let future_idx = Array::from_elem((1,1), last_idx + i as f64);
        let prediction = model.predict(&future_idx);
        let predicted_value = prediction[0];
        future_predictions.push(predicted_value);
        println!("Month {} after last data: Predicted total flights = {:.2}", i, predicted_value);
    }
}
