mod fecha;
use fecha::Fecha;
#[derive(PartialEq,Clone,Debug)]
enum Genero{
	NOVELA,
	INFANTIL,
	TECNICO,
	OTROS,
}
#[derive(PartialEq,Clone,Debug)]
struct Cliente{
	nombre:Option<String>,
	direccion:Option<String>,
	telefono:Option<String>,
	correo:Option<String>
}
impl Cliente{
	fn new(nombre:Option<String>,direccion:Option<String>,telefono:Option<String>,correo:Option<String>) -> Cliente{
		Cliente{
			nombre,
			direccion,
			telefono,
			correo,
		}
	}
}
#[derive(PartialEq,Clone,Debug)]
enum Estados{
	DEVUELTO,
	En_Prestamo,
}
#[derive(PartialEq,Clone,Debug)]
struct Libro{
	ISBN:Option<String>,
	Titulo:Option<String>,
	Autor:Option<String>,
	Numero_Paginas:Option<i32>,
	genero:Option<Genero>,
	Copias:Option<i32>,
}
impl Libro{
	fn new(ISBN:Option<String>,Titulo:Option<String>,Autor:Option<String>,Numero_Paginas:Option<i32>,genero:Option<Genero>,Copias:Option<i32>) -> Libro{
		Libro{
			ISBN,
			Titulo,
			Autor,
			Numero_Paginas,
			genero,
			Copias,	
		}
	}
}
#[derive(PartialEq,Clone,Debug)]
struct Prestamo{
	libro:Option<Libro>,
	cliente:Option<Cliente>,
	fecha_devolucion:Option<Fecha>,	
	fecha_prestamo:Option<Fecha>,
	estado:Option<Estados>,
}
impl Prestamo{
	fn new(libro:Option<Libro>,cliente:Option<Cliente>,fecha_devolucion:Option<Fecha>,fecha_prestamo:Option<Fecha>,estado:Option<Estados>) -> Prestamo{
		Prestamo{
			libro,
			cliente,
			fecha_devolucion,
			fecha_prestamo,
			estado,
		}
	}
}
#[derive(PartialEq)]
struct Biblioteca{
	nombre:Option<String>,
	direccion:Option<String>,
	copias:Vec<Libro>,
	prestamos:Vec<Prestamo>,
}
impl Biblioteca{
	fn new(nombre:Option<String>,direccion:Option<String>) -> Biblioteca{
		Biblioteca{
			nombre,
			direccion,
			copias:Vec::new(),
			prestamos:Vec::new(),
		}
		
	}
	fn obtener_cantidad(&mut self, libro:Libro) -> i32{
		let mut copias = 0;
		for i in 0..self.copias.len(){
			if self.copias[i].ISBN.clone().unwrap() == libro.ISBN.clone().unwrap() && self.copias[i].Copias.is_some(){ copias=self.copias[i].Copias.unwrap();}
		}
		copias
	}
	fn decrementar(&mut self,libro:Libro){
		for i in 0..self.copias.len(){
			if self.copias[i] == libro && self.copias[i].Copias.is_some() && self.copias[i].Copias.unwrap() > 0{ self.copias[i].Copias=Some(self.copias[i].Copias.unwrap()-1); break;}
		}
	}
	fn incrementar(&mut self,libro:Libro){
		for i in 0..self.copias.len(){
			if self.copias[i] == libro{self.copias[i].Copias=Some(self.copias[i].Copias.unwrap()+1); break;}
		}
	}
	fn prestamos_cliente(&self,cliente:Cliente) -> i32{
		let mut cant = 0;
		for i in 0..self.prestamos.len(){
			if self.prestamos[i].cliente.clone().unwrap() ==cliente{ cant = cant+1;}
		}
		cant
	}
	fn prestar_libro(&mut self,cliente:Option<Cliente>,libro:Option<Libro>) ->bool{
		let mut hecho = false;
		let fecha = Fecha::new(6,10,2027);
		for i in 0..self.copias.len(){
			if self.copias[i] == libro.clone().unwrap(){
				if self.cantidad_prestamos(cliente.clone()) < 5 && self.copias[i].Copias.unwrap() > 0 {
						self.copias[i].Copias = Some(self.copias[i].Copias.unwrap() - 1);
						let prestamo = Prestamo::new(libro.clone(),cliente.clone(),None,Some(fecha),Some(Estados::En_Prestamo));
						hecho = true;
						self.prestamos.push(prestamo);
				}
			}
		}
		hecho
	}
	
	fn prestamos_a_vencer(&mut self, dias:f64,F:Fecha) -> Vec<Prestamo>{
		let mut vector:Vec<Prestamo> = Vec::new();
		for i in 0..self.prestamos.len(){
			let restantes = (self.prestamos[i].fecha_prestamo.unwrap().año - F.año) as f64 + ((self.prestamos[i].fecha_prestamo.unwrap().mes - F.mes) as f64)/12.0 + ((self.prestamos[i].fecha_prestamo.unwrap().dia - F.dia) as f64)/360.0;
			if restantes*360.0 >= dias.into(){
				vector.push(self.prestamos[i].clone());
				}
			}
		return vector;
		}
	fn prestamos_vencidos(&mut self, dias:i32,F:Fecha) -> Vec<Prestamo>{
		let mut vector:Vec<Prestamo> = Vec::new();
		for i in 0..self.prestamos.len(){
			let restantes = (self.prestamos[i].fecha_prestamo.unwrap().año - F.año) as f64 + ((self.prestamos[i].fecha_prestamo.unwrap().mes + F.mes) as f64)/12.0 + ((self.prestamos[i].fecha_prestamo.unwrap().dia + F.dia) as f64)/360.0;
			if restantes*360.0 >= dias.into(){
				vector.push(self.prestamos[i].clone());
			}
		}
		return vector;
	}
	fn buscar_prestamos(&mut self, C:Option<Cliente>,L:Option<Libro>) -> Option<Prestamo>{
		let mut Prestamo:Option<Prestamo> = None;
		if C.is_some() && L.is_some(){
			for i in 0..self.prestamos.len(){
				if self.prestamos[i].libro.clone().unwrap() == L.clone().unwrap() && self.prestamos[i].cliente.clone().unwrap() == C.clone().unwrap() { Prestamo = Some(self.prestamos[i].clone());}
				}
			}
			Prestamo
		}
	fn cantidad_prestamos(&mut self, C:Option<Cliente>) ->i32{
		let mut cantidad = 0;
		for i in 0..self.prestamos.len(){
			if self.prestamos[i].cliente.clone().unwrap().nombre == C.clone().unwrap().nombre{
				cantidad = cantidad+1;
			}
		}
		cantidad
	}
	fn devolver_libro(&mut self,C:Option<Cliente>,L:Option<Libro>){
		for i in 0..self.prestamos.len(){
			if (C.is_some() && L.is_some()){	
			if self.prestamos[i].cliente.clone().unwrap() == C.clone().unwrap() && self.prestamos[i].libro.clone().unwrap() == L.clone().unwrap() {

				self.prestamos[i].estado = Some(Estados::DEVUELTO);
				for j in 0..self.copias.len(){
					if self.copias[j] == L.clone().unwrap(){
						self.copias[j].Copias = Some(self.copias[j].Copias.unwrap()+1);
						break;}
					}
				break;
				}
			}
		}
	}
}
#[test]
fn test_cliente_full(){
	let cliente = Cliente::new(Some("Pedro".to_string()),Some("Siempre viva 123".to_string()),Some("+541234567".to_string()),Some("correo@gmail.com".to_string()));
	assert_eq!(cliente.nombre, Some("Pedro".to_string()));
	assert_eq!(cliente.telefono, Some("+541234567".to_string()));
	assert_eq!(cliente.direccion, Some("Siempre viva 123".to_string()));
	assert_eq!(cliente.correo, Some("correo@gmail.com".to_string()));
}
#[should_panic="error insuficientes datos"]
#[test]
fn test_cliente_incompleto(){
	let cliente = Cliente::new(None,None,None,None);
	if cliente.nombre == None|| cliente.telefono == None || cliente.direccion == None || cliente.correo == None{ panic!("error insuficientes datos");}
}
#[test]
fn test_libro(){
	let libro = Libro::new(Some("1234".to_string()),Some("Libro".to_string()),Some("Autor".to_string()),Some(64),Some(Genero::TECNICO),Some(128));
	assert_eq!(libro.ISBN,Some("1234".to_string()));
	assert_eq!(libro.Titulo,Some("Libro".to_string()));
	assert_eq!(libro.Autor,Some("Autor".to_string()));
	assert_eq!(libro.Numero_Paginas,Some(64));
	assert_eq!(libro.genero,Some(Genero::TECNICO));
	assert_eq!(libro.Copias,Some(128));
}
#[should_panic = "error insuficientes datos"]
#[test]
fn test_libro_incompleto(){
	let libro = Libro::new(None,None,None,None,None,None);
	if libro.ISBN == None || libro.Titulo == None || libro.Autor == None || libro.Numero_Paginas == None || libro.Copias == None || libro.genero == None { panic!("error insuficientes datos");}
	}
#[test]
fn test_prestamo_full(){
	let libro = Libro::new(Some("1234".to_string()),Some("Libro".to_string()),Some("Autor".to_string()),Some(64),Some(Genero::TECNICO),Some(128));
	let cliente = Cliente::new(Some("Pedro".to_string()),Some("Siempre viva 123".to_string()),Some("+541234567".to_string()),Some("correo@gmail.com".to_string()));
	let fecha = Fecha::new(23,7,2026);
	let prestamo = Prestamo::new(Some(libro.clone()),Some(cliente.clone()),None,Some(fecha.clone()),Some(Estados::En_Prestamo));
	assert_eq!(prestamo.libro,Some(libro.clone()));
	assert_eq!(prestamo.cliente,Some(cliente.clone()));
	assert_eq!(prestamo.fecha_prestamo,Some(fecha.clone()));		
	assert_eq!(prestamo.estado,Some(Estados::En_Prestamo));
}	

#[should_panic="error insuficientes datos"]
#[test]
fn test_prestamo_incompleto(){
	let prestamo = Prestamo::new(None,None,None,None,None);
	if prestamo.libro == None || prestamo.cliente == None || prestamo.fecha_prestamo == None || prestamo.estado == None { panic!("error insuficientes datos");}
}
#[test]
fn test_biblioteca(){
	let biblioteca = Biblioteca::new(Some("Biblioteca".to_string()),Some("Elm Street".to_string()));
	assert_eq!(biblioteca.nombre,Some("Biblioteca".to_string()));
	assert_eq!(biblioteca.direccion,Some("Elm Street".to_string()));
}
#[should_panic="error insuficientes datos"]
#[test]
fn test_biblioteca_incompleta(){
	let biblioteca = Biblioteca::new(None,None);
	if biblioteca.direccion == None || biblioteca.nombre == None{ panic!("error insuficientes datos");}
}
#[test]
fn test_cantidad(){
	let mut biblioteca = Biblioteca::new(Some("Biblioteca".to_string()),Some("Elm Street".to_string()));
	let libro = Libro::new(Some("1234".to_string()),Some("Libro".to_string()),Some("Autor".to_string()),Some(64),Some(Genero::TECNICO),Some(128));
	biblioteca.copias.push(libro.clone());
	assert_eq!(biblioteca.obtener_cantidad(libro.clone()),128);
}

#[test]
fn test_cantidad_nula(){
	let mut biblioteca = Biblioteca::new(Some("Biblioteca".to_string()),Some("Elm Street".to_string()));
	let libro = Libro::new(Some("1234".to_string()),Some("Libro".to_string()),Some("Autor".to_string()),Some(64),Some(Genero::TECNICO),Some(128));
	assert_eq!(biblioteca.obtener_cantidad(libro.clone()),0);
}
#[test]
fn test_incrementar_existente(){
	let mut biblioteca = Biblioteca::new(Some("Biblioteca".to_string()),Some("Elm Street".to_string()));
	let libro = Libro::new(Some("1234".to_string()),Some("Libro".to_string()),Some("Autor".to_string()),Some(64),Some(Genero::TECNICO),Some(128));
	biblioteca.copias.push(libro.clone());
	biblioteca.incrementar(libro.clone());
	assert_eq!(biblioteca.obtener_cantidad(libro.clone()),129);
}
#[test]
fn test_incrementar_inexistente(){
	let mut biblioteca = Biblioteca::new(Some("Biblioteca".to_string()),Some("Elm Street".to_string()));
	let libro = Libro::new(Some("1234".to_string()),Some("Libro".to_string()),Some("Autor".to_string()),Some(64),Some(Genero::TECNICO),Some(128));
	biblioteca.incrementar(libro.clone());
	assert_eq!(biblioteca.obtener_cantidad(libro),0);
}
#[test]
fn test_decrementar_existente(){
	let mut biblioteca = Biblioteca::new(Some("Biblioteca".to_string()),Some("Elm Street".to_string()));
	let libro = Libro::new(Some("1234".to_string()),Some("Libro".to_string()),Some("Autor".to_string()),Some(64),Some(Genero::TECNICO),Some(128));
	biblioteca.copias.push(libro.clone());
	biblioteca.decrementar(libro.clone());
	assert_eq!(biblioteca.obtener_cantidad(libro),127);	
}
#[test]
fn test_decrementar_inexistente(){
	let mut biblioteca = Biblioteca::new(Some("Biblioteca".to_string()),Some("Elm Street".to_string()));
	let libro = Libro::new(Some("1234".to_string()),Some("Libro".to_string()),Some("Autor".to_string()),Some(64),Some(Genero::TECNICO),Some(128));
	biblioteca.decrementar(libro.clone());
	assert_eq!(biblioteca.obtener_cantidad(libro),0);
}
#[test]
fn test_decrementar_0(){
	let mut biblioteca = Biblioteca::new(Some("Biblioteca".to_string()),Some("Elm Street".to_string()));
	let libro = Libro::new(Some("1234".to_string()),Some("Libro".to_string()),Some("Autor".to_string()),Some(64),Some(Genero::TECNICO),Some(0));
	biblioteca.copias.push(libro.clone());
	biblioteca.decrementar(libro.clone());
	assert_eq!(biblioteca.obtener_cantidad(libro),0);
}
#[test]
fn prestamo_cliente(){
	let cliente = Cliente::new(Some("Pedro".to_string()),Some("Siempre viva 1234".to_string()),Some("+541234567".to_string()),Some("correo@gmail.com".to_string()));
	let fecha = Fecha::new(29,5,2027);
	let mut biblioteca = Biblioteca::new(Some("Biblioteca".to_string()),Some("Elm Street".to_string()));
	let libro = Libro::new(Some("1234".to_string()),Some("Libro".to_string()),Some("Autor".to_string()),Some(64),Some(Genero::TECNICO),Some(128));
	let prestamo = Prestamo::new(Some(libro.clone()),Some(cliente.clone()),None,Some(fecha.clone()),Some(Estados::En_Prestamo));
	biblioteca.prestamos.push(prestamo.clone());
	assert_eq!(biblioteca.prestamos_cliente(cliente.clone()),1);
}
#[test]
fn prestamo_cliente_inexistente(){
	let cliente = Cliente::new(Some("Pedro".to_string()),Some("Siempre viva 1234".to_string()),Some("+541234567".to_string()),Some("correo@gmail.com".to_string()));
	let fecha = Fecha::new(29,5,2027);
	let biblioteca = Biblioteca::new(Some("Biblioteca".to_string()),Some("Elm Street".to_string()));
	let libro = Libro::new(Some("1234".to_string()),Some("Libro".to_string()),Some("Autor".to_string()),Some(64),Some(Genero::TECNICO),Some(128));
	let prestamo = Prestamo::new(Some(libro.clone()),Some(cliente.clone()),None,Some(fecha.clone()),Some(Estados::En_Prestamo));
	assert_eq!(biblioteca.prestamos_cliente(cliente.clone()),0);

}
#[test]
fn prestar_libro(){
	let cliente = Cliente::new(Some("Pedro".to_string()),Some("Siempre viva 1234".to_string()),Some("+541234567".to_string()),Some("correo@gmail.com".to_string()));
	let mut biblioteca = Biblioteca::new(Some("Biblioteca".to_string()),Some("Elm Street".to_string()));
	let libro = Libro::new(Some("1234".to_string()),Some("Libro".to_string()),Some("Autor".to_string()),Some(64),Some(Genero::TECNICO),Some(128));
	biblioteca.copias.push(libro.clone());
	biblioteca.prestar_libro(Some(cliente.clone()),Some(libro.clone()));
	assert_eq!(biblioteca.prestamos.len(),1);
}
#[test]
fn prestar_libro_inexistente(){
	let cliente = Cliente::new(Some("Pedro".to_string()),Some("Siempre viva 1234".to_string()),Some("+541234567".to_string()),Some("correo@gmail.com".to_string()));
	let mut biblioteca = Biblioteca::new(Some("Biblioteca".to_string()),Some("Elm Street".to_string()));
	let libro = Libro::new(Some("1234".to_string()),Some("Libro".to_string()),Some("Autor".to_string()),Some(64),Some(Genero::TECNICO),Some(128));
	biblioteca.prestar_libro(Some(cliente.clone()),Some(libro.clone()));
	assert_eq!(biblioteca.prestamos.len(),0);
}
#[test]
fn test_prestamos_a_vencer(){
	let cliente = Cliente::new(Some("Pedro".to_string()),Some("Siempre viva 1234".to_string()),Some("+541234567".to_string()),Some("correo@gmail.com".to_string()));
	let mut biblioteca = Biblioteca::new(Some("Biblioteca".to_string()),Some("Elm Street".to_string()));
	let hoy = Fecha::new(10,6,2027);
	let fecha = Fecha::new(11,6,2027);
	let libro = Libro::new(Some("1234".to_string()),Some("Libro".to_string()),Some("Autor".to_string()),Some(64),Some(Genero::TECNICO),Some(128));
	let libro_2 = Libro::new(Some("1235".to_string()),Some("Libro".to_string()),Some("Autor".to_string()),Some(64),Some(Genero::TECNICO),Some(128));
	let prestamo = Prestamo::new(Some(libro.clone()),Some(cliente.clone()),None,Some(fecha.clone()),Some(Estados::En_Prestamo));
	let prestamo_2 = Prestamo::new(Some(libro_2.clone()),Some(cliente.clone()),None,Some(fecha.clone()),Some(Estados::En_Prestamo));
	biblioteca.prestamos.push(prestamo);
	biblioteca.prestamos.push(prestamo_2);
	assert_eq!(biblioteca.prestamos_a_vencer(1.0,hoy).len(),2);
}
#[test]
fn test_prestamos_vencidos(){
	let cliente = Cliente::new(Some("Pedro".to_string()),Some("Siempre viva 1234".to_string()),Some("+541234567".to_string()),Some("correo@gmail.com".to_string()));
	let mut biblioteca = Biblioteca::new(Some("Biblioteca".to_string()),Some("Elm Street".to_string()));
	let hoy = Fecha::new(12,6,2027);
	let fecha = Fecha::new(11,6,2027);
	let libro = Libro::new(Some("1234".to_string()),Some("Libro".to_string()),Some("Autor".to_string()),Some(64),Some(Genero::TECNICO),Some(128));
	let libro_2 = Libro::new(Some("1235".to_string()),Some("Libro".to_string()),Some("Autor".to_string()),Some(64),Some(Genero::TECNICO),Some(128));
	let prestamo = Prestamo::new(Some(libro.clone()),Some(cliente.clone()),None,Some(fecha.clone()),Some(Estados::En_Prestamo));
	let prestamo_2 = Prestamo::new(Some(libro_2.clone()),Some(cliente.clone()),None,Some(fecha.clone()),Some(Estados::En_Prestamo));
	biblioteca.prestamos.push(prestamo);
	biblioteca.prestamos.push(prestamo_2);
	assert_eq!(biblioteca.prestamos_a_vencer(1.0,hoy).len(),0);
}
#[test]
fn test_buscar_prestamo(){
	let cliente = Cliente::new(Some("Pedro".to_string()),Some("Siempre viva 1234".to_string()),Some("+541234567".to_string()),Some("correo@gmail.com".to_string()));
	let mut biblioteca = Biblioteca::new(Some("Biblioteca".to_string()),Some("Elm Street".to_string()));
	let fecha = Fecha::new(11,6,2027);
	let libro = Libro::new(Some("1234".to_string()),Some("Libro".to_string()),Some("Autor".to_string()),Some(64),Some(Genero::TECNICO),Some(128));
	let prestamo = Prestamo::new(Some(libro.clone()),Some(cliente.clone()),None,Some(fecha.clone()),Some(Estados::En_Prestamo));
	biblioteca.prestamos.push(prestamo.clone());
	assert_eq!(biblioteca.buscar_prestamos(Some(cliente),Some(libro)).unwrap(),prestamo);
}
#[test]
fn test_buscar_prestamo_inexistente(){
	let cliente = Cliente::new(Some("Pedro".to_string()),Some("Siempre viva 1234".to_string()),Some("+541234567".to_string()),Some("correo@gmail.com".to_string()));
	let mut biblioteca = Biblioteca::new(Some("Biblioteca".to_string()),Some("Elm Street".to_string()));
	let fecha = Fecha::new(11,6,2027);
	let libro = Libro::new(Some("1234".to_string()),Some("Libro".to_string()),Some("Autor".to_string()),Some(64),Some(Genero::TECNICO),Some(128));
	let prestamo = Prestamo::new(Some(libro.clone()),Some(cliente.clone()),None,Some(fecha.clone()),Some(Estados::En_Prestamo));
	assert_eq!(biblioteca.buscar_prestamos(Some(cliente),Some(libro)),None);
}
#[test]
fn test_devolver_libro(){
	let cliente = Cliente::new(Some("Pedro".to_string()),Some("Siempre viva 1234".to_string()),Some("+541234567".to_string()),Some("correo@gmail.com".to_string()));
	let mut biblioteca = Biblioteca::new(Some("Biblioteca".to_string()),Some("Elm Street".to_string()));
	let fecha = Fecha::new(11,6,2027);
	let libro = Libro::new(Some("1234".to_string()),Some("Libro".to_string()),Some("Autor".to_string()),Some(64),Some(Genero::TECNICO),Some(128));
	let prestamo = Prestamo::new(Some(libro.clone()),Some(cliente.clone()),None,Some(fecha.clone()),Some(Estados::En_Prestamo));
	biblioteca.prestamos.push(prestamo.clone());
	biblioteca.devolver_libro(Some(cliente),Some(libro));
	assert_eq!(biblioteca.prestamos[0].estado,Some(Estados::DEVUELTO));
}
#[test]
fn test_devolver_libro_inexistente(){
	let cliente = Cliente::new(Some("Pedro".to_string()),Some("Siempre viva 1234".to_string()),Some("+541234567".to_string()),Some("correo@gmail.com".to_string()));
	let mut biblioteca = Biblioteca::new(Some("Biblioteca".to_string()),Some("Elm Street".to_string()));
	let fecha = Fecha::new(11,6,2027);
	let libro = Libro::new(Some("1234".to_string()),Some("Libro".to_string()),Some("Autor".to_string()),Some(64),Some(Genero::TECNICO),Some(128));
	let prestamo = Prestamo::new(Some(libro.clone()),Some(cliente.clone()),None,Some(fecha.clone()),Some(Estados::En_Prestamo));
	biblioteca.devolver_libro(Some(cliente),Some(libro));
	assert_eq!(biblioteca.prestamos.len(),0);
}
#[test]
fn test_cantidad_prestamos(){
	let cliente = Cliente::new(Some("Pedro".to_string()),Some("Siempre viva 1234".to_string()),Some("+541234567".to_string()),Some("correo@gmail.com".to_string()));
	let mut biblioteca = Biblioteca::new(Some("Biblioteca".to_string()),Some("Elm Street".to_string()));
	let fecha = Fecha::new(11,6,2027);
	let libro = Libro::new(Some("1234".to_string()),Some("Libro".to_string()),Some("Autor".to_string()),Some(64),Some(Genero::TECNICO),Some(128));
	let prestamo = Prestamo::new(Some(libro.clone()),Some(cliente.clone()),None,Some(fecha.clone()),Some(Estados::En_Prestamo));
	biblioteca.prestamos.push(prestamo.clone());
	assert_eq!(biblioteca.cantidad_prestamos(Some(cliente)),1);
}

