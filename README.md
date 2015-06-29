# Meta Diff
[![Build Status](https://travis-ci.org/Botev/meta_diff.svg?branch=master)](https://travis-ci.org/Botev/meta_diff)
[![License](http://img.shields.io/:license-GPLv3+-blue.svg)](https://github.com/Botev/symbolic_polynomials/blob/master/LICENSE)

Meta Diff is a tool for automatic differentiation and code generation for developing scalable Machine Learning algorithms across different platforms with a single source file. It is implemented in Rust and will be distributed as binaries for different platforms. 

[Documentation website] (http://botev.github.io/meta_diff/index.html)

## Usage and Installation

When the project is ready it will be distributed as a binary file and will not requrie any form of installation. The usage is from the command line in the format:
```cmd
diff <source_file>
```
The command will create a new folder in the current directory with the name of the input file and in it you will find all of the auto generate sources. Note that these might need to be compiled on their own (in the case of C/C++, CUDA or OpenCL) or might directly be used (in the case of Matlab or Python).

## The source language

The source file follows a subset of Matlab syntax, but has several important differences. Consider the simple source file below for a feed forward network:
```matlab
function [L] = mat(@w1,@w2,x,y)
	h = tanh(w1 dot vertcat(x,1));
	h = tanh(w2 dot vertcat(h,1));
	L = l2(h-y,0);
end
```
The first line defines a funciton `mat` with four arguments - the first two are parameters and the second two are constatns, which means no gradients will be taken with respect to them. All of the standard operation are considered to be an elementwise operation. Thus the operator `*` is the so called Hadammart product. The multiplications in the Linear Algebra sense is implemented via the keyword `dot`. Thus, this snippet calclulates a forward pass over a network, by adding a bias term to each layer using `vertcat`. The last line specify that we are taking an `L2` squared norm of `h-y`. The second argument to the function specifies along which dimension and `0` has the meaning of all dimensions. The direct computation graph induced by this source and generated by the graphviz module file is shown below:
![initial](https://github.com/Botev/meta_diff/blob/master/examples/example.png?raw=true "Initial Graph")
And the graph generated for gradient calculation:
![gradient](https://github.com/Botev/meta_diff/blob/master/examples/example_grad.png?raw=true "Gradient Graph")

## Current stage of development

At the moment the core building blocks of the project have been implemented - the `ComputeGraph` and the parser. 
## Future goals

