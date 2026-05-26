fn multiplicar_valores (data: &mut [i32; 5],factor:i32){
	for i in 0..5{
		data[i] = data[i]*factor;
}
}

#[test]	
fn test_multiplicacion() {
   let mut arreglo = [7,9,23,67,1];
   multiplicar_valores(&mut arreglo,5);
   assert_eq!(arreglo,[35,45,115,335,5]);
   multiplicar_valores(&mut arreglo,0);
   assert_eq!(arreglo,[0;5]);
}

#[test]
fn test_negativo(){
	let mut arreglo = [3;5];
	multiplicar_valores(&mut arreglo,-1);
	assert_eq!(arreglo,[-3;5]);
	}
#[should_panic="parametros insuficientes"]
#[test]
fn test_pequeño(){
	let mut arreglo = [3;4];
	if arreglo.len()!=5{
		panic!("parametros insuficientes");
	}
}
