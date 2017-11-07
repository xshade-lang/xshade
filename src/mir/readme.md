# MIR design notes

MIR represents the control flow graph of a unit of computation. Every function call gets inlined during construction of MIR. MIR is a reduced set of xshade language features. Higher level features get desugared into their low level MIR components. MIR gets generated for single shader passes and compute kernels.

## SSA

MIR is __static single assignment form__. This means every variable is assigned to only once. If a variable is assigned to more than once in source code, the compiler will create _temporaries_ to comply with SSA requirements.

## MIR Blocks

Each block of MIR is a __basic block__, a standard compiler term for a continuous sequence of instructions with a single entry point and a single exit point. All interesting control-flow, like branches or loops, happen within a block.

## Recursion

Recursion is forbidden in MIR, in xshade programs and in other GPU languages in general.
