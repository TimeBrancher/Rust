use::std::collections::VecDeque;
mod fecha;

use fecha::Fecha;
#[derive(PartialEq,Clone,Debug)]
enum TIPO{
	PERRO,
	GATO,
	CABALLO,
	OTROS
}
#[derive(PartialEq,Clone,Debug)]
struct Dueño{
	nombre:Option<String>,
	direccion:Option<String>,
	telefono:Option<String>,
}
impl Dueño{
	fn new(nombre:Option<String>,direccion:Option<String>,telefono:Option<String>) -> Dueño{
		Dueño{
			nombre,
			direccion,
			telefono,
		}
}
}
#[derive(PartialEq,Clone,Debug)]
struct Mascota{
	nombre:Option<String>,
	edad:Option<i32>,
	Tipo:Option<TIPO>,
	dueño:Option<Dueño>,
}
impl Mascota{
	fn new(nombre:Option<String>,edad:Option<i32>,tipo:Option<TIPO>,dueño:Option<Dueño>) -> Mascota{
		Mascota{

			nombre,
			edad,
			Tipo:tipo,
			dueño,
		}
	}
}

#[derive(PartialEq,Debug,Clone)]
struct Atencion{
	mascota:Option<Mascota>,
	Diagnostico:Option<String>,
	Tratamiento:Option<String>,
	Siguiente:Option<Fecha>,
}	
impl Atencion{
	fn new(mascota:Option<Mascota>,Diagnostico:Option<String>,Tratamiento:Option<String>,Siguiente:Option<Fecha>) -> Atencion{
			Atencion{
				mascota,
				Diagnostico,
				Tratamiento,
				Siguiente
			}
	}
}
#[derive(PartialEq)]
struct Veterinaria{
	nombre:Option<String>,
	direccion:Option<String>,
	ID:Option<i32>,
	Atenciones:Vec<Atencion>,
	Cola:VecDeque<Mascota>,

}
impl Veterinaria{
	fn new(nombre:Option<String>,direccion:Option<String>, ID:Option<i32>) -> Veterinaria{
		Veterinaria{
			nombre,
			direccion,
			ID,
			Atenciones:Vec::new(),
			Cola:VecDeque::new(),
		}
		
	}
	fn agregar_mascota(&mut self, mascota:Mascota){
		self.Cola.push_back(mascota);
	}
	fn agregar_prioridad(&mut self,mascota:Mascota){
		self.Cola.push_front(mascota);
	}
	fn atender(&mut self, D:Option<String>, T:Option<String>, F:Option<Fecha>){
		if D.is_some() && T.is_some() && F.is_some(){
			let temp = Atencion{
				mascota:Some(self.Cola[self.Cola.len()-1].clone()),
				Diagnostico:D,
				Tratamiento:T,
				Siguiente:F	
			};
			self.Cola.pop_back();
			self.registrar_atencion(temp);
		}
	}
	fn Eliminar_Mascota(&mut self, mascota:Mascota){
		for i in 0..self.Atenciones.len(){
			if self.Atenciones[i].mascota == Some(mascota.clone()){
				self.Atenciones.remove(i);
				break;
			}
		}
	}
	fn registrar_atencion(&mut self, A:Atencion){
			self.Atenciones.push(A);
	}
	fn Buscar(&mut self, n_D:String, n_M:String, T:String) -> Option<Mascota>{
		let mut mascota = None;
		for i in 0..self.Atenciones.len(){
			if self.Atenciones[i].mascota.clone().unwrap().nombre.clone().unwrap() == n_M && self.Atenciones[i].mascota.clone().unwrap().dueño.clone().unwrap().nombre.unwrap() == n_D && self.Atenciones[i].mascota.clone().unwrap().dueño.clone().unwrap().telefono.unwrap() == T{
					mascota = self.Atenciones[i].mascota.clone();
				}
		}
		return mascota;
	}
	fn cambiar_diagnostico(&mut self, D:String, A:Atencion){
		for i in 0..self.Atenciones.len(){
			if self.Atenciones[i] == A {self.Atenciones[i].Diagnostico = Some(D.clone());}
		}
	}
	fn cambiar_fecha(&mut self, F:Fecha,A:Atencion){
		for i in 0..self.Atenciones.len(){
			if self.Atenciones[i]==A{self.Atenciones[i].Siguiente = Some(F);}
			}
		}
	fn Eliminar_Atencion(&mut self, A:Atencion){
		for i in 0..self.Atenciones.len(){
			if self.Atenciones[i]==A{
				self.Atenciones.remove(i);
				break;
			}
		}
	}
}
#[test]
fn test_dueño(){
	let dueño = Dueño::new(Some("Marcos".to_string()),Some("Siempre Viva 123".to_string()),Some("+541234567".to_string()));
	assert_eq!(dueño.nombre,Some("Marcos".to_string()));
	assert_eq!(dueño.direccion,Some("Siempre Viva 123".to_string()));
	assert_eq!(dueño.telefono,Some("+541234567".to_string()));
}
#[should_panic="error datos insuficientes"]
#[test]
fn test_dueño_error(){
	let dueño = Dueño::new(None,None,None);
	if dueño.nombre == None || dueño.direccion == None || dueño.telefono == None{ panic!("error datos insuficientes");}
}
#[test]
fn test_new_atencion(){
	let dueño = Dueño::new(Some("Marcos".to_string()),Some("Siempre viva 123".to_string()),Some("+541234567".to_string()));
	let mascota = Mascota::new(Some("Tobi".to_string()),Some(4),Some(TIPO::PERRO),Some(dueño.clone()));
	let fecha = Fecha::new(3,5,2026);
	let atencion = Atencion::new(Some(mascota.clone()),Some("Diarrea".to_string()),Some("Dieta".to_string()),Some(fecha));
	assert_eq!(atencion.mascota,Some(mascota.clone()));
	assert_eq!(atencion.Diagnostico,Some("Diarrea".to_string()));
	assert_eq!(atencion.Tratamiento,Some("Dieta".to_string()));
	assert_eq!(atencion.Siguiente,Some(fecha));	
}

#[should_panic="error datos insuficientes"]
#[test]
fn test_new_atencion_error(){
	let atencion = Atencion::new(None,None,None,None);
	if atencion.mascota == None || atencion.Tratamiento == None || atencion.Diagnostico == None || atencion.Siguiente == None{
		panic!("error datos insuficientes");
	}
}
#[test]
fn test_new_mascota() {
	let dueño = Dueño::new(Some("Marcos".to_string()),Some("Siempre Viva 123".to_string()),Some("+541234567".to_string()));
	let mascota = Mascota::new(Some("Tobi".to_string()),Some(4),Some(TIPO::PERRO),Some(dueño.clone()));
	assert_eq!(mascota.nombre,Some("Tobi".to_string()));
	assert_eq!(mascota.edad,Some(4));
	assert_eq!(mascota.Tipo,Some(TIPO::PERRO));
}
	

#[should_panic="error datos insuficientes"]
#[test]
fn test_new_mascota_error(){
	let mascota = Mascota::new(None,None,None,None);
	if mascota.nombre == None || mascota.edad == None || mascota.Tipo == None || mascota.dueño == None{
		panic!("error datos insuficientes");
	}
}
#[test]
fn test_new_veterinaria(){
	let veterinaria = Veterinaria::new(Some("Veterinaria".to_string()),Some("Siempre viva 1234".to_string()),Some(1342));
	assert_eq!(veterinaria.nombre,Some("Veterinaria".to_string()));
	assert_eq!(veterinaria.direccion,Some("Siempre viva 1234".to_string()));
	assert_eq!(veterinaria.ID,Some(1342));
}
#[should_panic="error datos insuficientes"]
#[test]
fn test_new_veterinaria_error(){
	let veterinaria = Veterinaria::new(None,None,None);
	if veterinaria.nombre == None || veterinaria.direccion == None || veterinaria.ID == None{panic!("error datos insuficientes");}
}

#[test]
fn agregar_normal(){
	let mut veterinaria = Veterinaria::new(Some("Veterinaria".to_string()),Some("Siempre viva 1234".to_string()),Some(1342));
	let dueño = Dueño::new(Some("Marcos".to_string()),Some("Siempre viva 123".to_string()),Some("+541234567".to_string()));
	let mascota = Mascota::new(Some("Tobi".to_string()),Some(4),Some(TIPO::PERRO),Some(dueño.clone()));
	let mascota_2 = Mascota::new(Some("Mati".to_string()),Some(5),Some(TIPO::PERRO),Some(dueño.clone()));
	veterinaria.agregar_mascota(mascota.clone());
	veterinaria.agregar_mascota(mascota_2.clone());
	assert_eq!(veterinaria.Cola[0],mascota);
	assert_eq!(veterinaria.Cola[1],mascota_2);
}
#[test]
fn agregar_prioritario(){
	let mut veterinaria = Veterinaria::new(Some("Veterinaria".to_string()),Some("Siempre viva 1234".to_string()),Some(1342));
	let dueño = Dueño::new(Some("Marcos".to_string()),Some("Siempre viva 123".to_string()),Some("+541234567".to_string()));
	let mascota = Mascota::new(Some("Tobi".to_string()),Some(4),Some(TIPO::PERRO),Some(dueño.clone()));
	let mascota_2 = Mascota::new(Some("Tobi".to_string()),Some(4),Some(TIPO::PERRO),Some(dueño.clone()));
	veterinaria.agregar_mascota(mascota.clone());
	veterinaria.agregar_prioridad(mascota_2.clone());
	assert_eq!(veterinaria.Cola[0],mascota_2);
	assert_eq!(veterinaria.Cola[1],mascota);
}	

#[test]
fn test_atender(){
	let mut veterinaria = Veterinaria::new(Some("Veterinaria".to_string()),Some("Siempre viva 1234".to_string()),Some(1342));
	let dueño = Dueño::new(Some("Marcos".to_string()),Some("Siempre viva 123".to_string()),Some("+541234567".to_string()));
	let mascota = Mascota::new(Some("Tobi".to_string()),Some(4),Some(TIPO::PERRO),Some(dueño));
	veterinaria.agregar_mascota(mascota.clone());
	let fecha = Fecha::new(17,4,2026);
	veterinaria.atender(Some("Diarrea".to_string()),Some("Dieta".to_string()),Some(fecha));
	assert_eq!(veterinaria.Cola.len(),0);
	assert_eq!(veterinaria.Atenciones.len(),1);
}
#[test]
fn eliminar_atencion(){
	let mut veterinaria = Veterinaria::new(Some("Veterinaria".to_string()),Some("Siempre viva 1234".to_string()),Some(1342));
	let dueño = Dueño::new(Some("Marcos".to_string()),Some("Siempre viva 123".to_string()),Some("+541234567".to_string()));
	let mascota = Mascota::new(Some("Tobi".to_string()),Some(4),Some(TIPO::PERRO),Some(dueño));
	let fecha = Fecha::new(18,5,2026);
	let atencion = Atencion::new(Some(mascota.clone()),Some("Diarrea".to_string()),Some("Dieta".to_string()),Some(fecha));
	veterinaria.registrar_atencion(atencion.clone());
	veterinaria.Eliminar_Mascota(mascota.clone());
	assert_eq!(veterinaria.Atenciones.len(),0);
}
#[test]
fn Buscar_Mascota_Existente(){
	let mut veterinaria = Veterinaria::new(Some("Veterinaria".to_string()),Some("Siempre viva 1234".to_string()),Some(1342));
	let dueño = Dueño::new(Some("Marcos".to_string()),Some("Siempre viva 123".to_string()),Some("+541234567".to_string()));
	let mascota = Mascota::new(Some("Tobi".to_string()),Some(4),Some(TIPO::PERRO),Some(dueño.clone()));
	let fecha = Fecha::new(10,7,2026);
	let atencion = Atencion::new(Some(mascota.clone()),Some("Diarrea".to_string()),Some("Dieta".to_string()),Some(fecha));
	veterinaria.registrar_atencion(atencion.clone());
	let busqueda = veterinaria.Buscar("Marcos".to_string(),"Tobi".to_string(),"+541234567".to_string());
	assert_eq!(busqueda,Some(mascota.clone()));
}
#[test]
fn Buscar_Mascota_Inexistente(){
	let mut veterinaria = Veterinaria::new(Some("Veterinaria".to_string()),Some("Siempre viva 1234".to_string()),Some(1342));
	assert_eq!(veterinaria.Buscar("Marcos".to_string(),"Tobi".to_string(),"+541234567".to_string()),None);
}
#[test]
fn test_registrar_atencion(){
	let mut veterinaria = Veterinaria::new(Some("Veterinaria".to_string()),Some("Siempre viva 1234".to_string()),Some(1342));
	let dueño = Dueño::new(Some("Marcos".to_string()),Some("Siempre viva 123".to_string()),Some("+541234567".to_string()));
	let mascota = Mascota::new(Some("Tobi".to_string()),Some(4),Some(TIPO::PERRO),Some(dueño.clone()));
	let fecha = Fecha::new(10,7,2026);
	let atencion = Atencion::new(Some(mascota.clone()),Some("Diarrea".to_string()),Some("Dieta".to_string()),Some(fecha));
	veterinaria.registrar_atencion(atencion.clone());
	assert_eq!(veterinaria.Atenciones[0],atencion.clone());
}

#[test]
fn test_cambiar_diagnostico(){
	let mut veterinaria = Veterinaria::new(Some("Veterinaria".to_string()),Some("Siempre viva 1234".to_string()),Some(1342));
	let dueño = Dueño::new(Some("Marcos".to_string()),Some("Siempre viva 123".to_string()),Some("+541234567".to_string()));
	let mascota = Mascota::new(Some("Tobi".to_string()),Some(4),Some(TIPO::PERRO),Some(dueño.clone()));
	let fecha = Fecha::new(10,9,2026);
	let atencion = Atencion::new(Some(mascota.clone()),Some("Diarrea".to_string()),Some("Dieta".to_string()),Some(fecha));
	veterinaria.registrar_atencion(atencion.clone());
	veterinaria.cambiar_diagnostico("Fiebre".to_string(),atencion.clone());
	assert_eq!(veterinaria.Atenciones[0].Diagnostico, Some("Fiebre".to_string()));
}
#[test]
fn test_cambiar_fecha(){
	let mut veterinaria = Veterinaria::new(Some("Veterinaria".to_string()),Some("Siempre viva 1234".to_string()),Some(1342));
	let dueño = Dueño::new(Some("Marcos".to_string()),Some("Siempre viva 123".to_string()),Some("+541234567".to_string()));
	let mascota = Mascota::new(Some("Tobi".to_string()),Some(4),Some(TIPO::PERRO),Some(dueño.clone()));
	let fecha = Fecha::new(10,9,2026);
	let fecha_2 = Fecha::new(11,9,2026);
	let atencion = Atencion::new(Some(mascota.clone()),Some("Diarrea".to_string()),Some("Dieta".to_string()),Some(fecha));
	veterinaria.registrar_atencion(atencion.clone());
	veterinaria.cambiar_fecha(fecha_2,atencion.clone());
	assert_eq!(veterinaria.Atenciones[0].Siguiente, Some(fecha_2));
	
}
#[test]
fn test_eliminar_atencion(){
	let mut veterinaria = Veterinaria::new(Some("Veterinaria".to_string()),Some("Siempre viva 1234".to_string()),Some(1342));
	let dueño = Dueño::new(Some("Marcos".to_string()),Some("Siempre viva 123".to_string()),Some("+54123457".to_string()));
	let mascota = Mascota::new(Some("Tobi".to_string()),Some(4),Some(TIPO::PERRO),Some(dueño.clone()));
	let fecha = Fecha::new(10,9,2026);
	let atencion = Atencion::new(Some(mascota.clone()),Some("Diarrea".to_string()),Some("Dieta".to_string()),Some(fecha));
	veterinaria.registrar_atencion(atencion.clone());
	veterinaria.Eliminar_Atencion(atencion.clone());
	assert_eq!(veterinaria.Atenciones.len(),0);
}
