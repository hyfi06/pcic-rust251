fn main() {
    println!("Por favor introduce tu nombre: ");
    let mut name: String = String::new();
    std::io::stdin().read_line(&mut name).unwrap();
    name = name.trim().to_string();

    println!("Por favor introduce tu edad: ");
    let mut edad: String = String::new();
    std::io::stdin().read_line(&mut edad).unwrap();
    let edad_int: u8 = edad.trim().parse().unwrap();

    println!("Por favor introduce tu país: ");
    let mut pais: String = String::new();
    std::io::stdin().read_line(&mut pais).unwrap();
    pais = pais.trim().to_string();

    println!(
        "Hola, bienvenido o bienvenida {} de {} años, proveniente de {}",
        name, edad_int, pais
    );
}
