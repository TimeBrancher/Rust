#[derive(Copy,Clone)]
struct Rectangulo{
	base:f64,
	altura:f64,

}

impl Rectangulo{
	fn new(base:f64,altura:f64) -> Rectangulo{
		Rectangulo{
			base,
			altura,
		}
	}
	fn calcular_area(self) -> f64{
		self.base*self.altura
	}
	fn calcular_perimetro(self)-> f64{
		if self.base > 0.0 && self.altura > 0.0{
			(2.0*self.base)+(2.0*self.altura)}
		else{0.0}
	}
	fn es_cuadrado(self)->bool{
		(self.base==self.altura) && (self.base>0.0)
	}
	

}

#[test]
fn test_cuadrado(){
	let rectangulo = Rectangulo::new(2.0,2.0);
	assert_eq!(rectangulo.calcular_area(),4.0);
	assert_eq!(rectangulo.calcular_perimetro(),8.0);
	assert_eq!(rectangulo.es_cuadrado(),true);
}
#[test]
fn test_rectangulo(){
	let rectangulo = Rectangulo::new(2.0,1.0);
	assert_eq!(rectangulo.calcular_area(),2.0);
	assert_eq!(rectangulo.calcular_perimetro(),6.0);
	assert_eq!(rectangulo.es_cuadrado(),false);
}
#[test]
fn test_cero(){
	let rectangulo = Rectangulo::new(2.0,0.0);
	assert_eq!(rectangulo.calcular_area(),0.0);
	assert_eq!(rectangulo.calcular_perimetro(),0.0);
	assert_eq!(rectangulo.es_cuadrado(),false);
}
