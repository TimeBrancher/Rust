fn sumar_arreglos ( data:[f64; 5],data2:[f64;5])->[f64;5]{
	let mut arreglo = [0.0,0.0,0.0,0.0,0.0];
	for i in 0..5{
		arreglo[i] = data[i]+data2[i];
}
	arreglo
}
#[test]	
fn test() {
   let arreglo = [3.2,5.1,9.5,21.89,3.14];
   let arreglo2 = [1.87,3.1415,39.21,13.78,5.08];
   assert_eq!(sumar_arreglos(arreglo,arreglo2),[5.07,8.2415,48.71,35.67,8.22]);
   
}
