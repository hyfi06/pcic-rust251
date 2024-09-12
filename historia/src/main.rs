use csv::{ReaderBuilder, StringRecord};
use std::{collections::HashMap, fs};

const FILENAME: &str = "history.csv";

// Tipo, TAG, TEXTO, VIDA
#[derive(Debug)]

enum Tipo {
    SITUACION,
    OPCION,
    ERORR,
}
struct DatoHistoria {
    tipo_dato: String,
    tag: String,
    texto: String,
    vida: i32,
}

impl From<StringRecord> for DatoHistoria {
    fn from(value: StringRecord) -> Self {
        let mut iter = value.iter();
        Self {
            tipo_dato: iter.next().unwrap().trim().to_string(),
            tag: iter.next().unwrap().trim().to_string(),
            texto: iter.next().unwrap().trim().to_string(),
            vida: iter.next().unwrap().trim().parse().unwrap_or(0),
        }
    }
}

fn main() {
    let content = fs::read_to_string(FILENAME).unwrap();

    let mut rdr = ReaderBuilder::new()
        .delimiter(b';')
        .from_reader(content.as_bytes());

    let mut datos_historia = HashMap::new();
    for record in rdr.records() {
        let data = DatoHistoria::from(record.unwrap());
        datos_historia.insert(data.tag.clone(), data);
    }
    println!("{:?}", datos_historia);
}
