fn multiplicar_valores (data: &mut [i32; 5],factor:i32){
	for i in 0..5{
		data[i] = data[i]*factor;
}
}

[#test]	
fn test() {
   let mut arreglo = [7,9,23,67,1];
   multiplicar_valores(&mut arreglo,5);
   assert_eq!(arreglo,[35,45,115,385,5]);
}
