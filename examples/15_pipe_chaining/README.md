# Pipe Chaining

Demonstrating complex data transformations with the pipe operator.

## Demonstrates
- Multi-step pipe chains
- String transformation functions
- Left-to-right data flow
- Function composition

## Expected Output
```
"Hello, world this is Amoskeag."
```

## Pipeline Breakdown
```
"  hello, WORLD! this is AMOSKEAG.  "
  | strip              →  "hello, WORLD! this is AMOSKEAG."
  | downcase           →  "hello, world! this is amoskeag."
  | capitalize         →  "Hello, world! this is amoskeag."
  | replace("amoskeag", "Amoskeag")  →  "Hello, world! this is Amoskeag."
  | replace("!", "")   →  "Hello, world this is Amoskeag."
  | truncate(40)       →  "Hello, world this is Amoskeag."
```

## Pipe Operator Semantics
The pipe operator is **syntactic sugar** for function application:

```
x | f        ≡  f(x)
x | f(a)     ≡  f(x, a)
x | f | g    ≡  g(f(x))
x | f(a) | g(b)  ≡  g(f(x, a), b)
```

## Not Object-Oriented
This is **not** method chaining like:
```
x.f().g().h()  // OO style (not Amoskeag)
```

It's functional composition:
```
x | f | g | h  ≡  h(g(f(x)))  // Amoskeag style
```

## Advantages
- Reads left-to-right (natural flow)
- No method pollution on types
- Functions can be provided by host context
- More flexible than OO chaining
