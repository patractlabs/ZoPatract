## Control Flow

ZoPatract provides a single thread of execution with a few flow constructs.

### Function calls

Function calls help make programs clear and modular.

Arguments are passed by value.

```zopatract
{{#include ../../../zopatract_cli/examples/book/side_effects.zop}}
```

### If-expressions

An if-expression allows you to branch your code depending on a boolean condition.

```zopatract
{{#include ../../../zopatract_cli/examples/book/if_else.zop}}
```

### For loops

For loops are available with the following syntax:

```zopatract
{{#include ../../../zopatract_cli/examples/book/for.zop}}
```

The bounds have to be constant at compile-time, therefore they cannot depend on execution inputs.

### Assertions

Any boolean can be asserted to be true using the `assert` function.

```zopatract
{{#include ../../../zopatract_cli/examples/book/assert.zop}}
```

If any assertion fails, execution stops as no valid proof could be generated from it.
