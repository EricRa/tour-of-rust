fn main() {

	// Variables

	// rust infers the type of x
	let x = 13;
	println!("{}", x);

	// rust can also be explicit about the type
	let x: f64 = 3.14159; // "shadowing" x
	println!("{}", x);

	// rust can also declare and initialize later, but this is rarely done
	let x;
	x = 0;
	println!("{}", x);
	
	// Changing Variables
	
	let mut x = 42;
	println!("{}", x);
	x = 13;
	println!("{}", x);
	
	// Basic Types
	let x = 12;  // by default this is i32
	let a = 12u8;
	let b = 4.3; // by default this is f64
	let c = 4.3f32;
	let d = "r"; // unicode character
	let ferris = "🦀"; // also a unicode character
	let bv = true;
	let t = (13, false);
	let sentence = "hello!";
	println!(
		"{} {} {} {} {} {} {} {} {} {}",
		x, a, b, c, d, ferris, bv, t.0, t.1, sentence
	);
	
	// Basic Type Conversion Using as
	let a = 13u8;
	let b = 7u32;
	let c = a as u32 + b;
	println!("{}", c);
	
	let t = true;
	println!("{}", t as u8);
	
	// Constants
	
	// consts would normally be declared at the top of the program
	// outside of the main function.
	const PI: f32 = 3.14159;  
	println!("apple {}", PI);
	
	// Arrays
	// fixed length.  for variable length, use vectors
	
	let nums: [i32; 3] = [1, 2, 3];
	println!("{:?}", nums);
	println!("{}", nums[1]);
	
	// Functions
	
	// you can return an expression by leaving out the return keyword
	// and the semicolon at the end of a function, as below.
	
	// function names are snake_case.
	
	// it probably makes more sense to declare the functions below
	// outside of the main function in a typical program
	
	fn add(x: i32, y:i32) -> i32 {
		return x + y;
	}
	
	fn subtract(x: i32, y: i32) -> i32 {
		x - y
	}
	
	println!("42 + 13 = {}", add(42, 13));
	println!("42 - 13 = {}", subtract(42, 13));
	
	// Multiple Return Values
	
	// functions can return multiple values by returning a tuple
	// of values.  these tuple elements can be referenced by their
	// index number.
	
	fn swap(x: i32, y: i32) -> (i32, i32) {
		return(y, x);
	}
	
	// return a tuple of return values
	let result = swap(123, 321);
	println!("{} {}", result.0, result.1);
	
	// destructure the tuple into two variable names
	let (a, b) = swap(result.0, result.1);
	println!("{} {}", a, b);
	
	
	// Returning Nothing
	
	// If no return type is specified for a function, it returns an empty
	// tuple, known as a unit
	// An empty tuple is represented by ()
	
	fn make_nothing() -> () {
		return();
	}
	
	// he return type is implied as ()
	fn make_nothing2() {
		// will return ()
	}
	
	let a = make_nothing();
	let b = make_nothing2();
	
	// printing a debug string ({:?}) for a and b because
	// it's hard to print nothingness
	println!("The value of a: {:?}", a);
	println!("The value of b: {:?}", b);
	
	
	
}
