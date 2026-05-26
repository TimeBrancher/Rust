struct Persona{
	nombre:Option<String>,
	edad:Option<i32>,
	direccion:Option<String>,

}

impl Persona{
	fn new(nombre:Option<String>,edad:Option<i32>,direccion:Option<String>) -> Persona{
		Persona{
			nombre,
			edad,
			direccion,
	}
	}
	fn to_string(&self) -> String{
		let mut texto:String = String::new();
		if self.nombre.is_some(){texto = format!("{}nombre:{}",texto,self.nombre.clone().unwrap());}
		if self.edad.is_some(){texto = format!("{} edad:{}",texto,self.edad.clone().unwrap());}
		if self.direccion.is_some(){texto = format!("{} direccion:{}",texto,self.direccion.clone().unwrap());}
		texto
	}
	fn obtener_edad(self)-> Option<i32>{
		match self.edad{
			Some(edad) => Some(edad),
			None => None,
	} 
	}
	fn actualizar_direccion(&mut self, direccion:Option<String>){
		self.direccion=direccion;
	}
	

}



#[test]
fn test_completo(){
	let persona = Persona::new(Some("Pablo".to_string()),Some(32),Some("Siempre Viva 234".to_string()));
	assert_eq!(persona.to_string(),"nombre:Pablo edad:32 direccion:Siempre Viva 234");
	assert_eq!(persona.obtener_edad(),Some(32));
}
#[test]
fn test_sin_edad(){
	let persona = Persona::new(Some("Pablo".to_string()),None,Some("Siempre Viva 234".to_string()));
	assert_eq!(persona.obtener_edad(),None);
}

#[test]
fn test_actualizar(){
	let mut persona = Persona::new(Some("Pablo".to_string()),Some(32),Some("Siempre Viva 234".to_string()));
	persona.actualizar_direccion(Some("Elm Street".to_string()));
	assert_eq!(persona.to_string(),"nombre:Pablo edad:32 direccion:Elm Street");
}
#[test]
fn test_to_String(){
	let persona_nombre=Persona::new(None,Some(32),Some("Siempre Viva 234".to_string()));
	assert_eq!(persona_nombre.to_string()," edad:32 direccion:Siempre Viva 234");
	let persona_edad=Persona::new(Some("Pablo".to_string()),None,Some("Siempre Viva 234".to_string()));
	assert_eq!(persona_edad.to_string(),"nombre:Pablo direccion:Siempre Viva 234");
	let persona_direccion=Persona::new(Some("Pablo".to_string()),Some(32),None);
	assert_eq!(persona_direccion.to_string(),"nombre:Pablo edad:32");
	let persona_vacio = Persona::new(None,None,None);
	assert_eq!(persona_vacio.to_string(),"");

}

