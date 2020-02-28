# maze-routing

Implementation of several grid routers in Rust. Might not follow the original paper strictly.

## Algorithms

Two-pin nets:

1. Lee
2. Lee (minimum crossing)
3. Lee (minimum edge effect)
4. Hadlock
5. Mikami-Tabuchi

Multiple-pin nets:

1. Lee
2. Mikami-Tabuchi
3. Single Trunk Steiner Tree

## References

1. [GRID ROUTING](http://www.facweb.iitkgp.ac.in/~isg/VLSI/SLIDES/Grid-Routing.pdf)
2. [Lecture of NTU](http://cc.ee.ntu.edu.tw/~jhjiang/instruction/courses/spring11-eda/lec06-3_4p.pdf)
3. [Lecture of NW](http://users.eecs.northwestern.edu/~haizhou/357/lec6.pdf)
4. [EDA Routing](http://cc.ee.ntu.edu.tw/~ywchang/Courses/PD_Source/EDA_routing.pdf)

References of paper are listed in code commits.
