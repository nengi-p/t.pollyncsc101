 fn main() {
	println!("Welcome ");
	let toshiba:f64 = 2.0 * 450000.0;
	let mac:f64 = 1500000.0;
	let hp:f64 = 3.0 * 750000.0;
	let dell:f64 = 3.0 * 2850000.0 ;
	let acer:f64 = 250000.0;

	let sum = toshiba + mac + hp + dell + acer;

	println!("sum is {}",sum );
	
	let average = (toshiba + mac + hp + dell + acer)/5.0;
	
	
	println!("average is {}",average );

}
