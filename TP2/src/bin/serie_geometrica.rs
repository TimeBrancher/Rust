fn serie_geometrica ( data:i32)->[i32;5]{
	let mut arreglo:[i32;5] = [1,0,0,0,0];
	for i in 1..5{
		arreglo[i] = arreglo[i-1]*(data);
}
	arreglo
}
#[test]	
fn test() {
   let size  = 2;
   let resultado = serie_geometrica(2);
   assert_eq!(resultado, [1,2,4,8,16]);
}
#[should_panic="error,desbordamiento"]
#[test]
fn test_over(){
	let num = 47;
	if num >= 47{
		serie_geometrica(num);
		panic!("error,desbordamiento");
	}
}
#[test]
fn test_neg(){
	let num = -1;
	assert_eq!(serie_geometrica(num),[1,-1,1,-1,1]);
}
#[test]
fn test_0(){
	let num = 0;
	assert_eq!(serie_geometrica(num),[1,0,0,0,0]);
} 
