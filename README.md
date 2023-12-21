# onefig
> One configuration file; all the configurations you need.
Onefig eliminates the clutter of multiple configuration files, a common issue in Unix-based systems, particularly NixOS. By consolidating configuration settings into a single language and a single file or directory, Onefig enhances organization, control over system features, and true reproducibility of such system.

# Installation
---
You will need:
- a `cargo` installation
- a `unix` operating system *(currently)*

Installation:
1. run `cargo install onefig` to install
2. add `$HOME/.cargo/bin` to your path if you haven't already
    - You can do this by setting the `PATH` varible to `$HOME/.cargo/bin:$PATH`
3. run `onefig test` to test if everything is working correctly
4. now run `onefig --help` for help using the cli :D

# Language Features
---
Onefig is designed to be both simple and concise and yet flexable enough to incorporate the features of the most common configuration langauges; this is reflected within the features of the language.

**example.nf**

```onefig
# An example onefig comment
// Another example onefig comment (they both work)

// it's generally prefered to use `//` for commenting about configurations
# while `#` is used for commenting out code

############################
# SCALAR / PRIMATIVE TYPES #
############################

# Our root object (which continues for the entire document) will be a map,
# which is equivalent to a dictionary, hash or object in other languages.

key = "value"; // configurations have the syntax of `key set-symbol value`
string = "hello" // semi-colons are optional though prefered
number: 42; // `:` is another valid set symbol along with `=`
float = 3.14, // `,` can also be used instead of `;`
boolean: true;
multi-line_string: "first line\nand second line"; // escape characters are `\n`, `\t` and `\'` or `\"`
path = bob.dog.name;
raw: </
		&)(*&%)(*&)@(#*$&)(*@&#$)
		woah, it's a statement that onefig may not support but allows anyways!
		(this gets directly written into the target configuration file)
	 \>;

"keys can also be quoted" = 'value'; // Both " and ' work
valid-Key123_name456 = true; // numbers, letters, `-` and `_` are all valid bare keys
12 = "number?"; // valid but discouraged

# A bare-quoted key is valid though heavily discouraged
"" = "blank"
'' = 'something blank' // valid but still, don't do it

# Two scalar types cannot be assigned to the same key
dont-do-this = true; // works
dont-do-this = false; // throws error as the two definitions conflict

####################
# COLLECTION TYPES #
####################

##########
# Arrays #
##########

array1 = [ 1, 2, 3 ]
array2 = [ "commas" "don't" "really"; "matter" ] // you can also use semi-colons
array3 = [ [1.2, "nice"] "types don't really matter" ]
array4 = [ "unless" "you", "compile" "to", "toml", "then" "types"; "matter" ] // see, commas don't matter
array5: [
    "whitespace"
    "never"
    "matters"
]

# Two arrays cannot be assigned to the same key (though this may change in future updates)
dont-do-this = [ 1, 2, 3 ]; // works
dont-do-this = [ 1 2 3 ]; // throws error as the two definitions conflict

##########
# Tables #
##########

# Tables are like another scope with the file as a root table
jim = {
    name = "Jim Johnson";
    "age" = 23;
}

# Tables get combined NOT overwritten
stewart = {
    name = "Stewart Lee"
}
stewart: { // gets combined as stewart = { name = "Stewart Lee" age = 94 };
    age = 94;
}

# Tables (or hash tables or dictionaries) are collections of key/value
# pairs. They appear in square brackets on a line by themselves.
# Empty tables are allowed and simply have no key/value pairs within them.

empty-table = {};

# Dots are prohibited in bare keys because dots are used to signify nested tables.
# Naming rules for each dot separated part are the same as for keys.
dave.dog.name = "Biscut"; // same as `dave: { dog: { name = "Biscut" } };`

# Tables are automatically initialised when using dot keys.
another.dot.key = 2; // automatically initalises tables `another: { dot: { key = 2 } };`

# Two tables actually can be assigned to the same key
actually_do-this = { one: "dog", three: "fish" }; // works
actually_do-this = { two = "cat" }; // the two tables get merged, `actually_do-this` becomes `{ one: "dog", two: "cat", three: "fish" }`
actually_do-this.four = "bird"; // same applies to dot keys

######################
# Special Statements #
######################

# There are quite a few special statements that define different things and perform different actions in onefig:
#	 - config file statements
#	 - import statements
#	 - include statements
#	 - shell commands

##########################
# Config File Statements #
##########################

# they define a target configuration file that onefig writes to

# valid config file types are (this may change in the future):
# 	- json
# 	- nix
# 	- toml

# config file statement syntax
// conff <config file type> <table name>   : <config file path>
   conff json               my-config-file : "my-config-file.json";

# they can then be written to
my-config-file.data = "Hello, world!"; // compiles to json `{ "data": "Hello, world" }`

###############################
# Import & Include Statements #
###############################

# Import statements simply import the configurations from another onefig file as if it were one large file instead of multiple
# While include statements include the contents of a specified file into the compiled binary and copies it to the desired place on running of the binary

# import statment syntax
// import <config file path>
   import "another-config-file.nf";

# include statment syntax
// include <current file-path> as <target file-path>
   include "example-file.jpg"  as "desired/example-file.jpg";
   
#####################################
# Shell Commands & Apply Operations #
#####################################

# Shell commands are executed once their parent conff (config file) is evaluated

# Shell command syntax
<parent conff> $ <shell command>
example-config $ echo "example config evaluated! :D";

# Apply operations are an operation in which all the paths in an array are prefixed with another path
# eg
paths = prefix >> [
	one,
	two,
	three,
	four,
	five,
	"not a path",
];
// same as
paths = [
	prefix.one,
	prefix.two,
	prefix.three,
	prefix.four,
	prefix.five,
	"not a path",
];
```