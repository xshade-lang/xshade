# Type Checker

## Conventions

### Checking as much as possible

Our new type checker aims to check as much about a program as possible. However it should never create errors due to previous errors. One good indicator of a previous error is that a type field that should contain a type is set to `None`. Just stop your pass at that point with a simple return.
