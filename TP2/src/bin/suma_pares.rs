fn suma_pares ( data:[i32; 5])->i32{
	let mut total = 0;
	for i in 1..5{
		if (data[i]%2)==0{
		total = total + data[i];
}
}
	total
}
	
#[test]
fn test_correcto(){
	assert_eq!(suma_pares([1,2,3,4,5]),6);}
#[test]
fn test_incorrecto(){
	assert_eq!(suma_pares([1,3,5,7,9]),0);
}	
#[should_panic(expected="el vector no tiene datos suficiente")]
#[test]
fn test_cantidad(){
	let mut vector = [1,4,7,9];
	if vector.len() != 5 {
		panic!("el vector no tiene datos suficiente");
	}
}
