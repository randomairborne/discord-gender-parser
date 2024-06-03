use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::io::{BufRead, BufReader, BufWriter, Write};

macro_rules! max_by_field {
    ($data:ident, $field:ident) => {
        $data
            .iter()
            .max_by(|a, b| a.$field.total_cmp(&b.$field))
            .unwrap()
    };
}

macro_rules! min_by_field {
    ($data:ident, $field:ident) => {
        $data
            .iter()
            .min_by(|a, b| a.$field.total_cmp(&b.$field))
            .unwrap()
    };
}

fn main() {
    let path = std::env::args().nth(1).expect("Expected argument");
    let input = std::fs::OpenOptions::new()
        .read(true)
        .open(path)
        .expect("Failed to open input");
    let buf = BufReader::new(input);
    let mut data = Vec::with_capacity(1024);
    for line in buf.lines() {
        let line = line.unwrap();
        if let Ok(datum) = serde_json::from_str::<DiscordDatum>(&line) {
            data.push(datum);
        }
    }

    data.sort_by(|a, b| a.day_pt.cmp(&b.day_pt));

    let output = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("output.json")
        .expect("Failed to open output");
    let mut output_buf = BufWriter::new(output);
    serde_json::to_writer_pretty(&mut output_buf, &data).unwrap();
    output_buf.flush().unwrap();

    let greatest_male = max_by_field!(data, prob_male);
    let greatest_nb = max_by_field!(data, prob_nb);
    let greatest_female = max_by_field!(data, prob_female);
    let least_male = min_by_field!(data, prob_male);
    let least_nb = min_by_field!(data, prob_nb);
    let least_female = min_by_field!(data, prob_female);

    println!(
        "greatest nb: {} (at {})",
        greatest_nb.prob_nb, greatest_nb.day_pt
    );
    println!(
        "greatest male: {} (at {})",
        greatest_male.prob_male, greatest_male.day_pt
    );
    println!(
        "greatest female: {} (at {})",
        greatest_female.prob_female, greatest_female.day_pt
    );
    println!("least nb: {} (at {})", least_nb.prob_nb, least_nb.day_pt);
    println!(
        "least male: {} (at {})",
        least_male.prob_male, least_male.day_pt
    );
    println!(
        "least female: {} (at {})",
        least_female.prob_female, least_female.day_pt
    );
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct DiscordDatum {
    predicted_gender: String,
    probability: f64,
    prob_male: f64,
    prob_female: f64,
    #[serde(rename = "prob_non_binary_gender_expansive")]
    prob_nb: f64,
    day_pt: DateTime<Utc>,
    model_version: DateTime<Utc>,
}
