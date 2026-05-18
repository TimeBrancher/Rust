fn reemplazar_par (data: &mut [i32; 5]){
	for i in 0..5{
		if data[i]%2==0{
			data[i] = -1;
}
}
}
#[test]	
fn test() {
   let mut arreglo = [7,9,23,67,2];
   reemplazar_par(&mut arreglo);
   assert_eq!(arreglo,[7,9,23,67,-1]);
}
