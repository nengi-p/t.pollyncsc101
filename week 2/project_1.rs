fn main() {
	println!("Welcome ");
	let p:f64 = 520000000.0;
	let r:f64 = 10.0;
	let n:f64 = 5.0;

	let a = p * (1.0 + (r/100.0)) * n;

	println!("amount is {}",a );
	
	let ci = a - p;
	
	
	println!("compound interest is {}",ci );

}

