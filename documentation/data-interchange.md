# Data Interchange Format

## Dynamically Typed Syntax

```shell
// Data Interchange Format
// File Extension: .data
// Data Interchange Source File: .data

// This is a Comment

// Data Structure Initialization
// Syntax - Name = {values}
DataStructure = {
	integer = 1;	
	float = 1.0;
	boolean_type = true;
	character = 'd';
	string = "string";
	array = [];
	hash_map = [()];
	DataStructure = {};
	EnumeratorType = {};
}

// Enumerator Initialization
// Syntax - Name = {values}
EnumeratorType = {
	FIRST_VALUE = 0,
	SECOND_VALUE = 1,
	THIRD_VALUE = 2,
	FOURTH_VALUE = 3,
	FIFTH_VALUE = 4
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
character = 's';

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
// Unknown Size Syntax - name = [values]
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

// This is a Comment

// Data Structure Declaration
// May be Declared with Initialized Fields
// Syntax - type Name = {values}
struct DataStructure = {
	option<uint8|null> unsigned_integer_8bit;
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
	option<char8|null> character_8bit;
	char16 character_16bit;
	optional<char32> character_32bit;
	option<str8|null> string_8bit;
	str16 string_16bit;
	optional<str32> string_32bit;
	array(5)<uint8> one_dimensional_array;
	array(5:2)<uint8> two_dimensional_array;
	map<uint8|str8> hash_map;
	DataStructure data_structure;
	EnumatorType enumerator_type;
}

// Enumerator Declaration
// May be Declared with Initialized Fields
// Syntax - type Name = {values}
enum EnumeratorType = {
	FIRST_VALUE, // value = 0
	SECOND_VALUE, // value = 1
	THIRD_VALUE, // value = 2
	FOURTH_VALUE, // value = 3
	FIFTH_VALUE // value = 4
}

// Data Structure Initialization
// Syntax - type name = {values}
DataStructure data_structure = {
	uint8 unsigned_integer_8bit = 1;
	uint16 unsigned_integer_16bit = 2;
	uint32 unsigned_integer_32bit = 3;
	uint64 unsigned_integer_64bit = 4;
	uint128 unsigned_integer_128bit = 5;
	int8 signed_integer_8bit = -1;
	int16 signed_integer_16bit = -2;
	int32 signed_integer_32bit = -3;
	int64 signed_integer_64bit = -4;
	int128 signed_integer_128bit = -5;
	float32 float_32bit = 1.0;
	float64 float_64bit = 2.0;
	float128 float_128bit = 3.0;
	bool boolean_type = true;
	char8 character_8bit = 'b';
	char16 character_16bit = 'c';
	char32 character_32bit = 'd';
	str8 string_8bit = "string";
	str16 string_16bit = "string";
	str32 string_32bit = "string";
	array(5)<uint8> one_dimensional_array = [];
	array(5:2)<uint8> two_dimensional_array = [];
	map<uint8|str8> hash_map = [()];
	DataStructure data_structure = {};
	EnumratorType enumerator_type = FIRST_VALUE;
}

// Enumerator Initialization
// Syntax - type name = value;
// OR
// Null Syntax - type name = null;
EnumeratorType enumerator_type = FIRST_VALUE;

// Boolean Initialization
// Syntax - type name = value;
// OR
// Null Syntax - type name = null;
bool boolean_type = true;

// Character Initialization
// Syntax - type name = value;
// OR
// Null Syntax - type name = null;
char8 character_8bit = 's';

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
str8 string_8bit = "string";

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
map<uint8|str8> hash_map = [
	(1|"string"),
	(2|"string"),
	(3|"string"),
	(4|"string"),
	(5|"string")
]
```

