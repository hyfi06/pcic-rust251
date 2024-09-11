fn main() {
    println!("Por favor introduce tu nombre: ");
    let mut name: String = String::new();
    std::io::stdin().read_line(&mut name).unwrap();
    name = name.trim().to_string();
    println!("Por favor introduce tu edad: ");
    let mut edad: String = String::new();
    std::io::stdin().read_line(&mut edad).unwrap();
    let edad_int: u8 = edad.trim().parse().unwrap();
    println!("Hola, bienvenido o bienvenida {} de {} años", name, edad_int);
}
