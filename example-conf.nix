{config,pkgs,...}:{"environment"={"systemPackages"=[
(pkgs.vscodium.overrideAttrs (oldAttrs: rec {
src = fetchurl {
url = "https://github.com/VSCodium/vscodium/releases/download/${version}/VSCodium-linux-x64-${version}.tar.gz";
sha256 = "a606e540f8dfe5a049513138adb01f03d6005cbb9b1b6a871688462ea51aa573";
};
version = "1.81.1.23222";
}))
 "pkgs"."vscodium" "pkgs"."rustup" "pkgs"."docker" "pkgs"."helix"];};"version"="20.05";}