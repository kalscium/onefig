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

// it's generally prefered to use // for commenting on specific lines of code
# while # is used for more general comments like sections

################
# SCALAR TYPES #
################

# Our root object (which continues for the entire document) will be a map,
# which is equivalent to a dictionary, hash or object in other languages.

key = "value"; // configurations have the syntax of `key set-symbol value`
string = "hello" // semi-colons are optional though prefered
number: 42; // `:` is another valid set symbol along with `=`
float = 3.14, // `,` can also be used instead of `;`
boolean: true;
multi-line_string: "first line\nand second line"; // escape characters are `\n`, `\t` and `\'` or `\"`
"keys can also be quoted" = 'value'; // Both " and ' work
valid-Key123_name456 = true; // numbers, letters, `-` and `_` are all valid bare keys
12 = "number?"; // valid but discouraged

# A bare-quoted key is valid though heavily discouraged
"" = "blank"
'' = 'something blank' // valid but still, don't do it

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
dave.dog.name = "Biscut"; // same as dave: { dog: { name = "Biscut" } };
```