use rusted_cypher::GraphClient;
use std::string::String;

fn main() {
    let graph = GraphClient::connect(
        "http://neo4j:first@localhost:7474/db/data/");
    match graph {
        Ok(_) => println!("Connected"),
        Err(e) => panic!("Error: {}", e),
    };
    let result = graph.unwrap().exec(
            "MATCH (people:Person) RETURN people.name, people.born;"
        );

    for row in result.unwrap().rows() {
        let name: String = row.get("people.name").unwrap();
        print!("{}: ", name);
        let DOB: Result<i32, rusted_cypher::error::GraphError> = row.get("people.born");
        match DOB {
            Err(_) => println!("null"),
            _ => println!("{}", DOB.unwrap()),
        };
        
    }

}

