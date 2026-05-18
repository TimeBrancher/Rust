enum Genero{
	ROCK,
	RAP,
	POP,
	JAZZ,
	OTROS
}
struct Cancion{
	titulo:String,
	artista:String,
	genero:Genero,
}

struct Playlist{
	nombre:String,
	Canciones:Vec<Cancion>

}
impl Playlist{
	fn new(nombre:String,cancion:Cancion) -> Playlist{
		Playlist{
			nombre,
			cancion,
		}
		
	}
	fn agregar_cancion(&mut self, cancion){
		self.canciones.push(cancion);
	}
	fn eliminar_cancion(&mut self,cancion){
		let j = Self.Canciones.len();
		let mut i = 0;
		while i < j{
			if Self.Canciones(i) == cancion{ Self.Canciones.remove(i); i = j-1;}
			i = i+1;
		}  
	}
	fn mover_cancion(&mut self,cancion,pos){
		let mut victima:Cancion = Self.Canciones(pos);
		let j = Self.Canciones.len();
		let mut i = 0;
		while i<j{
			if (Self.Canciones(i) == cancion){Self.Canciones(pos) = Self.Canciones(i) ; Self.Canciones(i) = victima; i=j-1;}
			i = i+1;
			} 
	}
	fn obtener_canciones(&mut self,genero){
		for i in 0..Self.Cancion.len(){
			if (Self.Canciones(i).genero == genero){println!("{}",Self.Canciones(i));}
			}
		}
	fn obtener_artista(&mut self,artista){
		for i in 0..Self.Cancion.len(){	
			if (Self.Canciones(i).artista == artista){
				println!("{}",Self.Canciones(i));
			}
		}
	fn modificar_titulo(&mut self, titulo){
		Self.nombre = titulo;
	}
	fn eliminar_canciones(&mut self){
		self.Canciones.clear();}

}

fn main(){}
