
#![crate_type = "cdylib"]
use chrono::{Local, NaiveDate, Duration, Utc}; // 0.4.40

fn main(){
    let fecha = Local::now();
    let dat = NaiveDate::from_ymd_opt(2025, 03, 02).unwrap();
    let hoy = Utc::now();
    let mañana = hoy + Duration::days(1);
    let fec_str="2025-04-10";
    let fec_dat = NaiveDate::parse_from_str(fec_str, "%Y-%m-%d").unwrap();
    let pasa_man = fec_dat + Duration::weeks(2);
    
    println!("
    Local: {}\n{}\n
    hoy: {}\n
    mañana: {}\n
    fec-parse: {}\n
    fec-dossem: {}", fecha, dat, hoy, mañana, fec_dat, pasa_man);
}

