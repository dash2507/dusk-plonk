This module explains the concepts behind Zero-Knowledge proofs

# Zero-Knowledge Proof

## Definition

A Zero-Knowledge proof is a protocol that satisfies the following three properties:

1. **Completeness**
  If the prover is honest he will be able to convince the verifier
2. **Soundness**
  Only an honest prover can satisfy a verifier
3. **Zero-Knowledge**
  The prover proofs they know a secret without revealing any information 
  about the secret itself

## Example

Consider the [graph vertex coloring problem][graph coloring] where you have a 
random graph of vertices connected by edges. Every vertex gets assigned one of 
three colors in a way that every edge connects vertices of two different 
colors. 
It happens that it is easy to construct a graph with a valid solution but it is 
not easy to decide whether a given graph has a valid solution or how that 
solution might look like.

![colored graph][graph img]

With a Zero-Knowledge proof I would convince you that I know a valid solution 
to a given graph but will not reveal anything about the solution itself. 
To do this 

## History

## New Concept of a Mathematical Proof

[graph coloring]: https://en.wikipedia.org/wiki/Graph_coloring#Vertex_coloring
[graph img]: https://upload.wikimedia.org/wikipedia/commons/c/c2/Triangulation_3-coloring.svg
