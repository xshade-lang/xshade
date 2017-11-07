# MIR design notes

MIR represents the control flow graph of a program. Every function call gets inlined during construction of MIR. MIR is a reduced set of xshade language features. Higher level features get _lowered_ into their lower level MIR components.

## SSA

MIR is a __static single assignment__ form. This means every variable is assigned to only once. If a variable is assigned to more than once in source code, the compiler will create _temporaries_ to comply with SSA requirements. This allows the compiler to trace back the flow of data within a program and eliminate redundant calculations.

## MIR Blocks

Each block of MIR is a __basic block__, a standard compiler term for a continuous sequence of instructions with a single entry point and a single exit point. All interesting control-flow, like branches or loops, happen within a block.

## Recursion

Recursion is forbidden in MIR, in xshade programs and in other GPU languages in general.

## Optimizations

MIR is the layer in which xshade transformations and optimizations are implemented.

### Exhaustive inlining

This transformation actually takes place during MIR construction. Every function called from the entry point gets inlined into the entry point's block.

### Loop unrolling

To speed up a given program, loop instructions will be unrolled to reduce the number of executed instructions during runtime in exchange for bigger binary sizes.

### Elimination of redundant calculations

After _lowering_, _inlining_ and _unrolling_ we have a verbose but complete form of a program. This form makes it possible to aggressively remove all redundant calculations by analyzing the flow of data through the program.
