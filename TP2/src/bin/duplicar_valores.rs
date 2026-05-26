fn duplicar_valores ( data:[f32; 5])->[f32;5]{
	let mut arreglo:[f32;5]=[0.0,0.0,0.0,0.0,0.0];
	for i in 0..5{
		arreglo[i] = data[i]*2.0;
}
	arreglo
}
#[test]	
fn test_correcto() {
   let arreglo = [3.2,5.1,9.5,21.89,3.14];
   assert_eq!(duplicar_valores(arreglo),[3.2,5.1,9.5,21.89,3.19]);}
#[should_panic="Arreglo incorrecto de tamaño"]
#[test]
fn test_largo(){
	let arreglo = [3.2,1.5,2.3,9.6];
	if arreglo.len()!=5{
		panic!("Arreglo incorrecto de tamaño");
	}
}
#[test]
fn test_incorrecto(){
	let arreglo = [3.5,1.5,2.5,6.5,9.5]
	assert_ne!(duplicar_valores(arreglo));
}
