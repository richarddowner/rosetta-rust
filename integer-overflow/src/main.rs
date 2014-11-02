// Write a program that does arithmetic computations for the 
// fixed size integers of Rust. These computations must be
// done such that the result would overflow.
// The program should demonstrate what the follow expressions do.

fn main() {
	println!("32-bit signed integer expressions");
	println!("-(-2147483647-1) should equal 2147483648 but was {}", -(-2147483647i32-1));
	println!("2000000000 + 2000000000 should equal 4000000000 but was {}", 2000000000i32 + 2000000000);
	println!("-2147483647 - 2147483647 should equal -4294967294 but was {}", -2147483647i32 - 2147483647);
	println!("46341 * 46341 should equal 2147488281 but was {}", 46341i32 * 46341);
	
	println!("task '<main>' failed at 'attempted to divide with overflow");
	//println!("(-2147483647-1) / -1 should equal 2147483648 but was {}", (-2147483647i32-1) / -1);

	println!("64-bit signed integer expressions");
	println!("-(-9223372036854775807-1) should equal 9223372036854775808 but was {}", -(-9223372036854775807i64-1));
	println!("5000000000000000000+5000000000000000000 should equal 10000000000000000000 but was {}", 5000000000000000000i64+5000000000000000000);
	println!("-9223372036854775807 - 9223372036854775807 should equal -18446744073709551614 but was {}", -9223372036854775807i64 - 9223372036854775807);
	println!("3037000500 * 3037000500 should equal 9223372037000250000 but was {}", 3037000500i64 * 3037000500);
	
	println!("task '<main>' failed at 'attempted to divide with overflow");
	// println!("(-9223372036854775807-1) / -1 should equal 9223372036854775808 but was {}", (-9223372036854775807i64-1) / -1);

	println!("32-bit unsigned integer expressions");
	println!("-4294967295 should equal -4294967295 but was {}", -4294967295u32);
	println!("3000000000 + 3000000000 should equal 6000000000 but was {}", 3000000000u32 + 3000000000);
	println!("2147483647 - 4294967295 should equal -2147483648 but was {}", 2147483647u32 - 4294967295);
	println!("65537 * 65537 should equal 4295098369 but was {}", 65537u32 * 65537);

	println!("64-bit unsigned integer expressions");
	println!("-18446744073709551615 should equal -18446744073709551615 but was {}", -18446744073709551615u64);
	println!("10000000000000000000 + 10000000000000000000 should equal 20000000000000000000 but was {}", 10000000000000000000u64 + 10000000000000000000);
	println!("9223372036854775807 - 18446744073709551615 should equal -9223372036854775808 but was {}", 9223372036854775807u64 - 18446744073709551615);
	println!("4294967296 * 4294967296 should equal 18446744073709551616 but was {}", 4294967296u64 * 4294967296);
}
