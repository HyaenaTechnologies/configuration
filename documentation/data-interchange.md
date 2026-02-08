# Data Interchange Format

## Dynamically Typed Syntax

```shell
// Data Interchange Format
// File Extension: .data, .schema
// Schema Definition Header File: .schema
// Data Interchange Source File: .data
// .data Files May be Dynamically Typed
// .data Files May be Statically Typed
// .data Files are Required
// .schema Files Must be Statically Typed
// .schema Files Cannot be Dynamically Typed
// .schema Files are Optional

// This is a Comment

// Data Structure Initialization
// Syntax - Name = {fields}
DataStructure = {
	integer = 1;	
	float = 1.0;
	boolean_type = true;
	character = 'a';
	string = "string";
	array = [];
	hash_map = [()];
	DataStructure = {};	
}

// Boolean Initialization
// Syntax - name = value;
// OR
// Null Syntax - name = null;
boolean_type = true;

// Character Initialization
// Syntax - name = value;
// OR
// Null Syntax - name = null;
character = 'a';

// Float Initialization
// Syntax - name = value;
// OR
// Null Syntax - name = null;
float = 1.0;

// Integer Initialization
// Syntax - name = value;
// OR
// Null Syntax - name = null;
integer = 1;

// String Initialization
// Syntax - name = value;
// OR
// Null Syntax - name = null;
string = "string";

// Array Initialization
// Syntax - name = [values]
// OR
// Null Syntax - name = [null]
array = [
	1,
	2,
	3,
	4,
	5
]

// Hash Map Initialization
// Syntax - name = [(key|value)]
// OR
// Null Syntax - name = [(null|null)]
hash_map = [
	(1|"string"),
	(2|"string"),
	(3|"string"),
	(4|"string"),
	(5|"string")
]
```

## Statically Typed Syntax

```shell
// Data Interchange Format
// File Extension: .data, .schema
// Schema Definition Header File: .schema
// Data Interchange Source File: .data
// .data Files May be Dynamically Typed
// .data Files May be Statically Typed
// .data Files are Required
// .schema Files Must be Statically Typed
// .schema Files Cannot be Dynamically Typed
// .schema Files are Optional

// This is a Comment

// Option Type Syntax - option<type|type> name = value;

// Data Structure Declaration
// May be Declared with Initialized Fields
// Syntax - type Name = {fields}
// Optional Field Syntax - optional<type> name;
struct DataStructure = {
	option<byte|null> byte_type;
	usize unsigned_integer_size; // Platform Dependant
	uint8 unsigned_integer_8bit;
	uint16 unsigned_integer_16bit;
	uint32 unsigned_integer_32bit;
	uint64 unsigned_integer_64bit;
	optional<uint128> unsigned_integer_128bit;
	option<int8|null> signed_integer_8bit;
	int16 signed_integer_16bit;
	int32 signed_integer_32bit;
	int64 signed_integer_64bit;
	optional<int128> signed_integer_128bit;
	option<float32|null> float_32bit;
	float64 float_64bit;
	optional<float128> float_128bit;
	bool boolean_type;
	char character;
	str string;
	array(5)<uint8> one_dimensional_array;
	array(5:2)<uint8> two_dimensional_array;
	map<uint8|str> hash_map;
	DataStructure data_structure;
}

// Data Structure Initialization
// Syntax - type name = {fields}
DataStructure data_structure = {
	byte byte_type = null;
	usize unsigned_integer_size = 0; // Platform Dependant
	uint8 unsigned_integer_8bit = 1;
	uint16 unsigned_integer_16bit = 2;
	uint32 unsigned_integer_32bit = 3;
	uint64 unsigned_integer_64bit = 4;
	uint128 unsigned_integer_128bit = 5;
	int8 signed_integer_8bit = null;
	int16 signed_integer_16bit = -2;
	int32 signed_integer_32bit = -3;
	int64 signed_integer_64bit = -4;
	int128 signed_integer_128bit = -5;
	float32 float_32bit = null;
	float64 float_64bit = 2.0;
	float128 float_128bit = 3.0;
	bool boolean_type = true;
	char character = 'a';
	str string = "string";
	array(5)<uint8> one_dimensional_array = [];
	array(5:2)<uint8> two_dimensional_array = [];
	map<uint8|str> hash_map = [()];
	DataStructure data_structure = {};
}

// Boolean Initialization
// Syntax - type name = value;
// OR
// Null Syntax - type name = null;
bool boolean_type = true;

// Character Initialization
// Syntax - type name = value;
// OR
// Null Syntax - type name = null;
char character = 'a';

// Float Initialization
// Syntax - type name = value;
// OR
// Null Syntax - type name = null;
float32 float_32bit = 1.0;

// Integer Initialization
// Syntax - type name = value;
// OR
// Null Syntax - type name = null;
uint8 unsigned_integer_8bit = 1;

// String Initialization
// Syntax - type name = value;
// OR
// Null Syntax - type name = null;
str string = "string";

// One Dimensional Array Initialization
// Syntax - type(length)<type> name = [values]
// OR
// Unknown Size Syntax - type(?)<type> name = [values]
// OR
// Null Syntax - type(length)<type> name = [null]
array(5)<uint8> one_dimensional_array = [
	1,
	2,
	3,
	4,
	5
]

// Two Dimensional Array Initialization
// Syntax - type(length:width)<type> name = [values]
// OR
// Unknown Size Syntax - type(?:?)<type> name = [values]
// OR
// Null Syntax - type(length:width)<type> name = [null]
array(3:2)<uint8> two_dimensional_array = [
	1,
	2,
	3,
	4,
	5,
	6
]

// Hash Map Initialization
// Syntax - type<type|type> name = [(key|value)]
// OR
// Null Syntax - type<type|type> name = [(null|null)]
map<uint8|str> hash_map = [
	(1|"string"),
	(2|"string"),
	(3|"string"),
	(4|"string"),
	(5|"string")
]
```

