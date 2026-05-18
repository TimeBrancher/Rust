struct Producto{
	nombre:String,
	precio_bruto:f64,
	identificador:String,

}

impl Producto{
	fn new(nombre:String,precio_bruto:f64,identificador:String) -> Producto{
		Producto{
			nombre,
			precio_bruto,
			identificador,
		}
	}
	fn calcularImpuesto(impuesto:i32) -> f64{
		impuesto_proc:f64 = impuesto/100;
		precio_bruto * impuesto_proc
	}
	fn calcularDescuento(descuento:i32) -> f64{
		descuento_proc:f64 = descuento/100;
		precio_bruto * descuento_proc
	}
	fn calcularPrecioTotal(impuesto:i32, descuento:i32){
		let mut tot = 0.0;
		match impuesto = {
			Some(n) => tot = self.precio + calcularImpuesto(impuesto);  
			none => __
	}
		match descuento = {
			Some(n) => tot = tot + tot * (descuento/100),
			none => __
	}
		
		tot

}

fn main(){}


//parametros opcionales
