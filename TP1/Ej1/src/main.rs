use::std::io::stdin;
fn main() {
	let mut num = 3.0;
	println!("Ingrese un numero: " );
	let mut num2 = String::new();
	stdin().read_line(&mut num2).expect("Error al leer el nombre." );
	let conver:f64 = num2.trim().parse().expect("no es numero");
	println!("{}, {},{},{}", num+conver,num-conver,num/conver,num*conver);
}
