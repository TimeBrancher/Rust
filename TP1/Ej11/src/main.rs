use std::io::stdin;
fn main() {
    let arreglo = ["Pip","Pop","Pup","Pep","Pap"];
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Error al leer la linea");
    input = input.trim().to_string();
    for cadena in arreglo{
	if cadena==input{println!("Se encuentra en el arreglo");}
	}

}
