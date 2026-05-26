fn es_primo (data:i32)->bool{
	let mut lim:i32 = 0;
	if data<0 { lim = data*-1;}
	if data>0 { lim = data;}
	let mut d = 1;
	for i in 2..lim-1{
		if lim%i==0{ d = i; break;}
	}
	d==1
}
	
#[test]
fn test_primo(){
	assert_eq!(es_primo(3),true);
}
#[test]
fn test_no_primo(){
	assert_eq!(es_primo(4),false);
}
#[test]
fn test_dato_chico(){
	assert_eq!(es_primo(-5),true);
}
