use std::collections::HashMap;
extern crate time;

const RES_BEFORE: i32 = 2;

fn fuzzy_hour(time: time::Tm) -> String {
    let translate: HashMap<i32, &str> = 
       [(1, "Eins"),
        (2, "Zwei"),
        (3, "Drei"),
        (4, "Vier"),
        (5, "Fünf"),
        (6, "Sechs"),
        (7, "Sieben"),
        (8, "Acht"),
        (9, "Neun"),
        (10, "Zehn"),
        (11, "Elf"),
        (0, "Zwölf")]
        .iter().cloned().collect();
    let mut hour = time.tm_hour;
    if time.tm_min + RES_BEFORE >= 25 {
        hour += 1
    }
    hour = hour % 12;
    return translate.get(&hour).unwrap().clone().into()
}

fn fuzzy_minute(time: time::Tm) -> String {
    let minute = time.tm_min - RES_BEFORE;
    let result = if minute <= 0 {"Uhr"}
        else if minute <= 5 {"Fünf nach"}
        else if minute <= 10 {"Zehn nach"}
        else if minute <= 15 {"Viertel nach"}
        else if minute <= 20 {"Zwanzig nach"}
        else if minute <= 25 {"Fünf vor halb"}
        else if minute <= 30 {"Halb"}
        else if minute <= 35 {"Fünf nach halb"}
        else if minute <= 40 {"Zwanzig vor"}
        else if minute <= 45 {"Viertel vor"}
        else if minute <= 50 {"Zehn vor"}
        else if minute <= 55 {"Fünf vor"}
        else {"Uhr"};
    return result.into();
}


fn main() {
    let now = time::now();
    let mut hour = fuzzy_hour(now);
    let minute = fuzzy_minute(now);
    if minute == "Uhr" {
        if hour == "Eins" { hour = "Ein".into(); }
        println!("{} {}", hour, minute);
    } else {
        println!("{} {}", minute, hour);
    }
}
