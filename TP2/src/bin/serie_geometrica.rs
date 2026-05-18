fn serie_geometrica ( data:i32)->[i32;5]{
	let mut arreglo:[i32;5] = [1,0,0,0,0];
	for i in 1..5{
		arreglo[i] = arreglo[i-1]*(data);
}
	arreglo
}
#[test]	
fn test() {
   let resultado = serie_geometrica(2);
   assert_eq!(resultado, [1,2,4,8,16]);
}
