
fn ordenar_nombres (data: &mut [String; 5]){
	for i in 0..5{
		for j in 0..5{
			let temp = data[i].clone();
			let aux = data[j].clone();
			if temp<aux{
				data[j]=temp.to_string();
				data[i]=aux.to_string();
			}
}
}
}

#[test]	
fn test() {
   let mut arreglo:[String;5] = ["Martinez".to_string(),"Aaron".to_string(),"Zara".to_string(),"Carlos".to_string(),"1".to_string()];
   ordenar_nombres(&mut arreglo);
   assert_eq!(arreglo,["1".to_string(),"Aaron".to_string(),"Carlos".to_string(),"Martinez".to_string(),"Zara".to_string()]);
}
#[should_panic="datos insuficientes"]
#[test]
fn test_insuficiente(){
	let mut arreglo:[String;4] = ["a".to_string(),"a".to_string(),"a".to_string(),"a".to_string()];
	if arreglo.len()!=5{
		panic!("datos insuficientes");
	}
}

