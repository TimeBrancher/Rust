fn cantidad_impares ( data:[i32; 5])->i32{
	let mut cantidad = 0;
	for i in 0..5{
		if (data[i]%2)>0{
		cantidad = cantidad + 1;
}
}
	cantidad
}
	
#[test]
fn test() {
   assert_eq!(cantidad_impares([1,3,5,7,9]),5);
   assert_eq!(cantidad_impares([0,2,4,6,8]),0);
}
