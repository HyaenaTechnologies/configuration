# Data Interchange Format

- File Extension: .data, .binary, .schema
- Data Interchange Source File: .data
- Binary Format Source File: .binary
- Schema Definition Header File: .schema
- .data Files May be Dynamically Typed
- .data Files May be Statically Typed
- .data Files are Required
- .schema Files Must be Statically Typed
- .schema Files Cannot be Dynamically Typed
- .schema Files are Optional
- // This is a Comment

## Dynamically Typed Syntax

```shell
// Data Structure Declaration
// May be Declared with Initialized Fields
// Syntax - typedef Name type = {fields}
typedef DataStructure struct = {
	byte_type; // Encoding Dependent: ASCII, Binary, UTF-8, UTF-16, UTF-32
	integer_type;
	float_type;
	boolean_type;
	character_type; // Encoding Dependent: ASCII, UTF-8, UTF-16, UTF-32
	string_type; // Encoding Dependent: ASCII, UTF-8, UTF-16, UTF-32
	array_type;
	map_type;
	structure_type DataStructure;
};

// Data Structure Initialization
// Syntax - var name type = {fields}
var structure_type DataStructure = {
	byte_type = null; // Encoding Dependent: ASCII, Binary, UTF-8, UTF-16, UTF-32
	integer_type = 1;
	float_type = 1.0;
	boolean_type = true;
	character_type = 'a'; // Encoding Dependent: ASCII, UTF-8, UTF-16, UTF-32
	string_type = "string"; // Encoding Dependent: ASCII, UTF-8, UTF-16, UTF-32
	array_type = [];
	map_type = [()];
	structure_type DataStructure = {};
};

// Boolean Initialization
// Syntax - var name = value;
// OR
// Null Syntax - var name = null;
var boolean_type = true;

// Character Initialization
// Syntax - var name = value;
// OR
// Null Syntax - var name = null;
// Encoding Dependent: ASCII, UTF-8, UTF-16, UTF-32
var character_type = 'a';

// Float Initialization
// Syntax - var name = value;
// OR
// Null Syntax - var name = null;
var float_type = 1.0;

// Integer Initialization
// Syntax - var name = value;
// OR
// Null Syntax - var name = null;
var integer_type = 1;

// String Initialization
// Syntax - var name = value;
// OR
// Null Syntax - var name = null;
// Encoding Dependent: ASCII, UTF-8, UTF-16, UTF-32
var string_type = "string";

// Array Initialization
// Syntax - var name = [values]
// OR
// Null Syntax - var name = [null]
// Array Index Syntax - name[index]
var array_type = [
	1,
	2,
	3,
	4,
	5
];

// Hash Map Initialization
// Syntax - var name = [(key:value)]
// OR
// Null Syntax - var name = [(null:null)]
// Hash Map Index Syntax - name[key]
var map_type = [
	(1:"string"),
	(2:"string"),
	(3:"string"),
	(4:"string"),
	(5:"string")
];
```

## Statically Typed Syntax

```shell
// Data Structure Declaration
// May be Declared with Initialized Fields
// Syntax - typedef Name type = {fields}
typedef DataStructure struct = {
	byte_type byte; // Encoding Dependent: ASCII, Binary, UTF-8, UTF-16, UTF-32
	unsigned_integer_size usize; // Platform Dependent: x86, x86_64, AARCH32, AARCH64, RISCV32, RISCV64
	unsigned_integer_8bit uint8;
	unsigned_integer_16bit uint16;
	unsigned_integer_32bit uint32;
	unsigned_integer_64bit uint64;
	unsigned_integer_128bit uint128;
	signed_integer_size isize; // Platform Dependent: x86, x86_64, AARCH32, AARCH64, RISCV32, RISCV64
	signed_integer_8bit int8;
	signed_integer_16bit int16;
	signed_integer_32bit int32;
	signed_integer_64bit int64;
	signed_integer_128bit int128;
	float_32bit float32;
	float_64bit float64;
	float_128bit float128;
	boolean_type bool;
	character_type char; // Encoding Dependent: ASCII, UTF-8, UTF-16, UTF-32
	string_type str; // Encoding Dependent: ASCII, UTF-8, UTF-16, UTF-32
	array_type [5]uint8;
	list_type list<uint8>;
	map_type map<uint8, str>;
	structure_type DataStructure;
};

// Data Structure Initialization
// Syntax - var name type = {fields}
var structure_type DataStructure = {
	byte_type byte = null; // Encoding Dependent: ASCII, Binary, UTF-8, UTF-16, UTF-32
	unsigned_integer_size usize = 0; // Platform Dependent: x86, x86_64, AARCH32, AARCH64, RISCV32, RISCV64
	unsigned_integer_8bit uint8 = 1;
	unsigned_integer_16bit uint16 = 2;
	unsigned_integer_32bit uint32 = 3;
	unsigned_integer_64bit uint64 = 4;
	unsigned_integer_128bit uint128 = 5;
	signed_integer_size isize = null; // Platform Dependent: x86, x86_64, AARCH32, AARCH64, RISCV32, RISCV64
	signed_integer_8bit int8 = -1;
	signed_integer_16bit int16 = -2;
	signed_integer_32bit int32 = -3;
	signed_integer_64bit int64 = -4;
	signed_integer_128bit int128 = -5;
	float_32bit float32 = null;
	float_64bit float64 = 2.0;
	float_128bit float128 = 3.0;
	boolean_type bool = true;
	character_type char = 'a'; // Encoding Dependent: ASCII, UTF-8, UTF-16, UTF-32
	string_type str = "string"; // Encoding Dependent: ASCII, UTF-8, UTF-16, UTF-32
	array_type [5]uint8 = [];
	list_type list<uint8> = [];
	map_type map<uint8, str> = [()];
	structure_type DataStructure = {};
};

// Boolean Initialization
// Syntax - var name type = value;
// OR
// Null Syntax - var name type = null;
var boolean_type bool = true;

// Character Initialization
// Syntax - var name type = value;
// OR
// Null Syntax - var name type = null;
// Encoding Dependent: ASCII, UTF-8, UTF-16, UTF-32
var character_type char = 'a';

// Float Initialization
// Syntax - var name type = value;
// OR
// Null Syntax - var name type = null;
var float_32bit float32 = 1.0;

// Integer Initialization
// Syntax - var name type = value;
// OR
// Null Syntax - var name type = null;
var unsigned_integer_8bit uint8 = 1;

// String Initialization
// Syntax - var name type = value;
// OR
// Null Syntax - var name type = null;
// Encoding Dependent: ASCII, UTF-8, UTF-16, UTF-32
var string_type str = "string";

// Array Initialization
// Syntax - var name [length]type = [values]
// OR
// Multi-Dimensional Syntax - var name [rows][columns]type = [values]
// OR
// Unknown Size Syntax - var name []type = [values]
// OR
// Null Syntax - var name [length]type = [null]
// Array Index Syntax - name[index]
var array_type [5]uint8 = [
	1,
	2,
	3,
	4,
	5
];

// List Initialization
// Syntax - var name type<type> = [values]
// OR
// Null Syntax - var name type<type> = [null]
// List Index Syntax - name[index]
var list_type list<uint8> = [
	1,
	2,
	3,
	4,
	5
];

// Hash Map Initialization
// Syntax - var name type<type, type> = [(key:value)]
// OR
// Null Syntax - var name type<type, type> = [(null:null)]
// Hash Map Index Syntax - name[key]
var map_type map<uint8, str> = [
	(1:"string"),
	(2:"string"),
	(3:"string"),
	(4:"string"),
	(5:"string")
];
```

