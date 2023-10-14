use flexar;

flexar::compiler_error! {
    [[Define] SyntaxError]
    (SY001) "unexpected character": ((1) "character `", "` is unexpected");
    (SY002) "non-terminated string": "expected `\"` to terminate string";
    (SY003) "invalid escape character": ((1) "escape character `", "` is invalid");
}