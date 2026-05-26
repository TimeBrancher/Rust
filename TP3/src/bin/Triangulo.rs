struct Triangulo{
	l1:Option<f64>,
	l2:Option<f64>,
	l3:Option<f64>,

}

impl Triangulo{
	fn new(l1:Option<f64>,l2:Option<f64>,l3:Option<f64>) -> Triangulo{
		Triangulo{
		l1,
		l2,
		l3,
		}
	}
	fn calcular_area(&self) -> Option<f64>{
		let mut res:Option<f64> = None;
		if self.determinar_tipo() == "Rectangulo".to_string(){ if self.l1.unwrap() > self.l2.unwrap() && self.l1.unwrap()>self.l3.unwrap() { res = Some((self.l2.unwrap()*self.l3.unwrap())/2.0);} if self.l2.unwrap()>self.l3.unwrap() && self.l2.unwrap()>self.l1.unwrap() {res = Some((self.l1.unwrap()*self.l3.unwrap())/2.0);} if self.l3.unwrap()>self.l1.unwrap() && self.l3.unwrap()>self.l2.unwrap(){res = Some((self.l2.unwrap()*self.l1.unwrap())/2.0);}}
		if self.determinar_tipo() == "Equilatero".to_string(){ let altura = (self.l1.unwrap().powf(2.0) - (self.l1.unwrap()/2.0).powf(2.0)).powf(0.5); res=Some(altura*self.l1.unwrap())};
		if self.determinar_tipo() == "Isosceles".to_string(){if self.l1.unwrap() == self.l2.unwrap() {let b = self.l3.unwrap()/2.0; let a = self.l1.unwrap(); res = Some(b * (a.powf(2.0)-b.powf(2.0)).powf(0.5));} if self.l1.unwrap() == self.l3.unwrap() {let b = self.l2.unwrap()/2.0; let a = self.l1.unwrap(); res = Some(b * (a.powf(2.0)-b.powf(2.0)).powf(0.5));} if self.l3.unwrap() == self.l2.unwrap() {let b = self.l1.unwrap()/2.0; let a = self.l2.unwrap(); res = Some(b * (a.powf(2.0)-b.powf(2.0)).powf(0.5));}}
		if self.determinar_tipo() == "Invalido".to_string(){ res = None;}
		res
	}
	fn calcular_perimetro(&self)-> Option<f64>{
		let mut Res:Option<f64> = None;
		if self.determinar_tipo() != "Invalido"{
		Res = Some(self.l1.unwrap()+self.l2.unwrap()+self.l3.unwrap());}
		Res
	}
	fn determinar_tipo(&self) -> String{
		let mut tipo = String::new();
		if self.l1.is_some() && self.l2.is_some() && self.l3.is_some(){
			if self.l1.unwrap() == self.l2.unwrap() && self.l2.unwrap() == self.l3.unwrap() && self.l1.unwrap()>0.0 {tipo = "Equilatero".to_string();}
			else if (self.l1.unwrap() == self.l2.unwrap() && self.l2.unwrap() != self.l3.unwrap()) || (self.l2.unwrap()==self.l3.unwrap() && self.l2.unwrap()!=self.l1.unwrap()) || (self.l1.unwrap() == self.l3.unwrap() && self.l3.unwrap() != self.l2.unwrap()){tipo="Isosceles".to_string();}
			else if (self.l1.unwrap().powf(2.0) == (self.l2.unwrap().powf(2.0) + self.l3.unwrap().powf(2.0))) || (self.l2.unwrap().powf(2.0) == (self.l3.unwrap().powf(2.0) + self.l1.unwrap().powf(2.0))) || (self.l3.unwrap().powf(2.0) == (self.l1.unwrap().powf(2.0) + self.l2.unwrap().powf(2.0))) {tipo = "Rectangulo".to_string();}
			else {tipo = "Invalido".to_string();}
		}
		else {tipo = "Invalido".to_string();}
		tipo
	}

}

#[test]
fn test_rectangulo(){
	let triangulo = Triangulo::new(Some(3.0),Some(4.0),Some(5.0));
	assert_eq!(triangulo.determinar_tipo(),"Rectangulo");
	assert_eq!(triangulo.calcular_area(),Some(6.0));
	assert_eq!(triangulo.calcular_perimetro(),Some(12.0));
}

#[test]
fn test_faltante(){
	let triangulo = Triangulo::new(Some(3.0),None,Some(5.0));
	assert_eq!(triangulo.determinar_tipo(),"Invalido");
	assert_eq!(triangulo.calcular_area(),None);
	assert_eq!(triangulo.calcular_perimetro(),None);
}
#[test]
fn test_rectangulo_invalido(){
	let triangulo = Triangulo::new(Some(3.0),Some(5.0),Some(7.0));
	assert_eq!(triangulo.determinar_tipo(),"Invalido");
	assert_eq!(triangulo.calcular_area(),None);
	assert_eq!(triangulo.calcular_perimetro(),None);
}
#[test]
fn test_equilatero(){
	let triangulo = Triangulo::new(Some(1.0),Some(1.0),Some(1.0));
	assert_eq!(triangulo.determinar_tipo(),"Equilatero");
	assert_eq!(triangulo.calcular_area(),Some(0.8660254037844386));
	assert_eq!(triangulo.calcular_perimetro(),Some(3.0));
}
#[test]
fn test_isosceles(){
	let triangulo = Triangulo::new(Some(2.0),Some(2.0),Some(3.0));
	assert_eq!(triangulo.determinar_tipo(),"Isosceles");
	assert_eq!(triangulo.calcular_area(),Some(1.984313483298443));
	assert_eq!(triangulo.calcular_perimetro(),Some(7.0))
}
