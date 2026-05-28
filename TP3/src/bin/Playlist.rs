#[derive(PartialEq,Clone,Debug)]
enum Genero{
	ROCK,
	RAP,
	POP,
	JAZZ,
	OTROS
}
#[derive(PartialEq,Clone,Debug)]
struct Cancion{
	titulo:Option<String>,
	artista:Option<String>,
	genero:Option<Genero>,
}
impl Cancion{
	fn new(titulo:Option<String>,artista:Option<String>,genero:Option<Genero>) -> Cancion{
		Cancion{
			titulo: titulo,
			artista: artista,
			genero: genero,
			}
		}
}
#[derive(PartialEq,Clone)]
struct Playlist{
	nombre:Option<String>,
	Canciones:Vec<Cancion>
}
impl Playlist{
	fn new(nombre:Option<String>) -> Playlist{
		Playlist{
			nombre:nombre,
			Canciones:Vec::new(),
		}
		
	}
	fn agregar_cancion(&mut self, cancion:Option<Cancion>){
		if cancion.is_some(){
			self.Canciones.push(cancion.unwrap());
		}
	}
	fn eliminar_cancion(&mut self,cancion:Option<Cancion>){
		if cancion.is_some(){
			for i in 0..self.Canciones.len(){
				if self.Canciones[i] == cancion.clone().unwrap(){ 
						self.Canciones.remove(i);
						break;
						}
					}
		}
	}	
	fn mover_cancion(&mut self,cancion:Option<Cancion>,pos:Option<usize>){
		if cancion.is_some() && pos.is_some(){
		for i in 0..self.Canciones.len(){
			if self.Canciones[i] == cancion.clone().unwrap() { 
				let temp_1 = self.Canciones[i].clone(); 
				let temp_2 = self.Canciones[pos.unwrap()].clone();
				self.Canciones[pos.unwrap()] = temp_1; 
				self.Canciones[i] = temp_2; break}}
		}
	}
	fn obtener_canciones(self,genero:Option<Genero>) -> Vec<Cancion>{
		let mut lista:Vec<Cancion> = Vec::new();
		for i in 0..self.Canciones.len(){
			if self.Canciones[i].genero.clone().unwrap() == genero.clone().unwrap(){lista.push(self.Canciones[i].clone());}
			}
		lista
		}
	fn obtener_artista(&mut self,artista:Option<String>) -> Vec<Cancion>{
		let mut lista:Vec<Cancion> = Vec::new();
		for i in 0..self.Canciones.len(){	
			if self.Canciones[i].artista == artista{lista.push(self.Canciones[i].clone());}
			}
		lista
	}
	fn modificar_titulo(&mut self, titulo:Option<String>){
		if titulo.is_some(){
			self.nombre = titulo;}
	}
	fn eliminar_canciones(&mut self){
		self.Canciones.clear();
		}

}
#[test]
fn test_new(){
	let playlist = Playlist::new(Some("Playlist".to_string()));
	assert_eq!(playlist.nombre,Some("Playlist".to_string()));
}
#[test]
fn test_genero(){
	let genero = Genero::ROCK;
	assert_eq!(genero,Genero::ROCK);
}
#[should_panic="Genero no reconocido en el sistema"]
#[test]
fn test_genero_fallido(){
	let genero = "Extratone";
	if (genero != "POP") && (genero != "Rock") && (genero != "RAP") && (genero != "JAZZ") && (genero != "OTROS"){
		panic!("Genero no reconocido en el sistema");
}
}
#[should_panic="Error, datos insuficientes"]
#[test]
fn playlist_invalida(){
	let playlist = Playlist::new(None);
	if playlist.nombre == None{
		panic!("Error, datos insuficientes");
	}
}
#[test]
fn new_cancion(){
	let cancion = Cancion::new(Some("Cancion".to_string()),Some("Pedro".to_string()),Some(Genero::ROCK));
	assert_eq!(cancion.titulo,Some("Cancion".to_string()));
	assert_eq!(cancion.artista,Some("Pedro".to_string()));
	assert_eq!(cancion.genero,Some(Genero::ROCK));
}
#[test]
fn test_agregar_cancion(){
	let mut playlist = Playlist::new(Some("Playlist".to_string()));
	let cancion = Cancion::new(Some("Cancion".to_string()),Some("Pedro".to_string()),Some(Genero::ROCK));
	playlist.agregar_cancion(Some(cancion.clone()));
	assert_eq!(playlist.Canciones.len(),1);
}
#[test]
fn test_agregar_cancion_vacia(){
	let mut playlist = Playlist::new(Some("Playlist".to_string()));
	playlist.agregar_cancion(None);
	assert_eq!(playlist.Canciones.len(),0);
}
#[test]
fn test_eliminar_cancion_existente_amplia(){
	let mut playlist = Playlist::new(Some("Playlist".to_string()));
	let cancion = Cancion::new(Some("Cancion".to_string()),Some("Pedro".to_string()),Some(Genero::ROCK));
	let cancion_2 = Cancion::new(Some("Cancion2".to_string()),Some("Pablo".to_string()),Some(Genero::JAZZ));
	playlist.agregar_cancion(Some(cancion.clone()));
	playlist.agregar_cancion(Some(cancion_2.clone()));
	playlist.eliminar_cancion(Some(cancion.clone()));
	assert_eq!(playlist.Canciones[0],cancion_2);
}

#[test]
fn test_eliminar_cancion_existente_unica(){
	let mut playlist = Playlist::new(Some("Playlist".to_string()));
	let cancion = Cancion::new(Some("Cancion".to_string()),Some("Pedro".to_string()),Some(Genero::ROCK));
	playlist.agregar_cancion(Some(cancion.clone()));
	playlist.eliminar_cancion(Some(cancion.clone()));
	assert_eq!(playlist.Canciones.len(),0);
}
#[test]
fn test_eliminar_cancion_inexistente(){
	let mut playlist = Playlist::new(Some("Playlist".to_string()));
	let cancion = Cancion::new(Some("Cancion".to_string()),Some("Pedro".to_string()),Some(Genero::ROCK));
	assert_eq!(playlist.Canciones.len(),0);
	playlist.eliminar_cancion(Some(cancion.clone()));
	assert_eq!(playlist.Canciones.len(),0);
}
#[test]
fn mover_cancion(){
	let mut playlist = Playlist::new(Some("Playlist".to_string()));
	let cancion = Cancion::new(Some("Cancion".to_string()),Some("Pedro".to_string()),Some(Genero::ROCK));
	let cancion_2 = Cancion::new(Some("Cancion2".to_string()),Some("Pablo".to_string()),Some(Genero::JAZZ));
	playlist.agregar_cancion(Some(cancion.clone()));
	playlist.agregar_cancion(Some(cancion_2.clone()));
	playlist.mover_cancion(Some(cancion.clone()),Some(1));
	assert_eq!(playlist.Canciones[1],cancion);
}

#[test]
fn mover_vacia(){
	let mut playlist = Playlist::new(Some("Pow".to_string()));
	let cancion = Cancion::new(Some("Cancion".to_string()),Some("Pedro".to_string()),Some(Genero::ROCK));
	playlist.mover_cancion(Some(cancion),Some(0));
	assert_eq!(playlist.Canciones.len(),0);
}
#[test]
fn mover_invalido(){
	let mut playlist = Playlist::new(Some("Pow".to_string()));
	assert_eq!(playlist.Canciones.len(),0);
	playlist.mover_cancion(None,None);
	assert_eq!(playlist.Canciones.len(),0);
}


#[test]
fn obtener_genero(){
	let mut playlist = Playlist::new(Some("Pow".to_string()));
	let cancion = Cancion::new(Some("Cancion".to_string()),Some("Pedro".to_string()),Some(Genero::ROCK));
	let cancion_2 = Cancion::new(Some("Cancion-2".to_string()),Some("Pedro".to_string()),Some(Genero::ROCK));
	playlist.agregar_cancion(Some(cancion.clone()));
	playlist.agregar_cancion(Some(cancion_2.clone()));
	let canciones_rock = playlist.obtener_canciones(Some(Genero::ROCK));
	assert_eq!(canciones_rock.len(),2);	
}

#[test]
fn obtener_genero_inexistente(){
	let mut playlist = Playlist::new(Some("Pow".to_string()));
	let cancion = Cancion::new(Some("Cancion".to_string()),Some("Pedro".to_string()),Some(Genero::ROCK));
	let cancion_2 = Cancion::new(Some("Cancion-2".to_string()),Some("Pedro".to_string()),Some(Genero::ROCK));
	playlist.agregar_cancion(Some(cancion.clone()));
	playlist.agregar_cancion(Some(cancion_2.clone()));
	let canciones_pop = playlist.obtener_canciones(Some(Genero::POP));
	assert_eq!(canciones_pop.len(),0);
}

#[test]
fn obtener_genero_vacia(){
	let playlist = Playlist::new(Some("Pow".to_string()));
	let canciones_rock = playlist.obtener_canciones(Some(Genero::ROCK));
	assert_eq!(canciones_rock.len(),0);
}

#[test]

fn obtener_artista(){
	let mut playlist = Playlist::new(Some("Pow".to_string()));
	let cancion = Cancion::new(Some("Cancion".to_string()),Some("Pedro".to_string()),Some(Genero::ROCK));
	let cancion_2 = Cancion::new(Some("Cancion-2".to_string()),Some("Pedro".to_string()),Some(Genero::ROCK));
	playlist.agregar_cancion(Some(cancion.clone()));
	playlist.agregar_cancion(Some(cancion_2.clone()));
	let artista = playlist.obtener_artista(Some("Pedro".to_string()));
	assert_eq!(artista.len(),2);
}

#[test]
fn obtener_artista_inexistente(){
	let mut playlist = Playlist::new(Some("Pow".to_string()));
	let cancion = Cancion::new(Some("Cancion".to_string()),Some("Pedro".to_string()),Some(Genero::ROCK));
	let cancion_2 = Cancion::new(Some("Cancion-2".to_string()),Some("Pedro".to_string()),Some(Genero::ROCK));
	playlist.agregar_cancion(Some(cancion.clone()));
	playlist.agregar_cancion(Some(cancion_2.clone()));
	let artista = playlist.obtener_artista(Some("Pablo".to_string()));
	assert_eq!(artista.len(),0);
}

#[test]
fn obtener_artista_vacia(){
	let mut playlist = Playlist::new(Some("Pow".to_string()));
	let artista = playlist.obtener_artista(Some("Pedro".to_string()));
	assert_eq!(artista.len(),0);
}
#[test]
fn modificar_titulo(){
	let mut playlist = Playlist::new(Some("Playlist".to_string()));
	assert_eq!(playlist.nombre,Some("Playlist".to_string()));
	playlist.modificar_titulo(Some("Pow".to_string()));
	assert_eq!(playlist.nombre,Some("Pow".to_string()));
}

#[test]
fn eliminar_canciones(){
	let mut playlist = Playlist::new(Some("Playlist".to_string()));
	let cancion = Cancion::new(Some("Cancion".to_string()),Some("Pedro".to_string()),Some(Genero::ROCK));
	let cancion_2 = Cancion::new(Some("Cancion_2".to_string()),Some("Pedro".to_string()),Some(Genero::ROCK));
	playlist.agregar_cancion(Some(cancion));
	playlist.agregar_cancion(Some(cancion_2));
	assert_eq!(playlist.Canciones.len(),2);
	playlist.eliminar_canciones();
	assert_eq!(playlist.Canciones.len(),0);
}
