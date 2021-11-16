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

## Key Concepts

Conventional proof systems make sure that the prover can not act malicious.
Zero-Knowledge proof systems take this a step further and also make sure the 
verifier can not act malicious.
After a Zero-Knowledge proof the verifier (or an eavesdropper) is neither able 
to prove that they posses the knowledge (because they don't) nor that the 
initial prover possess the knowledge.
In our example above, you would not get any information about how I solved the 
graph-color problem, yet you would be convinced that I have a valid solution.
Lets say you recorded the above procedure on camera, so you can later convince 
your very skeptical friends that I know a valid solution.
On the recording they would see how the two uncovered vertices are of 
different color each time but that will not convince them (they are very 
skeptical after all) since you and I could have on a the exact vertices to be 
uncovered beforehand.[^nyt]

Following this logic you would be able to prove that you know a password 
without ever giving it to any potentially malicious other party or 
eavesdropper,
online you could prove that you are you to just one person and they would 
not be able to prove that to anyone else
and in the context of blockchain you could prove that a transaction is valid 
without revealing any information about the transaction itself.

### Interactive vs. Non-Interactive

When first described by Goldwasser, Micali and Rackoff Zero-Knowledge proofs 
were interactive, meaning the prover and verifier need to interact[^zk].
Later Blum, Feldman, and Micali showed that a common random string shared 
between the prover and the verifier is enough to achieve computational 
zero-knowledge without requiring interaction[^ni-zk].


[^nyt]:
	[A New Approach to Protecting Secrets is Discovered](https://www.nytimes.com/1987/02/17/science/a-new-approach-to-protecting-secrets-is-discovered.html) by James Gleick; New York Times; Feb. 17, 1987
[^zk]:
	https://people.csail.mit.edu/silvio/Selected%20Scientific%20Papers/Proof%20Systems/The_Knowledge_Complexity_Of_Interactive_Proof_Systems.pdf
[^ni-zk]:
	
[graph coloring]: https://en.wikipedia.org/wiki/Graph_coloring#Vertex_coloring
[graph img]: https://upload.wikimedia.org/wikipedia/commons/c/c2/Triangulation_3-coloring.svg
