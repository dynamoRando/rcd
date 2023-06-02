use chrono::{NaiveDate, Days, Duration};
use yew::prelude::*;

use crate::{storage::get_last_x_events, logging::log_to_console};


#[function_component]
pub fn Stats() -> Html { 
    log_to_console("stats");
    let average = compute_average_days();
    let next = next_start(average);

    let average_formatted = format!("{:.2}", average);
    
    html!(
        <div>
            <p>{"Average is: "}<b>{average_formatted}</b>{ " days between periods."}</p>
            <p>{"Your projected next start is: "}<b>{next.to_string()}</b></p>
        </div>
    )
}

fn next_start(avg: f32) -> NaiveDate {
    let events = get_last_x_events(6);
    
    let mut dates: Vec<NaiveDate> = Vec::new();

    for event in events {
        dates.push(event.date().unwrap());
    }

    dates.sort();
    dates.reverse();


    let max_date = dates.first().unwrap().clone();
    let next_date = max_date + Duration::days(avg.round() as i64);
    log_to_console(&next_date.to_string());
    next_date
}

fn compute_average_days() -> f32 {
    let events = get_last_x_events(6);

    let mut dates: Vec<NaiveDate> = Vec::new();

    for event in events {
        dates.push(event.date().unwrap());
    }

    dates.sort();
    dates.reverse();

    let mut days: Vec<f32> = Vec::new();

    let num_periods = dates.len();

    for i in 0..dates.len() {
        if i + 1 < num_periods {
            let a = dates[i];

            let a_revised = a + Duration::days(-1);

            let message = format!("a_revised: {a_revised:?}");
            log_to_console(&message);


            let b = dates[i + 1];

            let message = format!("b: {b:?}");
            log_to_console(&message);

            let duration = a_revised - b;

            let message = format!("duration: {duration:?}");
            log_to_console(&message);

            let num_days = duration.num_days();

            let message = format!("num_days: {num_days:?}");
            log_to_console(&message);

            days.push(num_days as f32);
        }
    }

    let message = format!("{days:?}");
    log_to_console(&message);

    let mut sum: f32 = 0.0;
    
    
    for day in &days {
        sum += day;
    } 

    sum / days.len() as f32
}