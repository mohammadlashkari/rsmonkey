
- Python is a language the interpreter understands.
- Machine code is the language the CPU understands.
- An interpreter is a program that reads another program and uses its own machine code to perform what that program says.

```
High-level language
        ↓
     compiler
        ↓
   Assembly
        ↓
    assembler
        ↓
   Object file
        ↓
     linker
        ↓
   Executable
        ↓
       CPU

Machine code = instructions the CPU can execute.
Object file  = machine code + other information needed by the linker/debugger/loader
Executable   = the final program produced by the linker

Examples of information in an object file:
- Machine code
- Symbol table      → names/addresses of functions and variables
- Relocation info   → addresses that need to be fixed by the linker
- Data              → global variables, constants, string literals
- Debug info        → source code ↔ machine code mapping (optional)



High-level source code
        ↓
     Interpreter
        ↓
  Interpreter's
  machine code
        ↓
       CPU


Python code          Interpreter
   x = 2 + 2    →    reads/understands it
                         ↓
                  performs the operation
                         ↓
                        CPU

```


Token  = the category/kind of something in the source code
Lexeme = the actual sequence of characters in the source code that produces that token

```
let count = 42;

Lexeme       Token
------       -----
"let"        LET
"count"      IDENTIFIER
"="          ASSIGN
"42"         INTEGER
";"          SEMICOLON

```

A token can carry additional information extracted from its lexeme

```
42
 |
 v
Token {
    kind: Integer,
    lexeme/value: 42
}

if
 |
 v
Token {
    kind: If,
}

```

Lexeme and parsed value aren't always the same thing.

```
lexeme = "0042"   // exactly what appeared in the source
value  = 42       // parsed integer value
```


How does a lexer decide which token to choose when multiple token rules match?

- **Maximal munch**: consume the longest possible sequence of characters that forms a valid token.
- **Precedence**: if multiple token rules match the same lexeme, use the rule with higher precedence.

```
for        -> Token { For }
forexample -> Token { Identifier("forexample") }

```
