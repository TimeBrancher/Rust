#[derive(PartialEq,Debug,Clone)]

struct Auto{
	marca:String,
	modelo:String,
	año:i32,
	precio:f64,
	color:String
}

impl Auto{
	fn new(marca:String, modelo:String,año:i32,precio:f64,color:String) -> Auto{
		Auto{
			marca,
			modelo,
			año,
			precio,
			color,
	}
	}
	fn calcular_precio(&mut self) -> i32{
		let mut Precio = self.precio;
		match &self.color{
			col if col == "Rojo" || col == "Amarillo" || col == "Azul" => Precio = Precio * 1.25,
			__ => Precio=Precio*0.9,
		}
		if self.marca == "BMW"{
			Precio = Precio * 1.15;}
		if self.año > 2000{
			Precio = Precio * 0.95;}	
		let res = Precio as i32;
		res
	}
}
#[derive(Clone,Debug)]
struct ConcesionarioAuto{
	nombre:String,
	direccion:String,
	Autos:Vec<Auto>,
}
impl ConcesionarioAuto{
	fn new(nombre:String,direccion:String) -> ConcesionarioAuto{
		ConcesionarioAuto{
			nombre,
			direccion,
			Autos:Vec::new(),
		}
		
	}
	fn agregar_auto(&mut self, auto:Auto){
		self.Autos.push(auto);
	}
	fn eliminar_auto(&mut self, auto:Auto){
		for i in 0..self.Autos.len(){
			if self.Autos[i] == auto{
				self.Autos.remove[i];
				break;
			}
		}
	}
	fn buscar_auto(&mut self, auto:Auto) -> Option<Auto> {
		for i in 0..self.Autos.len(){
			if self.Autos[i] == auto{
				Some(self.Autos[i].pop())
			}
		
		}	
		None
	}

}

#[test]
fn Concensionario(){
	let mut auto = Auto::new("BMW".to_string(),"Modelo".to_string(),2005,32000.0,"Azul".to_string());
	let mut concencionario = ConcesionarioAuto::new("AutosCon".to_string(),"Elm Street".to_string());
	concencionario.agregar_auto(auto);
	assert_eq!(concencionario.Autos[0].calcular_precio(),43700);
	assert_eq!(concencionario.buscar_auto(auto),Some(auto));
	concencionario.eliminar_auto(auto);
	assert_eq!(concencionario.buscar_auto(auto),None);
}
