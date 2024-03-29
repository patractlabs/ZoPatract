## Functions

A function has to be declared at the top level before it is called.

```zopatract
{{#include ../../../zopatract_cli/examples/book/function_declaration.zop}}
```

A function's signature has to be explicitly provided.
Functions can return many values by providing them as a comma-separated list.

```zopatract
{{#include ../../../zopatract_cli/examples/book/multi_return.zop}}
```

### Inference

When defining a variable as the return value of a function, types are provided when the variable needs to be declared:

```zopatract
{{#include ../../../zopatract_cli/examples/book/multi_def.zop}}
```