fn es_par ( data:i32)->bool{
	data%2==0
}


#[test]
fn test(){
	assert_eq!(es_par(2),true);
	assert_eq!(es_par(3),false);
}
