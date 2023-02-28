# From Python to ...?
## Skip Go and go straight to Rust

## Why Python Isn't Like Golang

A common refrain throughout the Python community is that Go is the natural choice for Python developers looking to branch out into a more performant language.

This is bullshit.

Python is one of the most expressive languages in existence and this expressiveness is a big reason for it's popularity.  Go--on the other hand--is one of the least expressive languages in existence.  Go is a primitive language and it prides itself of being primitive.  Python is anything but primitive.

True, from the outside looking in Go seems like it's cut from the same clothe as Go.  Go's extreme primitivity means that it's extremely easier for someone new to the language to jump in, read other's code, and write rather idiomatic Go code.  It's this ease of learning that really draws comparisons to Python, as Python reads almost more like pseudocode that you can actually run.  Once you get past the ease-of-reading, you quickly find the closeness of Python and Go to be artificial at best.

In Python, there are a million ways to do any task.  It's not uncommon for people to find ways of packing entire programs into a single line of Python (see AoC problems).  This is the exact opposite of Go.  In Go, your options are limited.  Most things can only be done one way and, when they can be done multiple ways, the number of such ways is limited and generally in the same style as idiomatic Go.

Even with general coding styles, Python can do it all.  Imperative, functional, OOP--everything is on the table.  With Go, there is no OOP or functional--and I don't mean that in the sense that OOP and functional just aren't idiomatic Go, I mean that in the sense that it's literally impossible.

Go simply doesn't support a functional coding style, ripe with iterators, maps, and filters.  Everything in Go is done via for/while-loops and updating mutable variables while looping.

Similarly, Go simply doesn't support a full OOP paradigm.  There are no objects or classes in Go, structs are the most you'll get.  No inheritance, no simple overloading.  Every method you use on a struct must be painstakingly implemented manually.  Even now with Generics (which Go's developers fought tooth-and-nail to implement), that painstaking implementation persists.

(Note: Rust also doesn't support a traditional OOP paradigm, we'll get into that later.  Needless to say, Rust has implemented a system of structs, implementations, and traits that provides the full capabilities of an OOP paradigm, but it's a very different flavor.)
