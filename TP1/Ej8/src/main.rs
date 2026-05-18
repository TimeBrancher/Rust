use::std::io::stdin;
fn main() {
    const CADENA:&str = "Pip Pop";
    let mut rep = 0;
    println!("Ingresa un caracter:");
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Error al leer el nombre." ); 
    let caracter = input.trim().parse().expect("eso no es un caracter");
    for c in CADENA.chars() {
        if c == caracter{ rep = rep+1;}
    }
    println!("Se repite {} veces",rep);

}
