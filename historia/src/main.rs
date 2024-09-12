use csv::{ReaderBuilder, StringRecord};
use std::{collections::HashMap, fs};

const FILENAME: &str = "history.csv";
const FIRST_TAG: &str = "INICIO";

// Tipo, TAG, TEXTO, VIDA
#[derive(Debug)]

struct DatoHistoria {
    tipo_dato: String,
    tag: String,
    texto: String,
    vida: i32,
    opciones: Vec<DatoHistoria>,
}

impl From<StringRecord> for DatoHistoria {
    fn from(value: StringRecord) -> Self {
        let mut iter = value.iter();
        Self {
            tipo_dato: iter.next().unwrap().trim().to_string(),
            tag: iter.next().unwrap().trim().to_string(),
            texto: iter.next().unwrap().trim().to_string(),
            vida: iter.next().unwrap().trim().parse().unwrap_or(0),
            opciones: Vec::new(),
        }
    }
}

fn main() {
    let mut vida = 100;
    let mut tag_actual = FIRST_TAG;
    let mut last_record: String = "".to_string();

    let mut datos_historia = HashMap::new();

    let content = fs::read_to_string(FILENAME).unwrap();
    let mut rdr = ReaderBuilder::new()
        .delimiter(b';')
        .from_reader(content.as_bytes());

    for record in rdr.records() {
        let data = DatoHistoria::from(record.unwrap());
        if data.tipo_dato == "SITUACION" {
            last_record = data.tag.clone();
            datos_historia.insert(last_record.clone(), data);
        } else {
            if let Some(situation) = datos_historia.get_mut(&last_record) {
                (*situation).opciones.push(data);
            }
        }
    }

    // Game Loop
    loop {
        let data = datos_historia.get(tag_actual).unwrap();
        vida += data.vida;
        println!("Tienes {} de vida", vida);
        println!("{}", data.texto);

        if vida <= 0 {
            println!("Game over");
            break;
        }

        for (idx, option) in data.opciones.iter().enumerate() {
            println!("[{}] {}", idx, option.texto);
        }
        let mut selection = String::new();
        std::io::stdin().read_line(&mut selection).unwrap();
        let selection: usize = selection.trim().parse().unwrap_or(99);
        if let Some(eleccion) = data.opciones.get(selection) {
            tag_actual = &eleccion.tag;
        } else {
            println!("Comando no válido");
        }
        println!("");
    }
}
