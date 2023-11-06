use flexar;

flexar::compiler_error! {
    [[Define] SyntaxError]
    (SY001) "unexpected character": "character `", "` is unexpected";
    (SY002) "non-terminated string": "expected `\"` to terminate string";
    (SY003) "invalid escape character": "escape character `", "` is invalid";
    (SY004) "expected atom": "expected atom, found `", "`";
    (SY005) "expected path": "expected path, found `", "`";
    (SY006) "expected identifier": "expected ident, found `", "`";
    (SY007) "expected stmt": "expected stmt, found `", "`";
    (SY008) "expected asignment operator": "expected `:` or `=`, found `", "`";
    (SY009) "all paths must begin with identifer": "try using `\"", "\"` instead of `", "`";
    (SY010) "expected list": "expected list, found `", "`";
    (SY011) "expected `]` to terminate list": "expected `]`, found `", "`";
    (SY012) "expected shell command": "expected shell-cmd, found `", "`";
    (SY013) "expected table": "expected table, found `", "`";
    (SY014) "expected `}` to terminate table": "expected `}`, found `", "`";
    (SY015) "expected configuration": "expected config, found `", "`";
    (SY016) "expected expr": "expected expr, found `", "`";
    (SY017) "invalid config-file type (expected `toml`, `json` or `nix`)": "`", "` is an invalid conff type";
    (SY018) "expected config-file type": "expected conff type, found `", "`";
    (SY019) "non-terminated raw configuration": "expected `\\>` to terminate raw configuration";

    [Meta Errors]
    (SY404) "use of unimplemented or experimental feature": "unimplemented, experimental or unstable";
}

flexar::compiler_error! {
    [[Define] LogicError]
    [Logical Errors (errors in logic)]
    (LG001) "key assigned to twice": "key `", "` first assigned to at ln `", "`";
    (LG002) "unsupported feature in json": "the `", "` feature isn't supported in json";
    (LG003) "unsupported feature in toml": "the `", "` feature isn't supported in toml";
}

flexar::compiler_error! {
    [[Define] RuntimeError]
    [Runtime Errors (Errors that, like the name suggests; occur on runtime)]
    (RT001) "shell command exited with non-zero exit code": "this shell cmd failed";
    (RT002) "compiled conff-tree is likely to be corrupted (serialization error)": "occured while loading `", "`";
    (RT003) "io error occured during compilation of onefig-script": "occured while compiling at `", "`";
    (RT004) "error occured while running shell command": "occured while running cmd `", "`";
    (RT005) "io error occured during loading of onefig-binary": "occured while loading `", "`";
    (RT006) "io error occured while generating configuration-files": "occured while generating config-file `", "`";
    (RT007) "io error occured while importing onefig-script": "occured while importing `", "`";
}