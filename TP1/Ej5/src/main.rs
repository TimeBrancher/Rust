use::std::io::stdin;
fn main() {
    let mut cadena = "Hola Mundo".to_string();
    println!("Ingrese el string a concatenar: " );
    let mut nombre = String::new();
    stdin().read_line(&mut nombre).expect("Error al leer el nombre." );
    println!("La concatenacion es {}", cadena+&nombre);
}
