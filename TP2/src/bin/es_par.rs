fn es_par ( data:i32)->bool{
	data%2==0
}


#[test]
fn test_par(){
	assert_eq!(es_par(2),true);}
#[test]
fn test_impar(){
	assert_eq!(es_par(3),false);
}
