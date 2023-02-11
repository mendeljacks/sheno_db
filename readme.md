Sheno DB Manifesto:

Sheno DB is a rust-based database project which aims to address modularity concerns of traditional
relational databases.

It believes that by breaking down the traditional monolithic architecture of databases into smaller, more focused modules, it can increase the overall reliability extensibility and scalability of database systems. 

For Sheno to be the next-generation database for the modern era, it should address the following concerns:

- Modularity: 
  - Pluggable storage mechanisms (choose RAM, disk or even a google sheet)
  - Multi-syntax allows SQL, OrmaQL, or other custom DSL.
  - Schema based validation optional, using ajv or other validator with paths support
  - Verifications for constraints such as uniq and fk should include paths
  - native support for multi-tenancy using multi hierarchy perms model
  - choosable connection mechanism (http, websockets, pools, pub-sub)
  - UDF support for custom data types, async middlewares for custom insertion logic
  - Configurable index algorithms, including customo ones
  - Configurable internals eg balancing btree, and page size settings
  - replication and sharding strategy should be configurable
  - durability and conistency should be configurable
  - per table config decisions
  - can foreign key to table names and nest tables in each other for namespacing and organization
  
  
- Performance: Sheno should be at least as fast as mysql or postgres
  - Rust based
  - Clean modern code base with no legacy cruft
  - Fancy concurrency and locking primitives inspired by noria db
  - Horizontally scalable (as much as possible with cap theorem and configurable when not)

- Constraints
  - Not doing partially stateful queries like noria
  - Not doing a user interface
  
- Assumptions
  - Will handle query parsing
  - Will handle query planning
  - Will handle query executing 
  - Existing code bases can be utilised if they match project constraints

Bonuses
- Can be used as library directly
- Can be used as a server directly
- Can be resilient to hardware failures by using checksums and raid strategies

https://github.com/stencillogic/db-core/blob/master/src/buf_mgr/lru.rs
This is interesting but doesn't do vacuuming