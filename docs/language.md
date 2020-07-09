# Language Support in Talpa

> This document is incomplete

Talpa was designed with a unique capability to compile for multiple platforms from a single codebase. 
To do this, Talpa generates a tree which can be parsed into a language with a language compile structure. 
These structures implement functions that parse objects from the tree into the source code for that language. 

To code language support for Talpa, you will need to know a few things.

- Rust (The language Talpa is written in)
- Talpa's parser tree layout
- The language you wish to parse into

To implement the language support, you will need to complete 2 steps.

1. Create and implement the structure in languages/*mylanguage*.rs
2. Add your language to the languages/mod.js file
