fn main() {
    let tupla = ("Hola Mundo",[1,2,3,4,5]);
    let mut tot = 0;
    for i in 0..5{
	tot = tot + tupla.1[i];
	}
    println!("{},{}",tupla.0,tot);
}
