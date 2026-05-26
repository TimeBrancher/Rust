#[derive(Copy,Clone)]
struct Fecha{
	dia:i32,
	mes:i32,
	año:i32,

}

impl Fecha{
	fn new(dia:i32,mes:i32,año:i32) -> Fecha{
		Fecha{
			dia,
			mes,
			año,
	}
	}
	fn es_valido(&self) -> bool{
		let mut dia_valido = false;
		let mes_valido = self.mes > 0 && self.mes < 13;
		let año_valido = self.año > 0 && self.año < 2100;
		match self.mes{
			11|4|6|8 => dia_valido=self.dia > 0 && self.dia <=31,	
			1|3|5|7|9|10|12 => dia_valido=self.dia > 0 && self.dia <=30,
			2 => if self.es_bisiesto(){dia_valido=self.dia > 0 && self.dia <=29;} else {dia_valido=self.dia > 0 && self.dia<=28},
			_ => dia_valido=false,
			}
		dia_valido && mes_valido && año_valido
		}

	fn es_bisiesto(self) -> bool{
		if self.año % 100 ==0{ self.año%4 == 0 && self.año%400 == 0}
		else{ self.año%4==0} 
	}
	fn sumar_dias(&mut self, dias:i32){
		self.dia = self.dia+dias;
		if self.dia > 31{
			self.dia = self.dia-31;
			self.mes = self.mes + 1;
			if self.mes > 12{
				self.mes = 1;
				self.año = self.año+1;
			}
		}
				
		}
	fn restar_dias(&mut self, dias:i32){
		self.dia = self.dia - dias;
		if self.dia <= 0{
			self.dia = self.dia+31;
			self.mes = self.mes - 1;
			if self.mes < 1{
				self.mes = 12;
				self.año = self.año-1;
			}
		}
		}
	
	fn es_mayor(self,f:Fecha)-> bool{
		if self.año > f.año{return true;}
		if self.año < f.año{return false;}
		else{	
			if self.mes > f.mes{return true;}
			if self.mes < f.mes{return false;}
			else{
				if self.dia > f.dia{return true;}
				else {return false};
			}
		}
	
	}
	
}


#[test]
fn test_valido_dia(){
	let fecha = Fecha::new(4,3,2005);
	assert_eq!(fecha.es_valido(),true);
	let fecha_alta = Fecha::new(32,7,2005);
	assert_eq!(fecha_alta.es_valido(),false);
	let fecha_baja = Fecha::new(0,7,2005);
	assert_eq!(fecha_baja.es_valido(),false);
	let fecha_bisiesto = Fecha::new(29,2,2012);
	assert_eq!(fecha_bisiesto.es_valido(),true);
	let fecha_falso_bisiesto = Fecha::new(29,2,2005);
	assert_eq!(fecha_falso_bisiesto.es_valido(),false);
}
#[test]
fn test_mes_valido(){
	let fecha = Fecha::new(3,2,2005);
	assert_eq!(fecha.es_valido(),true);
	let fecha_baja = Fecha::new(3,0,2005);
	assert_eq!(fecha_baja.es_valido(),false);
	let fecha_alta = Fecha::new(3,13,2005);
	assert_eq!(fecha_alta.es_valido(),false);
}

#[test]
fn test_año_valido(){	
	let fecha = Fecha::new(3,2,2005);
	assert_eq!(fecha.es_valido(),true);
	let fecha_baja = Fecha::new(3,2,0);
	assert_eq!(fecha_baja.es_valido(),false);
	let fecha_alta = Fecha::new(3,2,2100);
	assert_eq!(fecha_alta.es_valido(),false);
}
#[test]
fn test_bisiesto(){
	let fecha = Fecha::new(3,5,2005);
	assert_eq!(fecha.es_bisiesto(), false);
	let fecha_bisiesto = Fecha::new(3,5,2012);
	assert_eq!(fecha_bisiesto.es_bisiesto(),true);
}
#[test]
fn test_sumar_dias(){
	let mut fecha = Fecha::new(31,12,1999);
	fecha.sumar_dias(1);
	assert_eq!(fecha.dia,1);
	assert_eq!(fecha.mes,1);
	assert_eq!(fecha.año,2000);
}

#[test]
fn test_restar_dias(){
	let mut fecha = Fecha::new(1,1,2000);
	fecha.restar_dias(1);
	assert_eq!(fecha.dia,31);
	assert_eq!(fecha.mes,12);
	assert_eq!(fecha.año,1999);
}

#[test]
fn test_mayor(){
	let fecha_base = Fecha::new(5,7,2005);
	let mayor_dia = Fecha::new(4,7,2005);
	let mayor_mes = Fecha::new(5,6,2005);
	let mayor_año = Fecha::new(5,7,2004);
	let menor_dia = Fecha::new(6,7,2005);
	let menor_mes = Fecha::new(5,8,2005);
	let menor_año = Fecha::new(5,7,2006);
	assert_eq!(fecha_base.es_mayor(mayor_dia),true);
	assert_eq!(fecha_base.es_mayor(mayor_mes),true);
	assert_eq!(fecha_base.es_mayor(mayor_año),true);
	assert_eq!(fecha_base.es_mayor(menor_dia),false);
	assert_eq!(fecha_base.es_mayor(menor_mes),false);
	assert_eq!(fecha_base.es_mayor(menor_año),false);
}
