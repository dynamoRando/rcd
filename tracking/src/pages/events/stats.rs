use chrono::{NaiveDate, Days, Duration};
use yew::prelude::*;

use crate::{storage::get_last_x_events, logging::log_to_console};


#[function_component]
pub fn Stats() -> Html { 

    let average = compute_average_days();

    log_to_console("stats");

    html!(
        <div>
            <p>{"Average is: "}{average.to_string()}{ " days between periods."}</p>
        </div>
    )
}

fn compute_average_days() -> u32 {
    let events = get_last_x_events(6);

    let mut dates: Vec<NaiveDate> = Vec::new();

    for event in events {
        dates.push(event.date().unwrap());
    }

    dates.sort();
    dates.reverse();

    let mut days: Vec<u32> = Vec::new();

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

            days.push(num_days as u32);
        }
    }

    let message = format!("{days:?}");
    log_to_console(&message);

    let mut sum:u32  = 0;
    
    
    for day in &days {
        sum += day;
    } 

    sum / days.len() as u32
}