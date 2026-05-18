fn incrementar (data: &mut f64){
	*data = *data+1.0;
}

[#test]	
fn test() {
   let mut parametro = 3.7;
   incrementar(&mut parametro);
   assert_eq!(parametro,4.7);
   parametro = 9999999;
   incrementar(&mut parametro);
   assert_eq!(parametro,10000000);
}
