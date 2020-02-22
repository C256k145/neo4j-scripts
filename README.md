## Notes about Neo4j and rusted_cypher crate
- Crate is a bit old/unmaintained(last update on April 25 2017)
- Neo4j Desktop doesnt seem to let more than one graph to run at the same time
- It seems like the only way to start/stop a graph is through the desktop GUI
- Documentation for the crate itself is a bit lackluster, had to make several changes to examples to get them to work, others I still havent gotten to work
- A an entire graph can be created with a .csv file, so it seems like it would end up being very similar to the JSON setup we have currently
- Scripting with it isnt too bad, I can definitely extend this(for now probably a command line based graph creation tool but we'll see)