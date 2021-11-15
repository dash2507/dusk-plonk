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

Lets say that I show you a sufficiently big, uncolored graph and tell you that 
I know a way to color the vertices as described above but I want to keep the 
solution a secret. 
How can I prove to you that I indeed know a valid solution without sharing any 
information about the solution itself?

First I would color the vertices of the graph with a random set of three 
colors according to my solution and cover the vertices.
Then I ask you to pick a random edge and show you that the two vertices 
connected to that edge are colored in different colors.
At this point you will most likely not be convinced yet that my solution is 
valid, after all I might just have been lucky.
So I take the graph again, reshuffle the three colors, color the vertices 
according to my solution and let you pick another edge at random.
This process will be repeated until the chances of me being lucky become so 
small, that you are convinced.

If we look at the definition of a Zero-Knowledge Proof above, we can see:
1. Our example is **complete**:  
  Given the above protocol, any prover with a valid solution will be able to 
  convince a verifier of the validity of said solution
2. Our example is **sound**:  
  A prover without a valid solution will be caught
3. Our example reveals **zero knowledge**:  
  Because the colors are reshuffled in every run, the verifier can not make 
  any conclusions about the solution itself

## History

## Key Concepts

With conventional proofs it was always assumed, that the prover could act 
malicious 

The goal of a Zero-Knowledge proof is to proof the knowledge of something in 
such a way, that the verifier (or an eavesdropper) will not be able to 
convince a third party that they posses said knowledge. [^nyt]
This concept has wide ranging consequences.
You could for example proof that you are in possesion of a credit card number 
without giving it away (limiting the risk of fraud uses).
In the internet you can prove that you are a human being (as opposed ot a bot) 
without giving any information about yourself.
Or you can prove that a transaction on a blockchain is valid without revealing 
any details about the transaction.


[^nyt]:
	[A New Approach to Protecting Secrets is Discovered](https://www.nytimes.com/1987/02/17/science/a-new-approach-to-protecting-secrets-is-discovered.html) by James Gleick; New York Times; Feb. 17, 1987

[graph coloring]: https://en.wikipedia.org/wiki/Graph_coloring#Vertex_coloring
[graph img]: https://upload.wikimedia.org/wikipedia/commons/c/c2/Triangulation_3-coloring.svg
