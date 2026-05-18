use::std::io::stdin;
fn main() {
    let num:u32 = 32;
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Error al leer dato");
    let proc:u32 = input.trim().parse().expect("eso no es un numero");
    println!("{}",num+proc);
}
