# btree-clrs
#### B-Tree implementation according to Cormen's "Introduction to Algorithms" (CLRS)

B-Tree constructor methods are `order(usize)` and `degree(usize)`.
* The Knuth order _k_ is an index counting the maximum number of children. A Knuth order of _k_ means every node must have a max = _k_, and a min = ceil(k/2).
For example, (3,6) is a B-tree of Knuth order 6.
* The CLRS degree _t_ is an index counting the minimum number of children. A CLRS degree of _t_ means every node must have a min = _t_ and a max = 2t.
For example, (3,6) is a B-tree of CLRS degree 3