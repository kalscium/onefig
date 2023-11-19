use flexar::prelude::*;
use crate::errors::SyntaxError;

flexar::lexer! {
    [[Token] lext, current, 'cycle]
    else flexar::compiler_error!((SY001, lext.position()) current).throw();

    token_types {
        Apply => ">>";
        Dot => '.';
        Not => '!';
        EE => "==";
        NE => "!=";
        GTE => ">=";
        GT => '>';
        LTE => "<=";
        LT => '<';
        Glob => '*';
        Ignore => '@';

        Shell(_command: Box<[Box<str>]>) => "<shell command>";
        RawConf(_conf: Box<str>) => "<raw configuration>";
        
        Set(is_eq: bool) => if *is_eq { '=' } else { ':' };
        Sep(is_semi: bool) => if *is_semi { ';' } else { ',' };
        
        LParen => '(';
        RParen => ')';
        LBrace => '{';
        RBrace => '}';
        LBracket => '[';
        RBracket => ']';
        
        Bool(val: bool) => val;
        Conff => "conff";
        Var => "var";
        Import => "import";
        Include => "include";
        As => "as";

        Ident(x: Box<str>) => x;
        Str(x: Box<str>) => format!("\"{x}\"");
        Int(x: usize) => x;
    }

    [" \n\t"] >> ({ lext.advance(); lext = lext.spawn(); continue 'cycle; });
    Dot: .;
    Glob: *;
    Ignore: @;

    // Multi-char
    EE: (= =);
    Apply: (> >);
    GTE: (> =);
    GT: >;
    NE: (! =);
    Not: !;

    // Matching
    LParen: '(';
    RParen: ')';
    LBrace: '{';
    RBrace: '}';
    LBracket: '[';
    RBracket: ']';

    // Shell commands
    $ child {
        advance:();
        set cmd { Vec::new() };
        set section { String::new() };
        rsome (current, 'shell) {
            { if " \n".contains(current) && !section.is_empty() { cmd.push(section.into_boxed_str()); section = String::new() } }; // Split the command into sections without spaces
            if (current == ' ') { advance:(); { continue 'shell }; }; // so that the space isn't included
            if (current == '\n') { done Shell(cmd.into_boxed_slice()); }; // terminate command
            { section.push(current) };
        };
    };

    // Raw Configurations & operators
    < child {
        advance: current;
        ck (current, =) { done LTE(); };
        ck (current, /) {
            advance:();
            set conf { String::new() };
            set section { String::new() };
            rsome (current, 'raw) {
                if (current == '\n') {
                    if (section.trim() == "\\>") {
                        done RawConf(conf.into_boxed_str());
                    };
                    { conf.push_str(section.trim()) };
                    { conf.push('\n') };
                    { section = String::new() };
                    advance:();
                    { continue 'raw };
                };
                { section.push(current) };
            };
            
        };
        done LT();
    };

    // Comments
    / child {
        advance: current;
        ck (current, /) {
            rsome current {
                { if current == '\n' { lext = child; continue 'cycle } };
            };
        };
    };
    
    # child {
        rsome current {
            { if current == '\n' { lext = child; continue 'cycle } };
        };
    };

    // 'Either way works' Tokens
    [",;"] child {
        ck (current, ;) {
            advance:();
            done Sep(true);
        };
        advance:();
        done Sep(false);
    };

    [":="] child {
        ck (current, =) {
            advance:();
            done Set(true);
        };
        advance:();
        done Set(false);
    };
    
    // Identifiers and keywords
    ["abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_"] child {
        set ident { String::new() };
        rsome (current, 'ident) {
            set matched false;
            ck (current, ["abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_-0123456789"]) {
                mut matched true;
                { ident.push(current) };
            };
            { if !matched { break 'ident } };
        };

        if (ident == "conff") { done Conff(); };
        if (ident == "var") { done Var(); };
        if (ident == "import") { done Import(); };
        if (ident == "include") { done Include(); };
        if (ident == "as") { done As(); };
        if (ident == "true") { done Bool(true); };
        if (ident == "false") { done Bool(false); };

        done Ident(ident.into_boxed_str());
    };

    ["0123456789"] child { // Integers
        set int { String::new() };
        rsome (current, 'ident) {
            set matched false;
            ck (current, ["0123456789"]) {
                mut matched true;
                { int.push(current) };
            };
            { if !matched { break 'ident } };
        };

        done Int(int.parse().unwrap());
    };

    '"' child {
        { child.advance() }
        set string { String::new() };
        rsome (current, 'string) {
            { if current == '\n' { break 'string; } };
            ck (current, '"') {
                advance:();
                done Str(string.into_boxed_str());
            };
            ck (current, '\\') { // Escape characters
                advance: current;
                ck (current, 'n') {
                    advance:();
                    { string.push('\n') };
                    { continue 'string };
                };
                ck (current, 't') {
                    advance:();
                    { string.push('\t') };
                    { continue 'string };
                };
                ck (current, '\\') {
                    advance:();
                    { string.push('\\') };
                    { continue 'string };
                };
                ck (current, '"') {
                    advance:();
                    { string.push('"') };
                    { continue 'string };
                };
                { return flexar::compiler_error!((SY003, child.spawn().position()) current).throw() };
            };
            { string.push(current) };
        };
        { return flexar::compiler_error!((SY002, child.spawn().position()) current).throw() };
    };

    '\'' child {
        { child.advance() }
        set string { String::new() };
        rsome (current, 'string) {
            { if current == '\n' { break 'string; } };
            ck (current, '\'') {
                advance:();
                done Str(string.into_boxed_str());
            };
            ck (current, '\\') { // Escape characters
                advance: current;
                ck (current, 'n') {
                    advance:();
                    { string.push('\n') };
                    { continue 'string };
                };
                ck (current, 't') {
                    advance:();
                    { string.push('\t') };
                    { continue 'string };
                };
                ck (current, '\\') {
                    advance:();
                    { string.push('\\') };
                    { continue 'string };
                };
                ck (current, '\'') {
                    advance:();
                    { string.push('\'') };
                    { continue 'string };
                };
                { return flexar::compiler_error!((SY003, child.spawn().position()) current).throw() };
            };
            { string.push(current) };
        };
        { return flexar::compiler_error!((SY002, child.spawn().position()) current).throw() };
    };
}