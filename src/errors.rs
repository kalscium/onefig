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

    [Meta Errors]
    (SY404) "use of unimplemented or experimental feature": "unimplemented, experimental or unstable";
}