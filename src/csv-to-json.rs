use std::fs;
use serde::Serialize;

#[derive(Serialize, Debug)]
struct CistiRecord {
    satir: Option<String>,
    dosya_id: Option<String>,
    dosya_no: Option<String>,
    dys_dosya_id: Option<String>,
}

#[derive(Serialize, Debug)]
struct DysRecord {
    satir: Option<String>,
    dosya_id: Option<String>,
    dosya_no: Option<String>,
    dosya_tur_kod: Option<String>,
}

fn check(col: &str) -> Option<String> {
    let trimmed = col.trim();
    if trimmed.is_empty() || trimmed.eq_ignore_ascii_case("null") {
        None
    } else {
        Some(trimmed.to_string())
    }
}

fn main() -> std::io::Result<()> {
    let cisti_content = fs::read_to_string("/Users/alankara/ws/rusttimedemo/src/cisti.csv")?;
    let dys_content = fs::read_to_string("/Users/alankara/ws/rusttimedemo/src/dys.csv")?;

    let cisti: Vec<CistiRecord> = cisti_content
        .lines()
        .map(|line| {
            let cols: Vec<_> = line.split('\t').map(check).collect();
            CistiRecord {
                satir: cols.get(0).cloned().unwrap_or(None),
                dosya_id: cols.get(1).cloned().unwrap_or(None),
                dosya_no: cols.get(2).cloned().unwrap_or(None),
                dys_dosya_id: cols.get(3).cloned().unwrap_or(None),
            }
        })
        .collect();

    let dys: Vec<DysRecord> = dys_content
        .lines()
        .map(|line| {
            let cols: Vec<_> = line.split('\t').map(check).collect();
            DysRecord {
                satir: cols.get(0).cloned().unwrap_or(None),
                dosya_id: cols.get(1).cloned().unwrap_or(None),
                dosya_no: cols.get(2).cloned().unwrap_or(None),
                dosya_tur_kod: cols.get(3).cloned().unwrap_or(None),
            }
        })
        .collect();

    let cisti_count = cisti.len();
    let dys_count = dys.len();

    println!("cistiCount: {}", cisti_count);
    println!("dysCount: {}", dys_count);

    let cisti_json = serde_json::to_string_pretty(&cisti)?;
    fs::write("./cisti.json", cisti_json)?;

    let dys_json = serde_json::to_string_pretty(&dys)?;
    fs::write("./dys.json", dys_json)?;

    Ok(())
}
