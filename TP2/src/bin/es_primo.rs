fn es_primo ( data:i32)->bool{
	let mut md = 0;
	for i in 1..data{
		if (data/i)==1{
	md = i;
	break;}

}
	data==md
}
	
[#test]
fn test_primo(){
	assert_eq!(es_primo(3),true);
	assert_eq!(es_primo(4),false)
}
