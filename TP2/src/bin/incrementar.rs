fn incrementar (data: &mut f64){
	*data = *data+1.0;
}

#[test]	
fn test() {
   let mut parametro = 3.7;
   incrementar(&mut parametro);
   assert_eq!(parametro,4.7);
}

#[test]
fn test_neg(){
   let mut parametro = -1.0;
   incrementar(&mut parametro);
   assert_eq!(parametro,0.0);
}

#[test]
fn test_point_zero(){
	let mut parametro = 0.0;
	incrementar(&mut parametro);
	assert_eq!(parametro,1.0);
}

