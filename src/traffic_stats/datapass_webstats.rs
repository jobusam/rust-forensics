use chrono::{Datelike, Utc};
use select::document::Document;
use select::predicate::Name;

pub fn check_data_limit() {
    match get_datapass_de_website() {
        Ok(content) => {
            extract_data_from_html(content);
        }
        Err(e) => println!("Can't access content from datapass.de : {}", e),
    }
}

fn get_datapass_de_website() -> Result<String, ureq::Error> {
    println!("Call datapass.de");
    let body: String = ureq::get("https://datapass.de/home?continue=true")
        .call()?
        .into_string()?;
    Ok(body)
}

fn extract_data_from_html(content: String) {
    // clone the date because it's not clear if the Document class itself
    // changes the data?
    // TODO: read about ownership in rust book
    extract_amount(content.clone());
    extract_duration(content.clone());
    extract_last_update(content.clone());
}

fn extract_last_update(content: String) {
    Document::from(content.as_str())
        .find(Name("section"))
        .filter(|n| n.attr("class").unwrap_or_default().contains("last-update"))
        .for_each(|x| {
            println!(
                "Last update of data amount: {}",
                x.text()
                    .split(".")
                    .last()
                    .unwrap_or("")
                    .replace("um", "at")
                    .replace("Uhr", "")
            )
        })
}

fn extract_duration(content: String) {
    Document::from(content.as_str())
        .find(Name("div"))
        .filter(|n| {
            n.attr("class")
                .unwrap_or_default()
                .contains("remaining-duration ")
        })
        .for_each(|x| {
            println!(
                "Remaining duration: {}",
                x.text()
                    .split("für")
                    .last()
                    .unwrap_or("")
                    .replace("Tage", "days")
                    .replace("Tag", "day")
                    .replace("Std.", "hours")
            )
        })
}

fn extract_amount(content: String) {
    Document::from(content.as_str())
        .find(Name("div"))
        .filter(|n| n.attr("class").unwrap_or_default().contains("volume "))
        .for_each(|x| {
            let volume = x.first_child().map_or(String::from(""), |c| c.inner_html());
            if volume.len() > 2 {
                // remove ending "/" char and convert , to . (due to us locale)
                let stripped_volume = &volume[..volume.len() - 1].replace(',', ".");
                match stripped_volume.parse::<f64>() {
                    Ok(num) => print_data_limit_and_stats(num),
                    Err(_) => println!("Can't extract lte volume constraints (invalid format)"),
                };
            } else {
                println!("Can't extract lte volume constraints (extracted content to small)")
            }
        });
}

fn print_data_limit_and_stats(available_amount: f64) {
    println!("{:.2} GB of 60 GB are available", available_amount);
    let used_amount = 60.0 - available_amount;
    println!("{:.2} GB are already consumed", used_amount);
    // how many days until end of month?
    let today = Utc::now().date().naive_local();
    let first_day_of_next_month = match today.month() {
        12 => today
            .with_day(1)
            .expect("Error using day of next month")
            .with_month(1)
            .expect("Error user first month")
            .with_year(today.year() + 1)
            .expect("Error creating increasing year"),
        _ => today
            .with_day(1)
            .expect("Error using day of next month")
            .with_month(today.month() + 1)
            .expect("Error increasing month"),
    };
    let available_days_until_end_of_month = first_day_of_next_month
        .signed_duration_since(today)
        .num_days() as f64;
    println!(
        "Today: {}, end of month: {} with days {}",
        today, first_day_of_next_month, available_days_until_end_of_month
    );
    println!(
        "Maximum average amount per day should be {:.2}",
        available_amount / available_days_until_end_of_month
    );
}
