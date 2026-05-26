mod TP3;
TP3::Fecha();
enum genero{
	NOVELA,
	INFANTIL,
	TECNICO,
	OTROS,
}
struct Cliente{
	nombre:String,
	direccion:String,
	telefono:String,
	correo:String
}
struct Libro{
	ISBN:String,
	Titulo:String,
	Autor:String,
	Numero_Paginas:i32,
	Genero:genero,
	Copias:i32,
struct Prestamo{
	libro:Libro,
	cliente:Cliente,
	fecha:Fecha,
	Estado:String
}
struct Biblioteca{
	nombre:String,
	direccion:String,
	copias:Vec<Libro>
	prestamos:Vec<Prestamo>

impl Biblioteca{
	fn new(nombre:String,direccion:String) -> Biblioteca{
		{
			nombre,
			direccion,
			self.copias = Vec::new(),,
			self.prestamos = Vec::new(),
		}
		
	}
	fn obtener_cantidad(&mut self, libro:Libro) -> i32{
		let mut copias = 0;
		for i in 0..self.copias.size(){
			if self.copias[i] == libro{ copias=self.copias(i).cantidad;}
		}
		copias
	}
	fn decrementar(&mut self,libro:Libro){
		for i in 0..self.copias.size(){
			if self.copias[i] == libro{ self.copias(i).Copias=self.copias(i).Copias-1; break;}
		}
	}
	fn incrementar(&mut self,libro:Libro){
		for i in 0..self.copias.size(){
			if self.copias[i] == libro{self.copias(i).Copias=self.copias(i).Copias+1; break;}
	}
	fn prestamos(&self,cliente:Cliente){
		let mut cant = 0;
		for i in 0..self.prestamos.size(){
			if self.prestamos[i].cliente ==cliente{ cant = cant+1;}
		}
	}
	fn prestamo(&mut self,cliente:Cliente,libro:Libro){
		for i in 0..self.copias.size(){
			if self.copias[i] == libro && self.copias[i].Copias > 0{
				if self.prestamos(cliente) < 5{
						self.copias[i].Copias = self.copias[i] - 1;
						prestamo = Prestamo::new(libro,fecha,cliente,self.copias[i].Estado);
						self.prestamos.push(prestamo);
						break;
				}
			}
		}
	
						
#[test]
fn test() {
}

