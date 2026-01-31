# Data Interchange File Format

```shell
// Data Interchange File Format
// File Extension: .data, .data-interchange

// This is a Comment

// Data Structure Declaration
// Initialized with Default Values
// Syntax - type: Name = {values};
structure: DataStructure = {
	uint8: unsigned_integer_8bit = 1,
	uint16: unsigned_integer_16bit = 2,
	uint32: unsigned_integer_32bit = 3,
	uint64: unsigned_integer_64bit = 4,
	uint128: unsigned_integer_128bit = 5,
	int8: signed_integer_8bit = -1,
	int16: signed_integer_16bit = -2,
	int32: signed_integer_32bit = -3,
	int64: signed_integer_64bit = -4,
	int128: signed_integer_128bit = -5,
	f8: float_8bit = 1.0,
	f16: float_16bit = 2.0,
	f32: float_32bit = 3.0,
	f64: float_64bit = 4.0,
	f128: float_128bit = 5.0,
	bool: boolean = true,
	achar8: character_ascii_8bit = 'a',
	uchar8: character_utf8 = 'b',
	uchar16: character_utf16 = 'c',
	uchar32: character_utf32 = 'd',
	astr8: string_ascii_8bit = "string",
	ustr8: string_utf8 = "string",
	ustr16: string_utf16 = "string",
	ustr32: string_utf32 = "string",
	array(5)<uint8>: array = [];,
	array(5:2)<uint8>: two_dimensional_array = [];,
	map(5)<uint8:ustr8>: hash_map = [()];,
	structure: Struct = {};,
	enumerator: Enum = {};
};

// Enumerator Declaration
// Initialized with Default Values
// Syntax - type: Name = {values};
enumerator: Enumerator = {
	FIRST_VALUE = 0,
	SECOND_VALUE = 1,
	THIRD_VALUE = 2,
	FOURTH_VALUE = 3,
	FIFTH_VALUE = 4
};

// Boolean Declaration
// Syntax - type: name = value;
// OR
// Syntax - type: name = null;
bool: boolean = true;

// Character Declaration
// Syntax - type: name = value;
// OR
// Syntax - type: name = null;
uchar8: character_utf8 = 's';

// Float Declaration
// Syntax - type: name = value;
// OR
// Syntax - type: name = null;
f8: float_8bit = 1.0;

// Integer Declaration
// Syntax - type: name = value;
// OR
// Syntax - type: name = null;
uint8: unsigned_integer_8bit = 1;

// String Declaration
// Syntax - type: name = value;
// OR
// Syntax - type: name = null;
ustr8: string_utf8 = "string";

// One Dimensional Array Declaration
// Syntax - type(dimension)<type>: name = [values];
// OR
// Syntax - type(?)<type>: name = [values];
// OR
// Syntax - type(dimension)<type>: name = [null];
array(5)<uint8>: array = [
	1,
	2,
	3,
	4,
	5
];

// Two Dimensional Array Declaration
// Syntax - type(dimension:dimension)<type>: name = [values];
// OR
// Syntax - type(?:?)<type>: name = [values];
// OR
// Syntax - type(dimension:dimension)<type>: name = [null];
array(5:2)<uint8>: two_dimensional_array = [
	1, 5
	2, 4
	3, 3
	4, 2
	5, 1
];

// Hash Map Declaration
// Syntax - type(dimension)<type:type>: name = [(key:value)];
// OR
// Syntax - type(?)<type:type>: name = [(key:value)];
// OR
// Syntax - type(dimension)<type:type>: name = [(null:null)];
map(5)<uint8:ustr8>: hash_map = [
	(1:"string"),
	(2:"string"),
	(3:"string"),
	(4:"string"),
	(5:"string")
];

// Data Structure Declaration
// Declared without Default Values
// Syntax - type: Name = {values};
structure: DataStructure = {
	uint8: unsigned_integer_8bit,
	uint16: unsigned_integer_16bit,
	uint32: unsigned_integer_32bit,
	uint64: unsigned_integer_64bit,
	uint128: unsigned_integer_128bit,
	int8: signed_integer_8bit,
	int16: signed_integer_16bit,
	int32: signed_integer_32bit,
	int64: signed_integer_64bit,
	int128: signed_integer_128bit,
	f8: float_8bit,
	f16: float_16bit,
	f32: float_32bit,
	f64: float_64bit,
	f128: float_128bit,
	bool: boolean,
	achar8: character_ascii_8bit,
	uchar8: character_utf8,
	uchar16: character_utf16,
	uchar32: character_utf32,
	astr8: string_ascii_8bit,
	ustr8: string_utf8,
	ustr16: string_utf16,
	ustr32: string_utf32,
	array(5)<uint8>: array,
	array(5:2)<uint8>: 2d_array,
	map(5)<uint8:ustr8>: hash_map,
	structure: Struct,
	enumerator: Enum
};

// Enumerator Declaration
// Declared without Default Values
// Syntax - type: Name = {values};
enumerator: Enumerator = {
	FIRST_VALUE,
	SECOND_VALUE,
	THIRD_VALUE,
	FOURTH_VALUE,
	FIFTH_VALUE
};
```

