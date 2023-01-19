# Sigma 0.0.1a programming language specification.
> By Quantumatic Team (2020-2022).
### Version of December 28, 2022

<table>
<tr><td width=33% valign=top>

- [Introduction](#introduction)
- [Notation](#notation)
- [Basic source code elements](#basic-source-code-elements)
  - [Characters](#characters)
  - [Letters and digits](#letters-and-digits)
- [Tokens](#tokens)
  - [Comments](#comments)
  - [Semicolons](#semicolons)
  - [Identifiers](#identifiers)
  - [Keywords](#keywords)
  - [Punctuators and operators](#punctuators-and-operators)
  - [Char literals](#character-literals)
  - [String literals](#string-literals)
  - [Numbers](#numbers)
    - [Integer literals](#integer-literals)
    - [Floating-point literals](#floating-point-literals)
    - [Imaginary number literals](#imaginary-literals)

</td><td width=33% valign=top>

- [Type system]()
  - [Boolean type]()
  - [Numeric type]()
  - [Pointer type]()
  - [Struct type]()
  - [Function type]()
  - [Interface type]()
- [Basic syntax]()
  - [Top-level statements]()
    - [Imports]()
    - [Function declarations]()
    - [Structure declarations]()
  - [Statements]()
    - [Expression statements]()
    - [Variable declaration statement]()
    - [If-else statements]()
    - [Switch statements]()
    - [For statements]()
    - [Return statements]()
    - [Break statements]()
    - [Return statements]()
    - [Continue statements]()


</td><td valign=top>

- [Memory managment system]()
  - [Stack and heap]()
  - [Heap allocations `new` and `destroy`]()
  - [Memory managment and OOP]()
- [Error handling]()
  - [Default error handler `handle`]()
  - [Custom error handlers]()

</td></tr>
</table>

## Introduction

This is the reference manual for the Sigma programming language. 

Sigma is a language designed with embeded programming in mind. It is strongly typed and has manual memory managment system.

The syntax is compact and simple to parse, allowing for easy analysis by automatic tools such as integrated development environments and fast compilation times.

Sigma is something between C and C++. It has more less abstractions than in C++, but at the same time allows developers to write easy-to-read and scalable code.

Here is an example of hello world program in Sigma:

```sigma
pub fun main() {
    println("hello world");
}
```

## Notation
The syntax is specified using a variant of Extended Backus-Naur Form(EBNF):
```ebnf
Syntax      = { Production } .
Production  = production_name "=" [ Expression ] "." .
Expression  = Term { "|" Term } .
Term        = Factor { Factor } .
Factor      = production_name | token [ "…" token ] | Group | Option | Repetition .
Group       = "(" Expression ")" .
Option      = "[" Expression "]" .
Repetition  = "{" Expression "}" .
```

Productions are expressions constructed from terms and the following operators, in increasing precedence:

```
|   alternation
()  grouping
[]  option (0 or 1 times)
{}  repetition (0 to n times)
```

Lowercase production names are used to identify lexical (terminal) tokens. Non-terminals are in CamelCase. Lexical tokens are enclosed in double quotes "" or back quotes ``.

The form a … b represents the set of characters from a through b as alternatives. The horizontal ellipsis … is also used elsewhere in the spec to informally denote various enumerations or code snippets that are not further specified. The character … (as opposed to the three characters ...) is not a token of the Sigma language.

## Basic source code elements

Source code is Unicode text encoded in UTF-8. The text is not canonicalized, so a single accented code point is distinct from the same character constructed from combining an accent and a letter; those are treated as two code points. For simplicity, this document will use the unqualified term character to refer to a Unicode code point in the source text.

Each code point is distinct; for instance, uppercase and lowercase letters are different characters.

Implementation restriction: For compatibility with other tools, a compiler may disallow the NUL character (U+0000) in the source text.

### Characters

The following terms are used to denote specific Unicode character categories:

```ebnf
newline        = /* the Unicode code point U+000A */ .
unicode_char   = /* an arbitrary Unicode code point except newline */ .
unicode_letter = /* a Unicode code point categorized as "Letter" */ .
unicode_digit  = /* a Unicode code point categorized as "Number, decimal digit" */ .
```

In [The Unicode Standard 8.0](https://www.unicode.org/versions/Unicode8.0.0/), Section 4.5 "General Category" defines a set of character categories. Sigma treats all characters in any of the Letter categories Lu, Ll, Lt, Lm, or Lo as Unicode letters, and those in the Number category Nd as Unicode digits.

### Letters and digits

The underscore character `_` (U+005F) is considered a lowercase letter.

```ebnf
letter        = unicode_letter | "_" .
decimal_digit = "0" … "9" .
binary_digit  = "0" | "1" .
octal_digit   = "0" … "7" .
hex_digit     = "0" … "9" | "A" … "F" | "a" … "f" .
```

## Tokens

### Comments

Comments serve as program documentation. There are two forms:

1. Line comments start with the character sequence `//` and stop at the end of the line. Example:
```sigma
// This is recursive implementation of factorial :3.
pub fun factorial(n: f64): f64 {
    if (n < 2) return 1;
    return factorial(n - 1) * n;
}
```

1. Multiline comments start with the character sequence `/*` and stop with the first subsequent character sequence `*/`. Example:

```sigma
/**
 * @param a first number
 * @param b second number
 *
 * @return maximum number of numbers a and b
 */
pub fun max(a: f64, b: f64): f64 {
    if (a > b) return a;
    return b;
}
```

> A comment **cannot start** inside **a character or string literal**, or **inside a comment**.

## Semicolons
The formal syntax uses semicolons ";" as terminators in a number of productions. Sigma programs are required to have ";" at the end of each statement. Compiler does **NOT** emit them automatically.

Example of wrong Sigma program:

```sigma
pub fun printNumber(n: f64) {
    printf("%f", n) // no semicolon here. syntax error
}
```

## Identifiers
Identifiers name program entities such as variables and types. An identifier is a sequence of one or more letters and digits. The first character in an identifier must be a letter.

```ebnf
identifier = unicode_letter { unicode_letter | unicode_digit } .
```

Here are some examples of valid identifiers:
```sigma
test_identifier
название22
_x15
a
someVariable
```

## Keywords
The following keywords are reserved and may not be used as identifiers.

```sigma
break       default     fun     interface       case
struct      else        switch  const           if
i8          i16         i32     i64
u8          u16         u32     u64
continue    for         import  return          var
```

## Punctuators and operators
The following character sequences represent operators and punctuators:
```sigma
+    &     +=    &=     &&    ==    !=    (    )
-    |     -=    |=     ||    <     <=    [    ]
/    <<    /=    <<=    ++    =     ,     ;    ~
%    >>    %=    >>=    --    !     ...   .    :
*    ^     *=    ^=     >     >=    {     }    ?
```

## Character literals
A character literal represents an integer value identifying a Unicode code point. A character literal is expressed as one or more characters enclosed in single quotes, as in 'x' or '\n'. Within the quotes, any character may appear except newline and unescaped single quote. A single quoted character represents the Unicode value of the character itself, while multi-character sequences beginning with a backslash encode values in various formats.

The simplest form represents the single character within the quotes; since Sigma source text is Unicode characters encoded in UTF-8, multiple UTF-8-encoded bytes may represent a single integer value. For instance, the literal 'a' holds a single byte representing a literal a, Unicode U+0061, value 0x61, while 'ä' holds two bytes (0xc3 0xa4) representing a literal a-dieresis, U+00E4, value 0xe4.

Several backslash escapes allow arbitrary values to be encoded as ASCII text. There are four ways to represent the integer value as a numeric constant: \x followed by exactly two hexadecimal digits; \u followed by exactly four hexadecimal digits; \U followed by exactly eight hexadecimal digits, and a plain backslash \ followed by exactly three octal digits. In each case the value of the literal is the value represented by the digits in the corresponding base.

Although these representations all result in an integer, they have different valid ranges. Octal escapes must represent a value between 0 and 255 inclusive. Hexadecimal escapes satisfy this condition by construction. The escapes \u and \U represent Unicode code points so within them some values are illegal, in particular those above 0x10FFFF and surrogate halves.

After a backslash, certain single-character escapes represent special values:
```
\a   U+0007 alert or bell
\b   U+0008 backspace
\f   U+000C form feed
\n   U+000A line feed or newline
\r   U+000D carriage return
\t   U+0009 horizontal tab
\v   U+000B vertical tab
\\   U+005C backslash
\'   U+0027 single quote  (valid escape only within rune literals)
\"   U+0022 double quote  (valid escape only within string literals)
```

An unrecognized character following a backslash in a character literal is illegal.

```ebnf
character_literal           = "'" ( unicode_value | byte_value ) "'" .
unicode_value               = unicode_char | little_u_value | big_u_value | escaped_char .
byte_value                  = octal_byte_value | hex_byte_value .
octal_byte_value            = `\` octal_digit octal_digit octal_digit .
hex_byte_value              = `\` "x" hex_digit hex_digit .
little_u_value              = `\` "u" hex_digit hex_digit hex_digit hex_digit .
big_u_value                 = `\` "U" hex_digit hex_digit hex_digit hex_digit
                                      hex_digit hex_digit hex_digit hex_digit .
escaped_char                = `\` ( "a" | "b" | "f" | "n" | "r" | "t" | "v" | `\` | "'" | `"` ) .
```

Examples:

```
'ä'
'本'
'c'
'\t'
'\000'
'\007'
'\377'
'\x07'
'\xff'
'\u12e4'
'\U00101234'
'\''         // character literal containing single quote character
'aa'         // illegal: too many characters
'\k'         // illegal: k is not recognized after a backslash
'\xa'        // illegal: too few hexadecimal digits
'\0'         // illegal: too few octal digits
'\400'       // illegal: octal value over 255
'\uDFFF'     // illegal: surrogate half
'\U00110000' // illegal: invalid Unicode code point
```

## String literals
A string literal represents a string constant obtained from concatenating a sequence of characters. There are two forms: raw string literals and interpreted string literals.

String literals are character sequences between double quotes, as in "foo". Within the quotes, any character may appear except newline and unescaped double quote. The text between the quotes forms the value of the literal, with backslash escapes interpreted as they are in rune literals (except that \' is illegal and \" is legal), with the same restrictions. The three-digit octal (\nnn) and two-digit hexadecimal (\xnn) escapes represent individual bytes of the resulting string; all other escapes represent the (possibly multi-byte) UTF-8 encoding of individual characters. Thus inside a string literal \377 and \xFF represent a single byte of value 0xFF=255, while ÿ, \u00FF, \U000000FF and \xc3\xbf represent the two bytes 0xc3 0xbf of the UTF-8 encoding of character U+00FF.

```ebnf
string_literal = `"` { unicode_value | byte_value } `"` .
```

## Numbers

### Integer literals
An integer literal is a sequence of digits. An optional prefix sets a non-decimal base: 0b or 0B for binary, 0, 0o, or 0O for octal, and 0x or 0X for hexadecimal. A single 0 is considered a decimal zero. In hexadecimal literals, letters a through f and A through F represent values 10 through 15.

For readability, an underscore character _ may appear after a base prefix or between successive digits; such underscores do not change the literal's value.

```ebnf
int_literal        = decimal_literal | binary_literal | octal_literal | hex_literal .
decimal_literal    = "0" | ( "1" … "9" ) [ [ "_" ] decimal_digits ] .
binary_literal     = "0" ( "b" | "B" ) [ "_" ] binary_digits .
octal_literal      = "0" [ "o" | "O" ] [ "_" ] octal_digits .
hex_literal        = "0" ( "x" | "X" ) [ "_" ] hex_digits .

decimal_digits = decimal_digit { [ "_" ] decimal_digit } .
binary_digits  = binary_digit { [ "_" ] binary_digit } .
octal_digits   = octal_digit { [ "_" ] octal_digit } .
hex_digits     = hex_digit { [ "_" ] hex_digit } .
```

```
42
4_2
0600
0_600
0o600
0O600       // second character is capital letter 'O'
0xBadFace
0xBad_Face
0x_67_7a_2f_cc_40_c6
170141183460469231731687303715884105727
170_141183_460469_231731_687303_715884_105727

_42         // an identifier, not an integer literal
42_         // invalid: _ must separate successive digits
4__2        // invalid: only one _ at a time
0_xBadFace  // invalid: _ must separate successive digits
```

### Floating-point literals
A floating-point literal is a decimal or hexadecimal representation of a floating-point constant.

A floating-point literal consists of an integer part (decimal digits), a decimal point, a fractional part (decimal digits), and an exponent part (e or E followed by an optional sign and decimal digits). One of the integer part or the fractional part may be elided; one of the decimal point or the exponent part may be elided. An exponent value exp scales the mantissa (integer and fractional part) by 10exp.

For readability, an underscore character _ may appear after a base prefix or between successive digits; such underscores do not change the literal value.

```ebnf
float_literal         = decimal_float_literal | hex_float_literal .

decimal_float_literal = decimal_digits "." [ decimal_digits ] [ decimal_exponent ] |
                    decimal_digits decimal_exponent |
                    "." decimal_digits [ decimal_exponent ] .
decimal_exponent  = ( "e" | "E" ) [ "+" | "-" ] decimal_digits .
```

Examples:
```
0.
72.40
072.40       
2.71828
1.e+0
6.67428e-11
1E6
.25
.12345E+5
1_5.         
0.15e+0_2    
```

### Imaginary literals
```ebnf
imaginary_literal = float_literal "i" .
```

Examples:

```
0i
0123i
0o123i
0xabci
0.i
2.71828i
1.e+0i
6.67428e-11i
1E6i
.25i
.12345E+5i
```