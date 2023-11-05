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

dave.dog: {
    name = "Biscut"; // should throw error
    age = 5;
}

dave.dog.favourite-treats = [ "cookies", "steaks" ];

// Global varibles
# conff nix nixos: "/etc/nixos/configuration.nix";
12."dave".name: value;
var env;
env.nix-version: 23;

// Stuff for nix os
# Yoo both kinds of comments work

# nixos = "not a table" // should throw error

# nixos: {
#     version: "20.05";
#     environment.systemPackages: pkgs >> [
#         // Overrides
#         </
#             (pkgs.vscodium.overrideAttrs (oldAttrs: rec {
#                 src = fetchurl {
#                     url = "https://github.com/VSCodium/vscodium/releases/download/${version}/VSCodium-linux-x64-${version}.tar.gz";
#                     sha256 = "a606e540f8dfe5a049513138adb01f03d6005cbb9b1b6a871688462ea51aa573";
#                 };
#                 version = "1.81.1.23222";
#             }))
#         \>
# 
#         // Installed packages
#         vscodium,
#         rustup,
#         docker,
#         helix,
#     ]
# }

# nixos$ sudo nixos-rebuild switch --upgrade;