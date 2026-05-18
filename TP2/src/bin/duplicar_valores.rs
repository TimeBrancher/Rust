fn duplicar_valores ( data:[f32; 5])->[f32;5]{
	let mut arreglo:[f32;5]=[0.0,0.0,0.0,0.0,0.0];
	for i in 0..5{
		arreglo[i] = data[i]*2.0;
}
	arreglo
}
#[test]	
fn test() {
   let arreglo = [3.2,5.1,9.5,21.89,3.14];
   let arreglo2 = duplicar_valores(arreglo);
   assert_eq!(arreglo2, [6.4,10.2,19.0,43.78,6.28]);
   assert_neq!(arreglo2, [9.6,15.3,28.5,65.67,0.42]);
}
