fn main() {
    let arreglo1 = [2,4,6,8,10];
    let arreglo2 = [1,3,5,7,9];
    let mut arreglo3:[i32;5] = [0;5];
    for i in 0..5{
	arreglo3[i] = arreglo2[i] + arreglo1[i]; 
	}
    println!("{:?}",arreglo3);
}
