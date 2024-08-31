Pre-requisites:
MIT-6.838-Shape Analysis:
https://www.youtube.com/watch?v=VjyBp6PrvB4&list=PLQ3UicqQtfNtUcdTMLgKSTTOiEsCw2VBW
MIT- 6.033- Computer Systems Engineering:
https://www.youtube.com/watch?v=zm2VP0kHl1M&list=PLCsF5SoER3r_yxaq-gPAjnu4YDBwQKSKp

MapReduce: Simpleified Data Processing on Large Clusters
https://static.googleusercontent.com/media/research.google.com/en//archive/mapreduce-osdi04.pdf

Why Distributed systems are necessary:
- Parallelism
- fault tolerance
- No single Point of Failure Instances.
- Security 

Challenges:
- Performance
- Concurrency
- Partial Failure

Labs:
- MapReduce
- Raft for fault tool
- K/V server
- Sharded K/V service

Infrastructure - Abstractions
1. Storage
2. Communications
3. Computing

Implementation:
1. RPC, threads, concurrency

Performance:
1. Scalability
2. Fault Tolerance
- Availability
- Recoverability: 
    - non-volatile storage use
    - Replication
- Consistency
        - Put(R, V)
        - Get(R) -> V 


<==========+MAP REDUCE+========>
- Think about Google web page indexing task. Sorting all links in order of relevance and writing infrastructure to track and update them require tons of computing power.
- Google therefore developed the map Reduce to tackle this problem.