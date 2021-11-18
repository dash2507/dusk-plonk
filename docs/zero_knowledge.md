This module explains the concepts behind Zero-Knowledge proofs

# Zero-Knowledge Proof

In the following article you will learn about the definition of a 
Zero-Knowledge proof system, understand what that definition implies and 
ultimately learn about different Zero-Knowledge protocols.

## Definition

A Zero-Knowledge system is a proving-protocol that satisfies the following 
three properties:

1. **Completeness**  
  Every prover with a valid solution will be able to convince a verifier
2. **Soundness**  
  Only a prover with a valid solution can convince a verifier
3. **Zero-Knowledge**  
  The prover proofs they know a secret without revealing any information 
  about the secret itself

## Example

To understand what this definition means, let us look at an example.

Consider the graph [coloring problem][graph coloring]. You have a graph of 
vertices connected by edges. Every edge connects to two vertices and every 
vertex can be connected to any number of edges.  
Now you get three colors and need to color each vertex in such a way, that no 
edge connects to two vertices of the same color.  
Not all graphs have a valid solution to this problem and it happens that while 
it is easy to construct a graph with a valid solution, it is not easy at all to 
determine *if* a given graph has a valid solution or *how* that solution 
might look like.

![colored graph][graph img]

Lets say that I show you a sufficiently big, uncolored graph and tell you that 
I know a way to color the vertices as described above but I want to keep the 
solution a secret.

How can I prove to you that I indeed know a valid solution without sharing any 
information about the solution itself?

First I hide the graph from you and color each vertex in one of three colors 
according to my solution. Then I cover each vertex with a sticker and show 
you the graph with the stickers.  
You now see the same graph and each vertex is covered with a sticker. You are 
not able to make any assumptions regarding my solution.  
Next I ask you to pick a random edge. The two vertices connected to that edge 
should have different colors. I peel off the stickers and show you that indeed 
the two vertices are of different color.  
At this point of course, you will not be convinced yet that my solution is 
valid, after all I might just have been lucky.
So I take another copy of the graph and we repeat the process but each run I 
use a different random distribution of the three colors to color the vertices.
We will repeat this process will until the chances of me being lucky become so 
small that you are convinced.

If we look at the definition of a Zero-Knowledge Proof above, we can see:
1. Our example is **complete**:  
  If I have a valid solution, I will be able to convince you
2. Our example is **sound**:  
  If I don't have a valid solution, I will not be able to convince you
3. Our example reveals **zero knowledge**:  
  Because the colors are reshuffled in every run, you can not draw any 
  conclusions about my solution

## Key Concepts

Conventional proof systems make sure that the prover can not act maliciously.
Zero-Knowledge proof systems take this a step further and also make sure the 
verifier can not act maliciously.  
After a Zero-Knowledge proof is completed, the verifier (or an eavesdropper) 
only knows that the prover is in possession of a certain knowledge but nothing 
more.  
This means that not only is the verifier not able to prove to a third party 
that they are in possession of the knowledge themselves (because they aren't) 
but they also can not prove that the initial prover possess the knowledge.

The second point might sound a bit strange at first, so let us look at it 
in more detail:

Say you recorded the above procedure on camera, so you can later convince 
your very skeptical friends that I know a valid solution.
On the recording they would see that every edge you chose indeed connects two 
vertices of different color. But that won't convince them (they are very 
skeptical) since you and I could have played false and agreed on the vertices 
to be uncovered beforehand.

All this means that with a Zero-Knowledge proof you would for example be able 
to prove that you know a password without ever giving it to any potentially 
malicious other party or eavesdropper.  
Online you could prove that you are you to just one person and they would 
not be able to prove that to anyone else.  
And in the context of blockchain you could prove that a transaction is valid 
without revealing any information about the transaction itself.

## Zero-Knowledge protocols

When first described Zero-Knowledge proofs were interactive, meaning the 
prover and verifier need to interact, like in our example above.
Later it was discovered that a shared reference string is enough to achieve 
computational zero-knowledge without requiring an interaction between prover 
and verifier.

Over time different Zero-Knowledge protocols have been discovered. They have 
beautiful names like *STARK* (Scalable Transparent ARgument of Knowledge), 
*BulletProof*, *SNARG* (Succinct Non-interactive ARGuments) and *SNARK* 
(Succinct Non-interactive ARgument of Knowledge).  
I will not explain the different protocols in detail here.
Instead, I will just give a brief overview of what those fancy abbreviation 
actually mean so you have a staring point for the next internet research 
rabbit hole.

- Scalable:  
  Both prover and verrifier running times are scalable.
- Transparent:  
  Doesn't require a setup-phase which uses a non-public random parameter like 
  the shared reference string mentioned above.
- ARgument:  
  Strictly mathematically speaking, many of the above protocols are no proofs 
  but arguments. However in practice we can still think of them as a proofs.
- Knowledge:  
  The prover proofs that they themselves know the solution. In our above 
  example that means that the prover doesn't prove that a solution exists 
  but that they actually know the solution.
- Succinct:  
  The proofs are relatively small and easy to verify
- Non-interactive:  
  No interaction between prover and verifier is needed



[graph coloring]: https://en.wikipedia.org/wiki/Graph_coloring#Vertex_coloring
[graph img]: https://upload.wikimedia.org/wikipedia/commons/c/c2/Triangulation_3-coloring.svg
