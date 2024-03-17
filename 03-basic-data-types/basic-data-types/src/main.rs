#![allow(dead_code)] // prevents compiler warnings

fn main() {
    
	// BASIC DATA STRUCTURE TYPES
	
	
	// Structures
	
	// A struct is a collection of fields.
	
	// A field is a data value associated with a data structure.
	// Its value can be of a primitive type or a data structure.
	
	// Its definition is like a blueprint for a compiler on how
	// to layout the fields in memory nearby each other.
	
	struct SeaCreature {
		// String is a struct
		animal_type: String,
		name: String,
		arms: i32,
		legs: i32,
		weapon: String,
	}
	
	// Calling Methods
	
	
	// methods are functions associated with a specific data type
	
	// static methods: methods that belong to a type itself and are
	// called using the :: operator
	
	// instance methods: methods that belong to an instance of a type
	// and are called using the . operator.
	
	// Using a static method to create an instance of String:
	let s = String::from("Hello world!");
	// Using a method on the instance
	println!("{} is {} characters long.", s, s.len());
	
	
	
	// MEMORY
	
	
	// Rust programs have 3 memory regions where data is stored:
	
	// 1) data memory: For data that is fixed in size and static (always
	// available through life of program).  Generally considered very
	// fast to use instance locations are known and fixed.
	
	// 2) stack memory: For data that is declared as variables within
	// a function.  The location of this memory never changes for the
	// duration of a function call.  Because of this, compilers can
	// optimize code so stack data is very fast to access.
	
	// 3) heap memory: For data that is created while the application
	// is running.  Data in this region may be added, moved, removed,
	// resized, etc.  Because of its dynamic nature, it's generally
	// considered slower to use, but it allows for much more creative
	// usages of memory.  When data is added to this region, we call it
	// an allocation.  When data is removed from this section, we call
	// it deallocation.
	
	
	// Creating Data in Memory
	
	let ferris = SeaCreature {
		// String struc is also on stack,
		// but holds a reference to data on heap
		animal_type: String::from("crab"),
		name: String::from("Ferris"),
		arms: 2,
		legs: 4,
		weapon: String::from("claw"),
	};
	
	let sarah = SeaCreature {
		animal_type: String::from("octopus"),
		name: String::from("Sarah"),
		arms: 8,
		legs: 0,
		weapon: String::from("brain"),
	};
	
	println!(
		"{} is a {}.  They have {} arms, {} legs, and a {} weapon.",
		ferris.name, ferris.animal_type, ferris.arms, ferris.legs,
		ferris.weapon
	);
	
	println!(
		"{} is a {}.  They have {} arms, and {} legs.  They have no weapon..",
		sarah.name, sarah.animal_type, sarah.arms, sarah.legs
	);
	
	
	// Tuple-like Structs
	
	// For conciseness, you can create structs that are used like a tuple
	
	struct Location(i32, i32);
	// this is still a struct on a stack:
	let loc = Location(42, 32);
	println!("{}, {}", loc.0, loc.1);
	
	
	// Unit-like Structs
	
	// Structs do not have to have any fields at all
	// As mentioned in Chapter 1, a unit is another word for an
	// empty tuple () .  This is why this kind of struct is 
	// called Unit-like.
	
	// **rarely used**
	
	struct Marker;
	
	let _m = Marker;
	
	
	// Enumerations
	
	// Enums allow you to create a new type that can have a value
	// of several tagged elements using the enum keyword.
	
	// match helps ensure exhaustive handling of all possible
	// enum values, making it a powerful tool in ensuring
	// quality code.
	
	enum Species {
		Crab,
		Octopus,
		Fish,
		Clam
	}
	
	struct SeaCreature2 {
		species: Species,
		name: String,
		arms: i32,
		legs: i32,
		weapon: String,
	}
	
	let ferris = SeaCreature2 {
		species: Species::Crab,
		name: String::from("Ferris"),
		arms: 2,
		legs: 4,
		weapon: String::from("claw"),
	};
	
	match ferris.species {
		Species::Crab => println!("{} is a crab", ferris.name),
		Species::Octopus => println!("{} is a octopus",ferris.name),
        Species::Fish => println!("{} is a fish",ferris.name),
        Species::Clam => println!("{} is a clam",ferris.name),
	}
	
	
	// Enumerations With Data
	
	// enum elements can also have one or more data types allowing
	// them to behave like union from C.
	
	// When an enum is pattern matched using match, you can bind
	// a variable name to each data value.
	
	// Memory details of enum:
	// An enum data value will have a memory size equal to its 
	// largest element.  This allows for all potential values to fit
	// in the same space of memory.
	// In addition to element data types (if any), each element also
	// has a numeric value that represents which tag it is.
	
	// Rust's enum is also known as a tagged union
	// The combining of types to make a new type is what people mean
	// when they say Rust has algebraic types.
	
	enum Species2 { Crab, Octopus, Fish, Clam }
	enum PoisonType { Acidic, Painful, Lethal }
	enum Size { Big, Small }
	enum Weapon {
		Claw(i32, Size),
		Poison(PoisonType),
		None
	}
	
	struct SeaCreature3 {
		species: Species2,
		name: String,
		arms: i32,
		legs: i32,
		weapon: Weapon,
	}
	
	// SeaCreature3's data is on stack
	let ferris = SeaCreature3 {
		// String struct is also on stack
		// but holds a reference to data on heap
		species: Species2::Crab,
		name: String::from("Ferris"),
		arms: 2,
		legs: 4,
		weapon: Weapon::Claw(2, Size::Small),
	};
	
	match ferris.species {
		Species2::Crab => {
			match ferris.weapon {
				Weapon::Claw(num_claws,size) => {
					let size_description = match size {
						Size::Big => "big",
						Size::Small => "small"
					};
					println!("ferris is a crab with {} {} claws",
						num_claws, size_description
					)
				},
				_ => println!("ferris is a crab with some other weapon")
			}
		},
		_ => println!("ferris is some other animal"),
	}
	
					
	
	
}
