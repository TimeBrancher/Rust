fn cantidad_de_cadenas_mayor_a ( arreglo:[&str; 5],lim:usize)->i32{
	let mut total = 0;
	for i in 0..5{
		if arreglo[i].len() > lim{
		total = total + 1;}
}
	total
}
#[test]	
fn test_mayor() {
   let arreglo:[&str; 5] = ["hola","holaa","holaaa","holaaaa","1"];
   assert_eq!(cantidad_de_cadenas_mayor_a(arreglo,5),2);
}
#[test]
fn test_vacio(){
   let arreglo = ["a";5];
   assert_eq!(cantidad_de_cadenas_mayor_a(arreglo,3),0);
}
#[should_panic="arreglo pequeño"]
#[test]
fn test_insuficiente(){
	let arreglo = ["a";4];
	if arreglo.len()!=5{
		panic!("arreglo pequeño");
	}
}
