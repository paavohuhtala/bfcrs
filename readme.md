# bfcrs

[![Build Status](https://travis-ci.org/paavohuhtala/bfcrs.svg?branch=master)](https://travis-ci.org/paavohuhtala/bfcrs)

This is (for the most part) a Rust port of my optimizing Brainfuck compiler, [bfcfs](https://github.com/paavohuhtala/bfcfs/).

It can compile Brainfuck into a WebAssembly binary module and while performing a number of optimizations.

It also includes an IR interpreter and an alternative backend which emits C source code.

Licensed under the MIT license.
