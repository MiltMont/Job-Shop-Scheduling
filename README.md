# Job shop scheduling problem. 
## Problem statement
We are given $n$ jobs $J_1, \cdots, J_n$ and $m$ machines $M_1, \cdots, M_m$. Each job is composed of several operations $J_i = (O_{i,1}, \cdots, O_{i, n_i})$
such that

1. Each operation $O_{i,j}$ must be processed on one of the $m$ machines.
2. Each operations uses such machine for a fixed duration of time.
3. Operation $O_{i,k}$ needs to be processed before operation $O_{i, k+1}$ can be processed.

Given a scheduling of the operations, our goal is to minimize the make span $C_{\max}$, which is the finish time of the last operation completed 
in the schedule.

# Main strategy

## From Jobs to Graphs

Let $N$ be the total number of operations. We represent this problem in graph theoretic terms. To do this we create a graph whose vertices are the operations $1, \cdots, N$ 
together with vertices $0,N+1$ such that: 

1. An arc $(i,j)$ connects operations $i$ and $j$ if they belong to the same job and $i$ has to be processed immediately before $j$.
2. There is an arc from $0$ to the beginning operation of every job. 

3. There is an arc from the ending operation of each job to $N+1$.

4. All operations that have to be processed by the same machine are fully connected by edges. 

The problem now is to orient these edges and give them a length corresponding to the duration of 
the operation taking into account its origin. Then, a feasible solution is an orientation of the edges in such a way that the resulting graph contains no directed cycle. 

We call the length of the longest path from $0$ to $N+1$ the make span, $C_{\max}$, of this graph.

## Graph representation (The `Solution` class)
// TODO

## The neighborhood of a solution

Recall that $C_{\max}$ denotes the length of a longest path from $0$ to $N+1$. We call such a longest path a *critical route* and we say an operation is *critical* if it belongs to 
a critical route. Given a solution $S$, a neighboring solution $S'$ can be obtained by permuting two successive critical operations. 

## Local and Taboo search.
// TODO


## References
- Taillard, Éric D. 1994. “Parallel Taboo Search Techniques for the Job Shop Scheduling Problem.” ORSA Journal on Computing. Institute for Operations Research and the Management Sciences (INFORMS). [![DOI:10.1287/ijoc.6.2.108](https://zenodo.org/badge/DOI/10.1287/ijoc.6.2.108.svg)](https://doi.org/10.1287/ijoc.6.2.108)
- Huang, Zhi, Lu Sun, and Bumjin Kim. 2008. “Infeasibility Testing and Repairing Algorithms for Job Shop Scheduling.” 2008 4th International Conference on Wireless Communications, Networking and Mobile Computing. IEEE. [![DOI:10.1109/wicom.2008.1657](https://zenodo.org/badge/DOI/10.1109/wicom.2008.1657.svg)](https://doi.org/10.1109/wicom.2008.1657)
- Dell’Amico, Mauro, and Marco Trubian. 1993. “Applying Tabu Search to the Job-Shop Scheduling Problem.” Annals of Operations Research. Springer Science and Business Media LLC. [![DOI:10.1007/bf02023076](https://zenodo.org/badge/DOI/10.1007/bf02023076.svg)](https://doi.org/10.1007/BF02023076)
- Jain, A.S., and S. Meeran. 1999. “Deterministic Job-Shop Scheduling: Past, Present and Future.” European Journal of Operational Research. Elsevier BV.
[![DOI:10.1016/S0377-2217(98)00113-1](https://zenodo.org/badge/DOI/110.1016/S0377-2217(98)00113-1.svg)](https://doi.org/10.1016/S0377-2217(98)00113-1)
