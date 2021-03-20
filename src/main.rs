use std::io;
use std::error::Error;
use std::process;
use serde::Deserialize;
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Record {
    buchungstag: String,
    wertstellung: String,
    buchungstext: String,
    #[serde(rename = "Auftraggeber / Begunstigter")]
    auftraggeber: String,
    verwendungszweck: String,
    kontonummer: String,
    #[serde(rename = "BLZ")]
    blz: String,
    betrag: String,
    #[serde(rename = "Glaubiger-ID")]
    glaubiger: String,
    mandatsreferenz: String,
    kundenreferenz: String,
}

fn run() -> Result<(), Box<dyn Error>> {
    let mut reader = csv::ReaderBuilder::new()
        .delimiter(b';')
        .flexible(true)
        .from_reader(io::stdin());

    for result in reader.deserialize() {
        let record: Record = result?;
    }
    
    Ok(())
}


fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }   
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;
    use csv::StringRecord;

    #[test]
    fn test() {
        let exm = r#" "Buchungstag";"Wertstellung";"Buchungstext";"Auftraggeber / Begünstigter";"Verwendungszweck";"Kontonummer";"BLZ";"Betrag (EUR)";"Gläubiger-ID";"Mandatsreferenz";"Kundenreferenz";
        "08.02.2019";"08.02.2019";"Gutschrift";"Test Full Name";"test";"DE5511199009900990099";"DEUTDEDBBER";"1,00";"";"";"test";
        "11.02.2019";"11.02.2019";"Umbuchung";"KREDITKARTEN GELDANLAGE";"Test Full Name";"1999888";"00000000";"15,00";"";"";"";
        "#;

        let exm = Cursor::new(exm);
        let mut reader = csv::ReaderBuilder::new()
        .delimiter(b';')
        .flexible(true)
        .from_reader(exm);

        let mut record = StringRecord::new();
        assert!(reader.read_record(&mut record).unwrap());
        assert_eq!(record.len(), 12);
    }
}
