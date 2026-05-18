struct Persona{
	nombre:String,
	edad:i32,
	direccion:String,

}

impl Persona{
	fn new(nombre:String,edad:i32,direccion:String) -> Persona{
		Persona{
			nombre,
			edad,
			direccion
	}
	fn toString(self){
		println!("La persona de nombre {}, tiene edad de {} y vive en {}",self.nombre,self.edad,self.direccion);
	}
	fn obtenerEdad(self)-> i32{
		self.edad
	}
	fn actualizardireccion(&mut self, direccion:String){
		self.direccion=direccion;
	}
	

}

fn main(){}
