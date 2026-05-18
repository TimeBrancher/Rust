fn main() {
    let mut num:[i32;6] = [4,8,16,32,64,128];
    for i in 0..5{
	num[i] = num[i]*6;
	}
    println!("{:?}",num);
}
