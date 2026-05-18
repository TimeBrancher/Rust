struct Auto{
	marca:String,
	modelo:String,
	año:i32,
	precio:f64,
	color:String
}

struct ConcesionarioAuto{
	nombre:String,
	direccion:String,
	Autos:Vec<Auto>,

}
impl Auto{
	fn new(marca:String, modelo:String,año:i32,precio:f64,color:String) -> Auto{
		marca,
		modelo,
		año,
		precio,
		color
	}
	fn calcular_precio(&mut Self) -> i32{
		let mut Precio = Self.precio;
		match Self.color{
			Self.color == "Rojo" || Self.color == "Amarillo" || Self.color == "Azul" => Precio = Precio * 1.25,
			__ => Precio=Precio*0.9,
		}
		if (Self.marca == "BMW"){
			Precio = Precio * 1.15;}
		if (Self.año > 2000){
			Self.precio = Precio * 0.95;
			}	
}
impl Concesionario{
	fn new(nombre:String,edad:i32,Au:Auto) -> Self{
		Auto{
			nombre,
			edad,
			Self.Autos.push(Au),
		}
		
	}
	fn agregar_auto(&mut Self, auto:Auto){
		Self.Autos.push(auto);
	}
	fn eliminar_auto(&mut Self, auto:Auto){
		Self.Autos.remove(auto);
	}
	fn buscar_auto(&mut Self, auto:Auto){
		for i in 0..Self.Autos.len(){
			if Self.Autos(i) == auto{
				Self.Autos.remove(i);
			}
		}	
	}

}

fn main(){}
