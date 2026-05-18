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
	fn es_valido(self) -> bool{
		dia_valido = false;
		mes_valido = Self.mes > 1 && Self.mes <= 12;
		año_valido = Self.año > 0 && Self.año < 2100;
		match self.mes{
			11,4,6,8 => dia_valido=self.dia > 0 && self.dia <=31,	
			1,3,5,7,9,10,12 => dia_valido=self.dia > 0 && self.dia <=30,
			2 => if es_bisiesto(Self.año){dia_valido=self.dia > 0 && self.dia =<29;} else {dia_valido=self.dia > 0 && self.dia<28},
			}
		dia_valido & mes_valido & año_valido
		}

	fn es_bisiesto(self) -> bool{
		self.año%4 == 0 && !(self.año%100 != 0) && !(self.año%400==0)
	}
	fn sumarDias(&mut self, dias){
		self.dia = self.dia+dias;
		if (self.dia > 31){
			self.dia = self.dia-31;
			self.mes = self.mes + 1;
			if (self.mes > 12){
				self.mes = 1;
				self.año = self.año+1;
			}
		}
				
		}
	fn restarDias(&mut self, dias){
		self.dia = self.dia - dias;
		if (self.dia < 0){
			self.dia = self.dia+31;
			self.mes = self.mes - 1;
			if self.mes < 1{
				self.mes = 12;
				self.año = self.año-1;
			}
		}
	}
	
	fn esMayor(F:Fecha)-> bool{
		if (self.año > F.año){return true;}
		if (self.año < F.año){return false;}
		else{	
			if (self.mes > F.mes){return true;}
			if (self.mes < F.mes){return false;}
			else{
				if (self.dia > F.dia){return true;}
				else return false;
			}
		}
	
	}
	
	fn actualizardireccion(&mut self, direccion:String){
		self.direccion=direccion;
	}
	

}

fn main(){}

