struct Examen{
	materia:Option<String>,
	nota:Option<f64>,
}

struct Alumno{
	nombre:Option<String>,
	num_id:Option<i32>,
	calificacion:Vec<Examen>,

}
impl Examen{
	fn new(materia:Option<String>, nota:Option<f64>) -> Examen{
		Examen{
			materia,
			nota,
		}
	}	
}
impl Alumno{
	fn new(nombre:Option<String>,num_id:Option<i32>) -> Alumno{
		Alumno{
			nombre,
			num_id,
			calificacion: Vec::new(),
			}
		
	}
	fn obtener_promedio(&self) -> Option<f64>{
		let cantidad = self.calificacion.len();
		let mut total:f64 = 0.0;
		for i in 0..cantidad{
			if self.calificacion[i].nota.is_some(){
				total = total + self.calificacion[i].nota.unwrap();
			}
		}
		if total==0.0{ None}
		else{
			let cant:f64 = cantidad as f64;
		Some(total/cant)}
	}
	fn obtener_calificacion_mas_alta(&self) -> Option<f64>{
		let mut max:Option<f64> = None;
		for i in 0..self.calificacion.len(){
			if self.calificacion[i].nota.is_some(){ 
				if max.is_some(){
						if self.calificacion[i].nota.unwrap() > max.unwrap(){ max = self.calificacion[i].nota;}}
				 else {
					max=self.calificacion[i].nota;}
			}
		}
				
		max
	}
	fn obtener_calificacion_mas_baja(&self) -> Option<f64>{
		let mut min:Option<f64> = None ;
		for i in 0..self.calificacion.len(){
			if self.calificacion[i].nota.is_some(){
				if min.is_some(){
					if self.calificacion[i].nota.unwrap() < min.unwrap(){ min = self.calificacion[i].nota;}}
				else{
					min=self.calificacion[i].nota;}
			}
	}
		min
	}

}
#[test]
fn test_examen(){
	let examen = Examen::new(Some("rust".to_string()), Some(5.0));}

#[should_panic="error,datos insuficientes"]
#[test]
fn test_examen_incompleto(){
	let examen = Examen::new(None,Some(5.0));
	let examen_2 = Examen::new(Some("AYED".to_string()),None);
	if (examen.materia == None || examen.nota == None) && (examen_2.materia == None || examen_2.nota == None){
		panic!("error,datos insuficientes");}
}
#[test]
fn test_promedio(){
	let mut alumno = Alumno::new(Some("Pedro".to_string()),Some(17));
	let examen = Examen::new(Some("AYED".to_string()),Some(7.0));
	let examen_2 = Examen::new(Some("rust".to_string()),Some(5.0));
	alumno.calificacion.push(examen);
	alumno.calificacion.push(examen_2);
	assert_eq!(alumno.obtener_promedio(),Some(6.0));
}

#[should_panic="error,datos insuficientes"]
#[test]
fn test_alumno_incompleto(){
	let alumno = Alumno::new(None,Some(17));
	let alumno_2 = Alumno::new(Some("Pedro".to_string()),None);
	if (alumno.nombre == None || alumno.num_id == None) || (alumno_2.nombre == None || alumno_2.num_id == None){
		panic!("error,datos insuficientes");
	}
}
#[test]
fn test_obtener_calificacion_alta(){
	let examen = Examen::new(Some("rust".to_string()),Some(5.0));
	let examen_2 = Examen::new(Some("AYED".to_string()),Some(6.0));
	let mut alumno = Alumno::new(Some("Pedro".to_string()),Some(17));
	alumno.calificacion.push(examen);
	alumno.calificacion.push(examen_2);
	assert_eq!(alumno.obtener_calificacion_mas_alta(),Some(6.0));
}

#[test]
fn test_obtener_calificacion_baja(){
	let examen = Examen::new(Some("rust".to_string()),Some(5.0));
	let examen_2 = Examen::new(Some("AYED".to_string()),Some(6.0));
	let mut alumno = Alumno::new(Some("Pedro".to_string()),Some(17));
	alumno.calificacion.push(examen);
	alumno.calificacion.push(examen_2);
	assert_eq!(alumno.obtener_calificacion_mas_baja(),Some(5.0));
}
