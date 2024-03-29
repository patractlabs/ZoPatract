## Variables

Variables can have any name which does not start with a number.
Variables are mutable, and always passed by value to functions.

### Declaration

Variables need to be declared to be used. Declaration and definition are always combined, so that undefined variables do not exist.
```zopatract
{{#include ../../../zopatract_cli/examples/book/declaration.zop}}
```

### Shadowing

Shadowing is not allowed.
```zopatract
{{#include ../../../zopatract_cli/examples/book/no_shadowing.zop}}
```

### Scope

#### Function

Functions have their own scope
```zopatract
{{#include ../../../zopatract_cli/examples/book/function_scope.zop}}
```

#### For-loop
For-loops have their own scope
```zopatract
{{#include ../../../zopatract_cli/examples/book/for_scope.zop}}
```