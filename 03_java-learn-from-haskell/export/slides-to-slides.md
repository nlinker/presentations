% What Java can learn from Haskell and vice versa
% Nick Linker
% November 2016, ISSArt

# What Java can learn from Haskell

## Haskell is a big language

1. Expressive (though terse) syntax
2. Immutable data structures
3. Pure functions _with currying_
4. No nulls
5. Algebraic Data Types and pattern matching
6. Type classes
7. Concurrency
8. Tools: GHCI, stack


## The most impact is done by

1. Syntax, type inference
2. ADT & pattern matching
3. Type classes
4. Pure functions and explicit side effects


# What Haskell can learn from Java

## Anyone knows Java

- Much lower learning curve.
  - The first time it is really hard to get even simple things.
  - Haskell forces to control side effects even when it is not needed.
  - Forces to spend much more time on resolving compiler errors.
  - Sometimes errors are hard to comprehend
  - Advanced types (type families and higher kinded types) appear even in ordinary things (like running REST server)
  - Haskell compiler implements a lot of extensions

- Stackoverflow
- However it is hard to get it done right
  - `cow.eat(grass)`, `grass.beEatenBy(cow)` or `field.interact(cow, grass)`
  - subtype polymorphism impossible to

## Convenient tooling

- Developer experience is good
- Intellij Idea
- Although not always good from the long-term perspective


# To use or not to use?

1. Use in the domains when it is really shine: compiler construction, provable correct code.
  1. There are a lot of compilers written in Haskell:
    GHC, Corrode, Elm, Agda, Kaleidoscope, Purescript, Pandoc.
  1. Many libraries to support compiler constructions.
1. It is good for small teams with eagerness to use FP.
1.

# To learn or not to learn?

1. Yes!
  1. Many FP concepts are here is the most explicit way.
  2. It is very different.
  3. Get used to type systems other to Java.
  4. Concurrency and parallelism.
  5. Monads.

# Questions?

![](../img/99_questions.jpg)
