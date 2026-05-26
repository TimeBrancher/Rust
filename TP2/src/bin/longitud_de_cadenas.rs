fn longitud_de_cadenas ( arreglo:[&str; 5])->[usize;5]{
	let mut total = [0,0,0,0,0];
	for i in 0..5{
		total[i] = arreglo[i].len();
}
	total
}
#[test]	
fn test() {
   let arreglo:[&str; 5] = ["hola","holaa","holaaa","holaaaa","1"];
   assert_eq!(longitud_de_cadenas(arreglo),[4,5,6,7,1]);}
#[test]
fn test_false(){
   let arreglo:[&str; 5] = ["hola", "holaa", "holaaa", "holaaaa", "1"];
   assert_ne!(longitud_de_cadenas(arreglo),[4,5,6,7,2]);
}
#[should_panic="Error de tamaño"]
#[test]
fn test_error(){
	let arreglo:[&str;4]=["o";4];
	if arreglo.len()!=5{
		panic!("Error de tamaño");
	}
}
