extern crate rusted_cypher;

use rusted_cypher::GraphClient;
use rusted_cypher::error::GraphError;
use std::process::Command;
use std::io::{self, Write};


fn main() {
    let uname = "neo4j";
    let pass = "first";
    let host = "localhost:7474";
    let uri = format!("http://{}:{}@{}/db/data/", &uname, &pass, &host);
    let graph = GraphClient::connect(uri);
    match graph {
        Ok(_) => println!("Connected Successfully"),
        Err(e) => panic!("Error: {}", e),
    };

    // let labels = get_labels(&graph);
    // for label in labels {
    //     println!("{}", label);
    // }
    // print_all(&graph, &labels);
    print_info(&graph);
}

fn print_info(graph: &Result<GraphClient, GraphError>) {
    /*
    This function prints some info about the graph, just figured it might be helpful to the user.
    */
    let res = graph.as_ref().unwrap().exec(
        "match (n) return 'Number of Nodes: ' + count(n) as output UNION
        match ()-[]->() return 'Number of Relationships: ' + count(*) as output UNION
        CALL db.labels() YIELD label RETURN 'Number of Labels: ' + count(*) AS output UNION
        CALL db.relationshipTypes() YIELD relationshipType  RETURN 'Number of Relationships Types: ' + count(*) AS output UNION
        CALL db.propertyKeys() YIELD propertyKey  RETURN 'Number of Property Keys: ' + count(*) AS output UNION
        CALL db.constraints() YIELD description RETURN 'Number of Constraints:' + count(*) AS output UNION
        CALL db.indexes() YIELD description RETURN 'Number of Indexes: ' + count(*) AS output UNION
        CALL dbms.procedures() YIELD name RETURN 'Number of Procedures: ' + count(*) AS output;"
    );
    for row in res.unwrap().rows() {
        let val: String = row.get("output").unwrap();
        println!("{:?}", val);
    }
}

fn print_all(graph: &Result<GraphClient, GraphError>, labels: &std::vec::Vec<String>) {
    /*
    This just shits out all of the nodes into stdout in a completely unformatted and gross way
    I use pretty print at the end here which helps some but it is a lot of data, with a lot
    of extraneous information. best way to do this is probably to redirect output into a file
    for the user to read through themselves, until i cant find a way to make it less gross
    */
    let mut query = "MATCH ".to_string();
    for label in labels.iter() {
        let addition = format!("({}:{}),", label, label).to_string();
        query.push_str(&addition);
    }

    query.truncate(query.len() - 1);
    query.push_str(" RETURN ");

    for label in labels.iter() {
        let addition = format!("{},", label);
        query.push_str(&addition);
    }

    query.truncate(query.len() - 1);
    query.push_str(";");

    let result = graph.as_ref().unwrap().exec(query);
    println!("{:#?}", result);
}

fn get_labels(graph: &Result<GraphClient, GraphError>) -> std::vec::Vec<String> {
    /*
    This function returns all of the labels avaiable in the database. For the movies example in the
    neo4j desktop, this will return the labels "Movie", and "Person"
    Labels cannot contain spaces, double quotes, or commas
    */
    let mut labels: std::vec::Vec<String> = vec![];
    let res = graph.as_ref().unwrap().exec(
        "CALL db.labels() YIELD label;"
    );
    let res = format!("{:#?}",res.unwrap());
    let mut res_vec = vec![];
    let mut index = 0;
    let mut word = vec![];
    for character in res.chars() {
        if character == '\n' {
            index = index + 1;
            let word_str: String = word.iter().collect();
            res_vec.push(word_str);
            word = vec![];
        }
        else {
            word.push(character);
        }
    }
    /*IMPORTANT: this loop makes it impossible to have spaces, double quotes, or commas in a label*/
    for i in 0..res_vec.len() {
        if res_vec[i] == "            row: [" {
            let temp = &res_vec[i+2];
            let copy_val = temp.to_string()
                .replace(" ", "")
                .replace("\"", "")
                .replace(",", "");
            labels.push(copy_val);
        }
    }
    return labels;
}
