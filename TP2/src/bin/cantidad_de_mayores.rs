fn cantidad_de_mayores ( data:[i32; 5], lim:i32)->i32{
	let mut total = 0;
	for i in 0..5{
		if data[i]>lim{
		total = total + 1;
}
}
	total
}
#[test]	
fn test() {
   let arreglo = [1,30,50,90,6];
   assert_eq!(cantidad_de_mayores(arreglo,5),4);
   assert_eq!(cantidad_de_mayores(arreglo,999),0);
}
