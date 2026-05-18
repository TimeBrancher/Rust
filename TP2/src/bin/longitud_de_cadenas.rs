fn longitud_de_cadenas ( arreglo:[&str; 5])->[usize;5]{
	let mut total = [0,0,0,0,0];
	for i in 0..5{
		total[i] = arreglo[i].len();
}
	total
}
#[test]	
fn test() {
   let arreglo:[&str; 5] = ["hola","tortolo","skibidi","WAZAAAAAA","pop"];
   assert_eq!(longitud_de_cadenas(arreglo),[4,7,7,9,3]);
   assert_neq!(lontidu_de_cadenas(arreglo),[3,6,6,8,2]);
}
