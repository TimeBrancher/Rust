struct Producto{
	nombre:String,
	precio_bruto:Option<f64>,
	identificador:String,

}

impl Producto{
	fn new(nombre:String,precio_bruto:Option<f64>,identificador:String) -> Producto{
		Producto{
			nombre,
			precio_bruto,
			identificador,
		}
	}
	fn calcular_impuesto(&self,impuesto:Option<i32>) -> Option<f64>{
		let mut res:Option<f64> = None;
		if impuesto.is_some() && self.precio_bruto.is_some(){
			let conversion:f64 = impuesto.unwrap() as f64;
			res = Some(self.precio_bruto.unwrap() * (conversion/100.0));
			}
		res
	}
	fn calcular_descuento(&self, descuento:Option<i32>) -> Option<f64>{
		let mut res:Option<f64> = None;
		if descuento.is_some() && self.precio_bruto.is_some(){
			let conversion:f64 = descuento.unwrap() as f64;
			res = Some(self.precio_bruto.unwrap() * (conversion/100.0));
		}
		res
	}
	fn calcular_precio_total(&mut self,impuesto:Option<i32>, descuento:Option<i32>){
		let mut imp = 0.0;
		let mut desc = 0.0;
		let mut tot = 0.0;
		if self.precio_bruto.is_some(){
			tot = self.precio_bruto.unwrap();
			if impuesto.is_some(){imp = self.calcular_impuesto(impuesto).unwrap();}
			if descuento.is_some(){desc = self.calcular_descuento(descuento).unwrap();}}
		tot = tot + imp - desc;
		self.precio_bruto = Some(tot);
		
	}

}

#[test]
fn impuesto_test(){
	let producto = Producto::new("Pava".to_string(),Some(3.0),"PV".to_string());
	assert_eq!(producto.calcular_impuesto(Some(20)),Some(0.6000000000000001));
}
#[test]
fn descuento_test(){
	let producto = Producto::new("Pava".to_string(),Some(3.0),"PV".to_string());
	assert_eq!(producto.calcular_descuento(Some(20)),Some(0.6000000000000001));
}
#[test]
fn calcular_precio_total(){
	let mut producto = Producto::new("Pava".to_string(),Some(3.0),"PV".to_string());
	producto.calcular_precio_total(Some(20),Some(35));
	assert_eq!(producto.precio_bruto,Some(2.5500000000000003));
}
#[test]
fn test_sin_valor(){
	let producto_valor = Producto::new("Pava".to_string(),None,"PV".to_string());
	assert_eq!(producto_valor.calcular_descuento(Some(20)),None);
}
