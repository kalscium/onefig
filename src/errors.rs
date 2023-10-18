use flexar;

flexar::compiler_error! {
    [[Define] SyntaxError]
    (SY001) "unexpected character": ((1) "character `", "` is unexpected");
    (SY002) "non-terminated string": "expected `\"` to terminate string";
    (SY003) "invalid escape character": ((1) "escape character `", "` is invalid");
    (SY004) "expected literal": ((1) "expected literal, found `", "`");
    (SY005) "expected path": ((1) "expected path, found `", "`");
    (SY006) "expected path-identifier after dot": ((1) "expected path-ident, found `", "`");
}