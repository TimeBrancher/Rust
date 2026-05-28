#[derive(PartialEq,Debug,Clone)]

struct Auto{
	marca:Option<String>,
	modelo:Option<String>,
	año:Option<i32>,
	precio:Option<f64>,
	color:Option<String>,
}

impl Auto{
	fn new(marca:Option<String>, modelo:Option<String>,año:Option<i32>,precio:Option<f64>,color:Option<String>) -> Auto{
		Auto{
			marca,
			modelo,
			año,
			precio,
			color,
	}
	}
	fn calcular_precio(&mut self) -> Option<i32>{
		if self.precio.is_some(){
			let mut Precio = self.precio;
			if self.color.is_some(){
				match &self.color.clone().unwrap(){
					col if col == "Rojo" || col == "Amarillo" || col == "Azul" => Precio = Some(Precio.unwrap() * 1.25),
					__ => Precio=Some(Precio.unwrap()*0.9),
				}
			}
			if self.marca.is_some(){
				if self.marca.clone().unwrap() == "BMW"{
					Precio =Some(Precio.unwrap() * 1.15);}
				}
			if self.año.is_some(){
				if self.año.unwrap() > 2000{
					Precio = Some(Precio.unwrap() * 0.95);}	
				}
				let temp = Precio.unwrap() as i32;
			Some(temp)
			}
		else{
			None}
	
		
	}
}
#[derive(Clone,Debug)]
struct ConcesionarioAuto{
	nombre:Option<String>,
	direccion:Option<String>,
	Autos:Vec<Auto>,
}
impl ConcesionarioAuto{
	fn new(nombre:Option<String>,direccion:Option<String>) -> ConcesionarioAuto{
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
				self.Autos.remove(i);
				break;
			}
		}
	}
	fn buscar_auto(&mut self, auto:Auto) -> Option<Auto> {
		for i in 0..self.Autos.len(){
			if self.Autos[i] == auto{
				return Some(self.Autos[i].clone());
			}
		
		}	
		None
	}

}

#[test]
fn test_new_auto(){
	let auto = Auto::new(Some("BMW".to_string()),Some("Modelo".to_string()),Some(2005),Some(32000.0),Some("Azul".to_string()));
	assert_eq!(auto.marca,Some("BMW".to_string()));
	assert_eq!(auto.modelo,Some("Modelo".to_string()));
	assert_eq!(auto.año,Some(2005));
	assert_eq!(auto.precio,Some(32000.0));
	assert_eq!(auto.color,Some("Azul".to_string()));
}
#[should_panic="datos insuficientes"]
#[test]
fn test_auto_fallido(){
	let auto_marca = Auto::new(None,Some("Modelo".to_string()),Some(2005),Some(32000.0),Some("Azul".to_string()));
	let auto_modelo = Auto::new(Some("BMW".to_string()),None,Some(2005),Some(32000.0),Some("Azul".to_string()));
	let auto_año = Auto::new(Some("BMW".to_string()),Some("Modelo".to_string()),None,Some(32000.0),Some("Azul".to_string()));
	let auto_precio = Auto::new(Some("BMW".to_string()),Some("Modelo".to_string()),Some(2005),None,Some("Azul".to_string()));
	let auto_color = Auto::new(Some("BMW".to_string()),Some("Modelo".to_string()),Some(2005),Some(32000.0),None);
	let Arr:[Auto;5] = [auto_marca,auto_modelo,auto_año,auto_precio,auto_color];
	for i in 0..5{
		if Arr[i].marca == None || Arr[i].modelo == None || Arr[i].año == None || Arr[i].precio == None || Arr[i].color == None{ panic!("datos insuficientes");}
	}
}
#[test]
fn test_calcular_precio(){
	let mut auto = Auto::new(Some("BMW".to_string()),Some("Modelo".to_string()),Some(2005),Some(32000.0),Some("Azul".to_string()));
	assert_eq!(auto.calcular_precio(),Some(43700));
}

#[test]
fn test_concensionario(){
	let concesionario = ConcesionarioAuto::new(Some("Consecionario".to_string()), Some("Siempre viva 1234".to_string()));
	assert_eq!(concesionario.nombre,Some("Consecionario".to_string()));
	assert_eq!(concesionario.direccion,Some("Siempre viva 1234".to_string()));
}
#[should_panic="datos insuficientes"]
#[test]
fn test_concesionario_fallo(){
	let concesionario_1 = ConcesionarioAuto::new(None,Some("Siempre viva 1234".to_string()));
	let concesionario_2 = ConcesionarioAuto::new(Some("Consecionario".to_string()),None);
	if concesionario_1.nombre == None || concesionario_1.direccion == None || concesionario_2.nombre == None || concesionario_2.direccion == None{panic!("datos insuficientes");}
}
#[test]
fn test_agregar_auto(){
	let mut concesionario = ConcesionarioAuto::new(Some("Consecionario".to_string()),Some("Siempre viva 1234".to_string()));
	let auto = Auto::new(Some("BMW".to_string()),Some("Modelo".to_string()),Some(2005),Some(32000.0),Some("Azul".to_string()));
	concesionario.agregar_auto(auto.clone());
	assert_eq!(concesionario.Autos[0],auto.clone());
}
#[should_panic="Vector sin valores"]
#[test]
fn test_quitar_auto(){
	let mut concesionario = ConcesionarioAuto::new(Some("Consecionario".to_string()),Some("Siempre viva 1234".to_string()));
	let auto = Auto::new(Some("BMW".to_string()),Some("Modelo".to_string()),Some(2005),Some(320000.0),Some("Azul".to_string()));
	concesionario.agregar_auto(auto.clone());
	assert_eq!(concesionario.Autos[0],auto);
	concesionario.eliminar_auto(auto.clone());
	if concesionario.Autos.len() <= 0{
		panic!("Vector sin valores");}
}

#[test]
fn test_buscar_auto(){
	let mut concesionario = ConcesionarioAuto::new(Some("Consecionario".to_string()),Some("Siempre viva 1234".to_string()));
	let auto_1 = Auto::new(Some("BMW".to_string()),Some("Modelo".to_string()),Some(2005),Some(320000.0),Some("Azul".to_string()));
	let auto_2 = Auto::new(Some("Ferrari".to_string()),Some("Modelo".to_string()),Some(2005),Some(50000.0),Some("Rojo".to_string()));
	concesionario.agregar_auto(auto_1.clone());
	assert_eq!(concesionario.buscar_auto(auto_1.clone()),Some(auto_1));
	assert_eq!(concesionario.buscar_auto(auto_2.clone()),None);
}
