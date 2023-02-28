# From Python to Rust and Why You Should Just Skip Trashlang (Go)

## Intro
Are you a Python developer looking for a good systems language to pick up?

We've all been there, Python is great.  Fantastic.  Wonderful.

Python is so intuitive and fluid that it's painless to go from idea to implementation.

Python is expressive and beautiful, with an experience more like writing poetry than code.

But also, Python is slow.  Horribly slow.  Even running the right C bindings under the hood.  Even using PyPy instead of the standard CPython implementation.

Moreso, it's messy.  Scripting and small projects are easy enough to maintain, but anything more substantial and it becomes an increasing hassle--even with type hints and a good static type checker.

The fully dynamic, everything-is-a-class, anything-goes wild west mythology of Python is a bittersweet romance.  It's what you fall in love with at first.  "I can change anything! Do anything!  Rules do not apply to me!"  It gives you power--and with it, more than enough rope to hang yourself.

The library ecosystem is unmatched, particularly in the dark realm of data driven software development.  The tooling ecosystem, however, is a disaster.  Dependency management, virtual environments, reproducibility, and lack of defined project structure all coalesce slowly steal your joy.

But really, it's slow.

If you're anything like most Python developers, you start to wonder what's on the other side of the carefully crafted hedges that Python maintains.  What does the world of performant languages look like?  Not so much to leave Python, but to supplement it when necessary.

Python is a great 'glue' language, perfect for binding together disparate processes and controlling workflow--but those disparate processes being controlled usually need a bit more juice behind them.

So here we are, once again, looking for the next language to take up some slack.

## The Other Languages
When it comes to the hunt for a second language, there are three (well, more like 2.5) big contenders that the Pythonista starts to look at: Go, Julia, and Scala.

Scala really only for data engineers--and they quickly learn that PySpark really isn't that bad.

Julia--the future of numeric and scientific computing.  Julia really is a great language for math-heavy implementations.  In fact, I'd say Julia's out-of-the-box numerical prowess is second-to-none--including MATLAB!  With a relatively straightforward syntax similar to Python, it's easy to see why it gets sold as the up-and-coming data language I really hope it becomes.

Finally we have Go, the promised language.  Created by Google, performant, dead simple syntax, easy concurrency--on the surface it's easy to see why it's become the most popular second place choice of Python developers everywhere.

But!  That's just on the surface and I'm here to convince you that there is a better path.

## Why Not Go
So, why not Go you ask?  It's simple: it's as expressive as a dead fish and as primitive as a fossilized death fish.

As we'll see later in this video, Go is primitive.  There's really no other way to describe it but primitive.  There is only one way to do anything and it is the absolutely most boring way it could possibly be done.  All Go code inevitably becomes a large collection of for-loops and control flow statements.  Even the error handling is dull and uninspiring.

This extreme primativity comes with it's benefits.  It makes it extremely easy to jump in and start reading/writing code. But more than anything else, it's boring.  Hell, the Go developers fought tooth-and-nail for the longest time against implementing something as simple and fundamental 

More and more, I've started to see Python developers that turned to Go now turn to the Rust.  More performant than Go and--excitingly enough for a Python developer--truly expressive.  Just like Python, it's a real pleasure to write Rust code.

Rust gets a bad reputation as a hard-to-learn language, but over the course of this series I hope to show that it's rather undeserved and--once you get over the initial concepts of the borrow checker and lifetimes--a whole new world for the Pythonista.  Once you get a taste of the zero cost abstraction, no garbage collector life it becomes very hard to go back.

But, enough with the words, why don't we jump into some real code and really compare.
