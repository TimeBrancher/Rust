struct Examen{
	materia:String,
	nota:f64,
}

struct Alumno{
	nombre:String,
	Num_ID:i32,
	Calificacion:Vec<Examen>,

}
impl Examen{
	fn new(materia:String, nota:i32) -> Examen{
		materia,
		nota,
	}	
}
impl ALumno{
	fn new(nombre:String,edad:i32,examen:Examen) -> Alumno{
		nombre,
		edad,
		Self.Calificacion.push(examen
		
	}
	fn obtener_promedio(self) -> f64{
		let mut cantidad = Self.Calificacion.len();
		let total = 0.0;
		for i in 0..cantidad{
			total = total + Self.Calificacion(i).nota;
		}
		total/cantidad
	}
	fn obtener_calificacion_mas_alta(self) -> f64{
		let mut max = 0.0;
		for i in Self.Calificacion.len(){
			if Self.Calificacion(i).nota > max{
				max = Self.Calificacion(i).nota;
			}
		max
	}
	fn obtener_calificacion_mas_baja(self) -> f64{
		let mut min = 9999.9;
		for i in Self.Calificacion.len(){
			if Self.Calificacion(i).nota < min{
				min = Self.Calificacion(i).nota;
			}
		min
	}

}

fn main(){}
