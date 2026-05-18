use std::io::stdin;
fn main() {
    let variable:bool = true;
    let mut entrada = String::new();
    stdin().read_line(&mut entrada).expect("Error al leer la línea");
    let lectura: bool = entrada.trim().parse().expect("Eso no es un número");
    println!("{},{}",variable||lectura,variable&&lectura);
}
