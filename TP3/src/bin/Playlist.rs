#[derive(PartialEq,Clone)]
enum Genero{
	ROCK,
	RAP,
	POP,
	JAZZ,
	OTROS
}
#[derive(PartialEq,Clone)]
struct Cancion{
	titulo:String,
	artista:String,
	genero:Option<Genero>,
}
impl Cancion{
	fn new(titulo:String,artista:String,genero:Option<Genero>){
		Cancion{
			titulo,
			artista,
			genero,
			}
		}
	}


struct Playlist{
	nombre:Option<String>,
	Canciones:Vec<Cancion>

}
impl Playlist{
	fn new(nombre:Option<String>) -> Playlist{
		Playlist{
			nombre,
			Canciones:Vec::new(),
		}
		
	}
	fn agregar_cancion(&mut self, cancion:Cancion){
		self.Canciones.push(cancion);
	}
	fn eliminar_cancion(&mut self,cancion:Cancion){
		let j = self.Canciones.len();
		let mut i = 0;
		while i < j{
			if self.Canciones[i] == cancion{ self.Canciones.remove(i); i = j-1;}
			i = i+1;
		}  
	}
	fn mover_cancion(&mut self,cancion:Cancion,pos:usize){
		let victima:&Cancion = &self.Canciones[pos];
		let j = self.Canciones.len();
		let mut i = 0;
		while i<j{
			if self.Canciones[i] == cancion{self.Canciones[pos] = self.Canciones[i].clone() ; self.Canciones[i] = victima.clone(); i=j-1;}
			i = i+1;
			} 
	}
	fn obtener_canciones(&mut self,genero:Genero) -> Vec<Cancion>{
		let mut lista:Vec<Cancion> = Vec::new();
		for i in 0..self.Canciones.len(){
			if self.Canciones[i].genero.clone().unwrap() == genero{lista.push(self.Canciones[i].clone());}
			}
		lista
		}
	fn obtener_artista(&mut self,artista:String) -> Vec<Cancion>{
		let mut lista:Vec<Cancion> = Vec::new();
		for i in 0..self.Canciones.len(){	
			if self.Canciones[i].artista == artista{lista.push(self.Canciones[i]);}
			}
		lista
	}
	fn modificar_titulo(&mut self, titulo:String){
		self.nombre = Some(titulo);
	}
	fn eliminar_canciones(&mut self){
		self.Canciones.clear();}

	}

#[test]
fn test_new(){
	let playlist = Playlist::new(Some("Playlist".to_string()));
	assert_eq!(playlist.nombre,Some("Playlist"));
}
