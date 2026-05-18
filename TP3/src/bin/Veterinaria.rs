//como llamo el modulo fecha?
enum tipo{
	PERRO,
	GATO,
	CABALLO,
	OTROS
}
struct Dueño{
	nombre:String,
	direccion:String,
	telefono:String
}
struct mascota{
	nombre:String,
	edad:String,
	Tipo:tipo,
	dueño:Dueño,
}

struct atencion{
	Mascota:mascota,
	Diagnostico:String,
	Tratamiento:String,
	}	
struct Veterinaria{
	nombre:String,
	direccion:String,
	ID:i32,
	Atenciones:Vec<Mascota>
	Cola:VecDeque<mascota>

}
impl Veterinaria{
	fn new(nombre:String,direccion:Cancion, ID:i32) -> Veterinaria{
		{
			nombre,
			direccion,
			ID,
			self.Atenciones = Vec::new(),
			self.Cola = VecDeque::new(),
		}
		
	}
	fn agregar_mascota(&mut self, Mascota:mascota){
		self.Cola.pushback(Mascota);
	}
	fn eliminar_prioridad(&mut self,Mascota:mascota){
		self.Cola.pushfront(Mascota);
	}
	fn atender(&mut self,cancion,pos){
		//???
	}

fn main(){}
