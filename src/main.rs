use rusted_cypher::GraphClient;
use std::string::String;

fn main() {
    let uname = "neo4j";
    let pass = "first";
    let host = "localhost:7474";
    let uri = format!("http://{}:{}@{}/db/data/", uname, pass, host);
    let graph = GraphClient::connect(uri);
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
        let dob: Result<i32, rusted_cypher::error::GraphError> = row.get("people.born");
        match dob {
            Err(_) => println!("null"),
            _ => println!("{}", dob.unwrap()),
        };

    }

}
