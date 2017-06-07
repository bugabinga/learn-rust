#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {

	fn add_one( x:u8 ) -> u8
	{
		x + 1
	}

	let a = add_one(5);

	assert_eq!(a, 6);

	let add_two = | yo:u8 | {
		yo + 2
	};

	let a = add_two(3);

	assert_eq!(a, 5);

	//Not a fan of shadowing... use rarely
	let a = add_one;

	assert_eq!(1, a(0));

	let op = |a:u8, b:u8|{
		(a+b)/2
	};

	assert_eq!(op(2,8), 5);
    }
}
