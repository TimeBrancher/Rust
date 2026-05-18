fn ordenar_nombres (data: &mut [String; 5]){
	for i in 0..5{
		for j in 0..5{
			let temp = &data[i];
			let aux = &data[j];
			if temp<aux{
				data[j]=temp.to_string();
				data[i]=aux.to_string();
			}
}
}
}

[#test]	
fn test() {
   let mut arreglo:String = ["Martinez","Aaron","Zara","Carlos","1"].to_string();
   ordenar_nombres(&mut arreglo);
   assert_eq!(arreglo,["1","Aaron","Carlos","Martinez","Zara"]);
}


