# btree-clrs
#### B-Tree implementation according to Cormen's "Introduction to Algorithms" (CLRS)

B-Tree constructor methods are `order(usize)` and `degree(usize)`.
* Order _k_ is an index counting the maximum number of children. An order of _k_ means every node must have a max = _k_, and a min = ceil(k/2).
* The CLRS degree _t_ is an index counting the minimum number of children. A CLRS degree of _t_ means every node must have a min = _t_ and a max = 2t.
