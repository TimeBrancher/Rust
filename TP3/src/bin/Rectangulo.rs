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
	fn calcular_Area(self) -> f64{
		self.base*self.altura
	}
	fn calcular_Perimetro(self)-> f64{
		(2.0*self.base)+(2.0*self.altura)
	}
	fn esCuadrado(self)->bool{
		self.base==self.altura
	}
	

}

fn main(){}
