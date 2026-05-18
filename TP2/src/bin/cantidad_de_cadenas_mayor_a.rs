fn cantidad_de_cadenas_mayor_a ( arreglo:[&str; 5],inf:usize,sup:usize)->i32{
	let mut total = 0;
	for i in 0..5{
		if arreglo[i].len() > inf && arreglo[i].len() < sup{
		total = total + 1;}
}
	total
}
#[test]	
fn test() {
   let arreglo:[&str; 5] = ["hola","tortolo","skibidi","WAZAAAAAA","pop"];
   assert_eq!(cantidad_de_cadenas_mayor_a(arreglo,4,9),2);
   assert_neq!(cantidad_de_cadenas_mayor_a(arreglo,1,9),1);
}
