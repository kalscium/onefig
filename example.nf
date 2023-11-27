// Testing
dave.name = "Dave Smith";
dave.age = 23;
dave.skills = [
    "programming"
    "people skills"
    "and other stuff"
];

conff json dave: "dave.json";

# dave.dog.name = 86;

# dave.dog.owner = dave; // should throw error (cause json target)

@dave.dog: { // ignores the entire dave.dog object
    name = "Biscut"; // should throw error
    age = 5;
}

dave.dog.favourite-treats = [ "cookie\"s\"", "steaks" ];

import "example2.nf"; // Imports stewart from the other onefig script
include "Cargo.toml" as "Cargo.toml"; // as an example it'll include this project's `Cargo.toml` file

conff nix nixos: "example.nix";
12."dave".name: value; // orphan will be dropped when compiled
var env; // varibles are fully implemented yet, so it will also be dropped
env.example-varible: 23;

// Stuff for nix os
# Yoo both kinds of comments work

# nixos = "not a table" // should throw error

nixos: {
    version: "20.05";
    environment.systemPackages: pkgs >> [
        // Overrides
        </
            (pkgs.vscodium.overrideAttrs (oldAttrs: rec {
                src = fetchurl {
                    url = "https://github.com/VSCodium/vscodium/releases/download/${version}/VSCodium-linux-x64-${version}.tar.gz";
                    sha256 = "a606e540f8dfe5a049513138adb01f03d6005cbb9b1b6a871688462ea51aa573";
                };
                version = "1.81.1.23222";
            }))
        \>

        // Installed packages
        vscodium,
        rustup,
        docker,
        helix,
    ]
}

dave$ echo hello, world! (example shell command)
# nixos$ sudo nixos-rebuild switch --upgrade;