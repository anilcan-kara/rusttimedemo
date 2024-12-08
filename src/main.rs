use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

#[derive(Serialize, Deserialize, Clone)]
struct CistiRecord {
    satir: Option<String>,
    dosya_id: Option<String>,
    dosya_no: Option<String>,
    dys_dosya_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    dys_dosyalar: Option<Vec<DysRecord>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dys_dosya_count: Option<usize>,
}

#[derive(Serialize, Deserialize, Clone)]
struct DysRecord {
    satir: Option<String>,
    dosya_id: Option<String>,
    dosya_no: Option<String>,
    dosya_tur_kod: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    cisti_dosyalar: Option<Vec<CistiRecord>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cisti_dosya_count: Option<usize>,
}

fn main() -> std::io::Result<()> {
    let cisti_data = fs::read_to_string("cisti.json")?;
    let dys_data = fs::read_to_string("dys.json")?;

    let mut cisti: Vec<CistiRecord> = serde_json::from_str(&cisti_data).expect("Invalid cisti JSON");
    let mut dys: Vec<DysRecord> = serde_json::from_str(&dys_data).expect("Invalid dys JSON");

    let mut cisti_map: HashMap<String, Vec<usize>> = HashMap::new();
    for (i, c) in cisti.iter().enumerate() {
        if let Some(dosya_no) = &c.dosya_no {
            cisti_map.entry(dosya_no.clone()).or_default().push(i);
        }
    }

    let mut dys_map: HashMap<String, Vec<usize>> = HashMap::new();
    for (i, d) in dys.iter().enumerate() {
        if let Some(dosya_no) = &d.dosya_no {
            dys_map.entry(dosya_no.clone()).or_default().push(i);
        }
    }

    for cisti_record in &mut cisti {
        if let Some(dosya_no) = &cisti_record.dosya_no {
            if let Some(dys_indexes) = dys_map.get(dosya_no) {
                let related: Vec<DysRecord> = dys_indexes.iter().map(|&idx| dys[idx].clone()).collect();
                cisti_record.dys_dosyalar = Some(related.clone());
                cisti_record.dys_dosya_count = Some(related.len());
            } else {
                cisti_record.dys_dosyalar = Some(vec![]);
                cisti_record.dys_dosya_count = Some(0);
            }
        } else {
            cisti_record.dys_dosyalar = Some(vec![]);
            cisti_record.dys_dosya_count = Some(0);
        }
    }

    for dys_record in &mut dys {
        if let Some(dosya_no) = &dys_record.dosya_no {
            if let Some(cisti_indexes) = cisti_map.get(dosya_no) {
                let related: Vec<CistiRecord> = cisti_indexes.iter().map(|&idx| cisti[idx].clone()).collect();
                dys_record.cisti_dosyalar = Some(related.clone());
                dys_record.cisti_dosya_count = Some(related.len());
            } else {
                dys_record.cisti_dosyalar = Some(vec![]);
                dys_record.cisti_dosya_count = Some(0);
            }
        } else {
            dys_record.cisti_dosyalar = Some(vec![]);
            dys_record.cisti_dosya_count = Some(0);
        }
    }

    println!("Total Cisti: {}", cisti.len());

    println!("Total DYS: {}", dys.len());

    println!(
        "Cisti Count with 0 DYS: {}",
        cisti.iter().filter(|c| c.dys_dosya_count == Some(0)).count()
    );

    println!(
        "Cisti Count with 1 DYS: {}",
        cisti.iter().filter(|c| c.dys_dosya_count == Some(1)).count()
    );

    println!(
        "Cisti Count with 2 DYS: {}",
        cisti.iter().filter(|c| c.dys_dosya_count == Some(2)).count()
    );

    println!(
        "Cisti Count with more then 3 DYS: {}",
        cisti.iter().filter(|c| c.dys_dosya_count.map_or(false, |count| count > 2)).count()
    );

    println!(
        "DYS Count with 0 Cisti: {}",
        dys.iter().filter(|d| d.cisti_dosya_count == Some(0)).count()
    );

    println!(
        "DYS Count with 1 Cisti: {}",
        dys.iter().filter(|d| d.cisti_dosya_count == Some(1)).count()
    );

    println!(
        "DYS Count with 2 Cisti: {}",
        dys.iter().filter(|d| d.cisti_dosya_count == Some(2)).count()
    );

    println!(
        "DYS Count with more then 3 Cisti: {}",
        dys.iter().filter(|d| d.cisti_dosya_count.map_or(false, |count| count > 2)).count()
    );

    let cisti_json = serde_json::to_string_pretty(&cisti).expect("Failed to serialize cisti");
    fs::write("cisti.json", cisti_json)?;

    let dys_json = serde_json::to_string_pretty(&dys).expect("Failed to serialize dys");
    fs::write("dys.json", dys_json)?;

    Ok(())
}
