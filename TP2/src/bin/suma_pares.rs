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
fn test(){
	assert_eq!(suma_pares([1,2,3,4,5]),6);
	assert_eq!(suma_pares([1,3,5,7,9]),0);
}	
