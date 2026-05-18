mod TP3;
TP3::Fecha();
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
	Siguiente:Fecha,
	}	
struct Veterinaria{
	nombre:String,
	direccion:String,
	ID:i32,
	Atenciones:Vec<atencion>
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
	fn atender(&mut self, D:Diagnostico, T:Tratamiento, F:Fecha){
		temp = atencion{
			Self.Cola.Mascoa,
			D,
			T,
			F,	
	}
		Self.Atenciones.add(temp);
	}
	fn Eliminar_Mascota(&mut self, Mascota:mascota){
		let mut i = 0,
		while (i<Self.Atenciones.size(){
			if (i.Self.Atenciones(i) == Mascota{
				Self.Atenciones.remove(i);
				i = Self.Atenciones.size();
				}
				i=i+1;
			}
		}
	fn Registrar_Atencion(&mut self, A:Atencion){
			Self.Atenciones.add(A);
	}
	fn Buscar(&mut self, n_D:String, n_M:String, T:String) -> bool{
		let mut temp = false;
		for i in 0..self.Atenciones.size(){
			if (self.Atenciones(i).mascota.nombre == n_M && self.Atenciones(i).nombre == n_D && self.Atenciones(i).mascota.dueño.telefono == T){temp=true}
		temp
	}
	fn Cambiar_Diagnostico(&mut self, D:Diagnostico, A:Atencion){
		for i in 0..self.Atenciones.size(){
			if (Self.Atenciones(i) == A){Self.Atenciones(i).Diagnostico == D;}
		}
	}
	fn Cambiar_Fecha(&mut self, F:Fecha,A:Atencion){
		for i in 0..self.Atenciones.size(){
			if (Self.Atenciones(i)==A){Self.Atencion(i).Fecha == F;}
			}
		}
	fn Eliminar_Atencion(&mut self, A:Atencion){
		for i in 0..self.Atenciones.size(){
			if Self.Atenciones(i)==A{
				Self.Atenciones.remove(i);}
			}
	}
#[test]
fn test() {
}

