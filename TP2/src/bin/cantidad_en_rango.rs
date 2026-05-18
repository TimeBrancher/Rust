fn cantidad_en_rango ( data:[i32;5],inf:i32,sup:i32)->i32{
	let mut cant = 0;
	for i in 0..5{
		if data[i]>=inf && data[i] <= sup{
		cant=cant+1;}
	}
	cant
}
#[test]
fn test() {
   let arreglo = [4,9,2,5,7];
   assert_eq!(cantidad_en_rango(arreglo,3,7),3);
   assert_eq!(cantidad_en_rango(arreglo,10,13),0);
}
