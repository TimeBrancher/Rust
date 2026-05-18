struct Triangulo{
	l1:f64,
	l2:f64,
	l3:f64;

}

impl Persona{
	fn new(base:f64,altura:f64) -> Triangulo{
		Triangulo{
		l1,
		l2,
		l3,
		}
	}
	fn calcular_Area(self) -> f64{
		let mut res = 0.0
		match self.tipo{
			"Rectangulo" => if (self.l1 > self.l2 && self.l2>self.l3){ res = self.l2*self.l3} if (self.l2>self.l3 && self.l2>self.l1){res = self.l1*self.l3} if (self.l3>self.l1 && self.l3>self.l2){res = self.l2*l3}
			"Equilatero" || "Isosceles" => let altura = (self.l1.pow(2)-(self.l1/2).pow(2)).pow(1/2); res=(altura*self.l1)/2;
		
	}
	res
	}
	fn calcular_Perimetro(self)-> f64{
		self.l1+self.l2+self.l3
	}
	fn tipo(self) -> String{
		let mut tipo &String = String::new();
		match (self.c_a,self.c_h,self.base){
			self.c_a == self.c_h && self.c_b == self.base => tipo = "Equilatero",
			self.c_a == self.c_b && self.c_b != self.base => tipo="Isosceles",
			self.c_a != self.c_b && self.c_a != self.base => tipo = "Rectangulo"
		}
		tipo
	}

}

fn main(){}
