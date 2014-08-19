// Write a program that does arithmetic computations for the 
// fixed size integers of Rust. These computations must be
// done such that the result would overflow.
// The program should demonstrate what the follow expressions do.

fn main() {
	println!("32-bit signed integer expressions");
	println!("-(-2147483647-1) should equal 2147483648 but was {}", -(-2147483647i32-1));	
	println!("2000000000 + 2000000000 should equal 4000000000 but was {}", 2000000000i32 + 2000000000);
}
