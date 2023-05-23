# WEB ASSEMBLY
A language for the web to run apps at near native speeds.

## Systems vs Application Programming
-   Application programming focuses on software that provides a UI for users
-   Systems programming focuses on hardware

Web assembly is seen as a compilation target.
-   A compilation target is a language targetced by a compiler
    JS can be seen as compilation target when converting from TS to JS
-   We can use our language to write web assembly as it is also a compilation target

## Why RUST?
-   A statically-typed language
-   highly memory efficient
-   Memory safe (rust is aware of when memory should be released)
-   Highly performant

## Variables
-   Snake case
-   All variables are Immutable by default, make them mutable by using mut
-   Variables type is guessed called type inference only if we initialize it

## Most common datatype for number is i32

## Macros
-   A feature in rust for metaprogramming

## Arrays
-   Arrays are fixed

## Vectors
-   Vectors are dynamically-sized
-   Vectors take up most storage

## Ownership
Values assigned to a variable are owned by that variable
rust will automatically deallocate a memory when the variable is not needed anymore
but it can create problems

## Webpack
It is a tool for optimising webapps
we are using to compile rust
1.  Webpack = Core of Webpack
2.  Webpack CLI = A CLI tool for running webpack from the command line
3.  Webpack Dev Server = A development server to preview our app
Web assembly doesnt work with a file protocol, file must be delivered by a server
