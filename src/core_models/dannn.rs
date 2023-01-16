use std::collections::*;
/*
TODO
 - Write node type descriptors for each node type
 - Write functions for each node type
 - Constructors for all parts of the network
   - Node
   - Connections
   - Layers
   - Networks
*/


/*
Every Component of the network is separated into structs, I was told by some random software
engineer dude that this was a slow approach when the DANNN model was first constructed in Python.
This is probably very right, and makes sense too. However, when it comes to a compiled language,
I think it makes more sense than a giant data structure anyways, and might even be faster, due
to compiler optimizations, user optimizations, etc. Anyways, just felt like I should note that.
 */


#[derive(Debug)]
pub enum Error {
    Initialize,
    PrevalenceConfigure,
    InvalidLayer,
    InvalidNode,
    NodeAddition,
    LayerRetrieval,
    NodeRetrieval
}

#[derive(Clone, Debug)]
struct NodeSelectionDescriptor {
    //Used for Storing information on a certain node type, used in the unsigned integer to Node Type Map.
    minimum_connections: Option<usize>, //Minimum input connections that a node can have
    maximum_connections: Option<usize>, //Maximum input connections that a node can have
    description: String, //A String description of the node type
    name: String, //the name of the node
    prevalence: f32, //a float (from 0 to 1) that helps the trainer and the user define how much a certain node will be expressed
}
#[derive(Debug)]
pub struct NodeSelectionTable { //Table of unsigned integers to Node Descriptors object
    node_type_mapping_descriptors: HashMap<usize, NodeSelectionDescriptor>,
}

impl NodeSelectionTable { //Associated functions with NodeSelectionTable - Modify which nodes are used
    pub fn new() -> NodeSelectionTable {
        let mut table = NodeSelectionTable {
            node_type_mapping_descriptors: HashMap::new(),
        };
        return table;
    }
    pub fn enable_all(&mut self){ //Runs all of the functions below that enable larger components of the table
        self.enable_logic();
        self.enable_simple_arithmetic();
        self.enable_advanced_arithmetic();
        self.enable_string_modification();
        self.enable_special_functions();

    } //Enables all node types

    pub fn disable_all(&mut self){ //Same as enable_all but for disabling
        self.disable_logic();
        self.disable_simple_arithmetic();
        self.disable_advanced_arithmetic();
        self.disable_string_modification();
        self.disable_special_functions();
    } //Disables all node types

    pub fn enable_logic(&mut self){
        /*
        Restricting logic states to zero and not zero leaves too many cases, and really ruins the point of the logic operations.
        to fix this, a mean is generated between the largest and smallest number in the set, if the state of any given input is above
        this mean, it is true, below, it is false. This allows for constants to set the parameters for state logic, while still giving
        more capability for the logic system. This will be performed for every node type in this category. The functions will return the greater
        value if true, and the smallest value if false. This is referred to as a "mapped truth value".
         */
        self.node_type_mapping_descriptors.insert(101, NodeSelectionDescriptor {
            minimum_connections: Some(1),
            maximum_connections: None,
            description: String::from("performs an AND comparison of all inputs, omits least input during comparison"),
            name: String::from("AND Gate"),
            prevalence: 1.0,
        });
        self.node_type_mapping_descriptors.insert(102, NodeSelectionDescriptor {
            minimum_connections: Some(1),
            maximum_connections: None,
            description: String::from("performs an OR comparison of all inputs, omits least and greatest input during comparison"),
            name: String::from("OR Gate"),
            prevalence: 1.0,

        });
        self.node_type_mapping_descriptors.insert(103, NodeSelectionDescriptor {
            minimum_connections: Some(1),
            maximum_connections: None,
            description: String::from("performs an NOR comparison of all inputs, omits least and greatest input during comparison"),
            name: String::from("NOR Gate"),
            prevalence: 1.0,

        });
        self.node_type_mapping_descriptors.insert(104, NodeSelectionDescriptor {
            minimum_connections: Some(1),
            maximum_connections: Some(1),
            description: String::from("performs a NOT operation on one input, returns inverse of number to follow mapped truth value"),
            name: String::from("NOT Gate"),
            prevalence: 1.0,

        });
        self.node_type_mapping_descriptors.insert(105, NodeSelectionDescriptor {
            minimum_connections: Some(1),
            maximum_connections: None,
            description: String::from("Performs a NAND comparison of all inputs"),
            name: String::from("NAND Gate"),
            prevalence: 1.0,

        });
        self.node_type_mapping_descriptors.insert(106, NodeSelectionDescriptor {
            minimum_connections: Some(1),
            maximum_connections: None,
            description: String::from("Performs an XOR comparison of all inputs"),
            name: String::from("XOR Gate"),
            prevalence: 1.0,

        });
        self.node_type_mapping_descriptors.insert(107, NodeSelectionDescriptor {
            minimum_connections: Some(1),
            maximum_connections: None,
            description: String::from("tests if all inputs are equal, returns the greatest input if true, and the least input if false"),
            name: String::from("Equality Test"),
            prevalence: 1.0,

        });
        self.node_type_mapping_descriptors.insert(108, NodeSelectionDescriptor {
            minimum_connections: Some(3),
            maximum_connections: Some(3),
            description: String::from("tests if first and second inputs are equal, returns the 3rd input if true, and 0 if false"),
            name: String::from("Equality Test"),
            prevalence: 1.0,

        });
        self.node_type_mapping_descriptors.insert(109, NodeSelectionDescriptor {
            minimum_connections: Some(2),
            maximum_connections: Some(2),
            description: String::from("tests if first value is greater than second value, returns mapped truth value"),
            name: String::from("Greater Than"),
            prevalence: 1.0,

        });
        self.node_type_mapping_descriptors.insert(110, NodeSelectionDescriptor {
            minimum_connections: Some(3),
            maximum_connections: Some(3),
            description: String::from("If the first input is greater than the second input, the 3rd input is returned"),
            name: String::from("Greater Than Gate"),
            prevalence: 1.0,

        });
        self.node_type_mapping_descriptors.insert(111, NodeSelectionDescriptor {
            minimum_connections: Some(2),
             maximum_connections: Some(2),
            description: String::from("tests if first value is less than second value, returns mapped truth value"),
            name: String::from("Less Than"),
            prevalence: 1.0,

        });
        self.node_type_mapping_descriptors.insert(112, NodeSelectionDescriptor {
            minimum_connections: Some(3),
            maximum_connections: Some(3),
            description: String::from("tests if first value is less than second value, returns 3rd value if true, returns 0 if false"),
            name: String::from("Less Than Gate"),
            prevalence: 1.0,

        });
        self.node_type_mapping_descriptors.insert(113, NodeSelectionDescriptor {
            minimum_connections: Some(1),
            maximum_connections: Some(1),
            description: String::from("Passes the input value to the output value"),
            name: String::from("Passthrough"),
            prevalence: 1.0,

        });
        self.node_type_mapping_descriptors.insert(114, NodeSelectionDescriptor {
            minimum_connections: Some(2),
            maximum_connections: Some(2),
            description: String::from("tests if first value is greater than or equal to second value, returns mapped truth value"),
            name: String::from("Greater Than or Equal to"),
            prevalence: 1.0,

        });
        self.node_type_mapping_descriptors.insert(115, NodeSelectionDescriptor {
            minimum_connections: Some(3),
            maximum_connections: Some(3),
            description: String::from("Tests if first value is greater than or equal to second value, returns 3rd value if true, returns 0 if False"),
            name: String::from("Greater Than or Equal to Gate"),
            prevalence: 1.0,

        });
        self.node_type_mapping_descriptors.insert(116, NodeSelectionDescriptor {
            minimum_connections: Some(2),
            maximum_connections: Some(2),
            description: String::from("tests if first value is less than or equal to second value, returns mapped truth value"),
            name: String::from("Less Than Or Equal to"),
            prevalence: 1.0,

        });
        self.node_type_mapping_descriptors.insert(117, NodeSelectionDescriptor {
            minimum_connections: Some(3),
            maximum_connections: Some(3),
            description: String::from("tests if first value is less than or equal to second value, returns 3rd value if true, returns 0 if false"),
            name: String::from("Less Than or Equal to Gate"),
            prevalence: 1.0,

        });
    } //Enables all logic node types

    pub fn disable_logic(&mut self){
        self.node_type_mapping_descriptors.remove(&101);
        self.node_type_mapping_descriptors.remove(&102);
        self.node_type_mapping_descriptors.remove(&103);
        self.node_type_mapping_descriptors.remove(&104);
        self.node_type_mapping_descriptors.remove(&105);
        self.node_type_mapping_descriptors.remove(&106);
        self.node_type_mapping_descriptors.remove(&107);
        self.node_type_mapping_descriptors.remove(&108);
        self.node_type_mapping_descriptors.remove(&109);
        self.node_type_mapping_descriptors.remove(&110);
        self.node_type_mapping_descriptors.remove(&111);
        self.node_type_mapping_descriptors.remove(&112);
        self.node_type_mapping_descriptors.remove(&113);
        self.node_type_mapping_descriptors.remove(&114);
        self.node_type_mapping_descriptors.remove(&115);
        self.node_type_mapping_descriptors.remove(&116);
        self.node_type_mapping_descriptors.remove(&117);
    } //Disables all logic node types

    pub fn enable_simple_arithmetic(&mut self){
        self.node_type_mapping_descriptors.insert(201, NodeSelectionDescriptor {
            minimum_connections: Some(1),
            maximum_connections: None,
            description: String::from("Adds all given numbers together"),
            name: String::from("Add"),
            prevalence: 1.0,

        });
        self.node_type_mapping_descriptors.insert(202, NodeSelectionDescriptor {
            minimum_connections: Some(1),
            maximum_connections: None,
            description: String::from("Subtracts all numbers from the first number"),
            name: String::from("Subtract"),
            prevalence: 1.0,

        });
        self.node_type_mapping_descriptors.insert(203, NodeSelectionDescriptor {
            minimum_connections: Some(1),
            maximum_connections: None,
            description: String::from("Multiplies all numbers"),
            name: String::from("Multiply"),
            prevalence: 1.0,

        });
        self.node_type_mapping_descriptors.insert(204, NodeSelectionDescriptor {
            minimum_connections: Some(1),
            maximum_connections: None,
            description: String::from("Divides all numbers from the first number"),
            name: String::from("Divide"),
            prevalence: 1.0,

        });
    } //Enables all basic arithmetic node types

    pub fn disable_simple_arithmetic(&mut self){
        self.node_type_mapping_descriptors.remove(&201);
        self.node_type_mapping_descriptors.remove(&202);
        self.node_type_mapping_descriptors.remove(&203);
        self.node_type_mapping_descriptors.remove(&204);
    } //Disables all basic simple arithmetic node types

    pub fn enable_advanced_arithmetic(&mut self){
        self.node_type_mapping_descriptors.insert(205, NodeSelectionDescriptor {
            minimum_connections: Some(2),
            maximum_connections: Some(2),
            description: String::from("Takes the n root of the first input, n being the second input"),
            name: String::from("Root"),
            prevalence: 1.0,

        });
        self.node_type_mapping_descriptors.insert(206, NodeSelectionDescriptor {
            minimum_connections: Some(2),
            maximum_connections: Some(2),
            description: String::from("Calculates the first input to the second inputs power"),
            name: String::from("Exponent"),
            prevalence: 1.0,

        });
        self.node_type_mapping_descriptors.insert(207, NodeSelectionDescriptor {
            minimum_connections: Some(1),
            maximum_connections: Some(1),
            description: String::from("Calculates the sine of the first input"),
            name: String::from("Sin"),
            prevalence: 1.0,

        });
        self.node_type_mapping_descriptors.insert(208, NodeSelectionDescriptor {
            minimum_connections: Some(1),
            maximum_connections: Some(1),
            description: String::from("Calculates the cosine of the first input"),
            name: String::from("Cos"),
            prevalence: 1.0,

        });
        self.node_type_mapping_descriptors.insert(209, NodeSelectionDescriptor {
            minimum_connections: Some(1),
            maximum_connections: Some(1),
            description: String::from("Calculates the tangent of the first input"),
            name: String::from("Tan"),
            prevalence: 1.0,

        });
        self.node_type_mapping_descriptors.insert(210, NodeSelectionDescriptor {
            minimum_connections: Some(1),
            maximum_connections: Some(1),
            description: String::from("Calculates the inverse sine of the first input"),
            name: String::from("Arcsin"),
            prevalence: 1.0,

        });
        self.node_type_mapping_descriptors.insert(211, NodeSelectionDescriptor {
            minimum_connections: Some(1),
            maximum_connections: Some(1),
            description: String::from("Calculates the inverse cosine of the first input"),
            name: String::from("Arccos"),
            prevalence: 1.0,

        });
        self.node_type_mapping_descriptors.insert(212, NodeSelectionDescriptor {
            minimum_connections: Some(1),
            maximum_connections: Some(1),
            description: String::from("Calculates the inverse tangent of the first input"),
            name: String::from("Arctan"),
            prevalence: 1.0,

        });
        self.node_type_mapping_descriptors.insert(213, NodeSelectionDescriptor {
            minimum_connections: Some(1),
            maximum_connections: Some(1),
            description: String::from("Returns the absolute value of the first input"),
            name: String::from("Abs"),
            prevalence: 1.0,

        });
        self.node_type_mapping_descriptors.insert(214, NodeSelectionDescriptor {
            minimum_connections: Some(1),
            maximum_connections: Some(1),
            description: String::from("Returns the rounded ceiling of the first input"),
            name: String::from("Ceiling"),
            prevalence: 1.0,

        });
        self.node_type_mapping_descriptors.insert(215, NodeSelectionDescriptor {
            minimum_connections: Some(1),
            maximum_connections: Some(1),
            description: String::from("Returns the rounded floor of the first input"),
            name: String::from("Floor"),
            prevalence: 1.0,

        });
        self.node_type_mapping_descriptors.insert(216, NodeSelectionDescriptor {
            minimum_connections: Some(1),
            maximum_connections: Some(1),
            description: String::from("Returns the rounded first input"),
            name: String::from("Round"),
            prevalence: 1.0,

        });
        self.node_type_mapping_descriptors.insert(217, NodeSelectionDescriptor {
            minimum_connections: Some(1),
            maximum_connections: Some(1),
            description: String::from("Returns the inverted first input"),
            name: String::from("Invert sign"),
            prevalence: 1.0,

        });
        self.node_type_mapping_descriptors.insert(218, NodeSelectionDescriptor {
            minimum_connections: Some(2),
            maximum_connections: Some(2),
            description: String::from("Limits the first input to be no greater than the second input"),
            name: String::from("Maximum Limiter"),
            prevalence: 1.0,

        });
        self.node_type_mapping_descriptors.insert(219, NodeSelectionDescriptor {
            minimum_connections: Some(2),
            maximum_connections: Some(2),
            description: String::from("Limits the first input to be no less than the second input"),
            name: String::from("Minimum Limiter"),
            prevalence: 1.0,

        });
        self.node_type_mapping_descriptors.insert(220, NodeSelectionDescriptor {
            minimum_connections: Some(2),
            maximum_connections: Some(2),
            description: String::from("Returns the modulus of the first and second input"),
            name: String::from("Modulus"),
            prevalence: 1.0,

        });
        self.node_type_mapping_descriptors.insert(221, NodeSelectionDescriptor {
            minimum_connections: Some(1),
            maximum_connections: Some(1),
            description: String::from("Returns the inverse absolute value of the first input"),
            name: String::from("Make Negative"),
            prevalence: 1.0,

        });
        self.node_type_mapping_descriptors.insert(222, NodeSelectionDescriptor {
            minimum_connections: Some(1),
            maximum_connections: Some(1),
            description: String::from("Returns the cosecant of the first input"),
            name: String::from("Cosecant"),
            prevalence: 1.0,

        });
        self.node_type_mapping_descriptors.insert(223, NodeSelectionDescriptor {
            minimum_connections: Some(1),
            maximum_connections: Some(1),
            description: String::from("Returns the secant of the first input"),
            name: String::from("Secant"),
            prevalence: 1.0,

        });
        self.node_type_mapping_descriptors.insert(224, NodeSelectionDescriptor {
            minimum_connections: Some(1),
            maximum_connections: Some(1),
            description: String::from("Returns the cotangent of the first input"),
            name: String::from("Cotangent"),
            prevalence: 1.0,

        });
    } //Enables all advanced arithmetic (Trig, sqrt, exponent, Limiters, Inverters)

    pub fn disable_advanced_arithmetic(&mut self){
        self.node_type_mapping_descriptors.remove(&205);
        self.node_type_mapping_descriptors.remove(&206);
        self.node_type_mapping_descriptors.remove(&207);
        self.node_type_mapping_descriptors.remove(&208);
        self.node_type_mapping_descriptors.remove(&209);
        self.node_type_mapping_descriptors.remove(&210);
        self.node_type_mapping_descriptors.remove(&211);
        self.node_type_mapping_descriptors.remove(&212);
        self.node_type_mapping_descriptors.remove(&213);
        self.node_type_mapping_descriptors.remove(&214);
        self.node_type_mapping_descriptors.remove(&215);
        self.node_type_mapping_descriptors.remove(&216);
        self.node_type_mapping_descriptors.remove(&217);
        self.node_type_mapping_descriptors.remove(&218);
        self.node_type_mapping_descriptors.remove(&219);
        self.node_type_mapping_descriptors.remove(&220);
        self.node_type_mapping_descriptors.remove(&221);
        self.node_type_mapping_descriptors.remove(&222);
        self.node_type_mapping_descriptors.remove(&223);
        self.node_type_mapping_descriptors.remove(&224);
    } //Disables all Advanced arithmetic

    pub fn enable_string_modification(&mut self){
        self.node_type_mapping_descriptors.insert(301, NodeSelectionDescriptor {
            minimum_connections: Some(2),
            maximum_connections: Some(2),
            description: String::from("Removes instances of second value from first value"),
            name: String::from("String Remove"),
            prevalence: 1.0,
        });
        self.node_type_mapping_descriptors.insert(302, NodeSelectionDescriptor {
            minimum_connections: Some(3),
            maximum_connections: Some(3),
            description: String::from("Replace instances of 2nd value from first value with 3rd value"),
            name: String::from("String Replace"),
            prevalence: 1.0,

        });
        self.node_type_mapping_descriptors.insert(303, NodeSelectionDescriptor {
            minimum_connections: Some(2),
            maximum_connections: Some(2),
            description: String::from("Count amount of instances of 2nd Value inside 1st value"),
            name: String::from("String Count"),
            prevalence: 1.0,

        });
        self.node_type_mapping_descriptors.insert(304, NodeSelectionDescriptor {
            minimum_connections: Some(1),
            maximum_connections: None,
            description: String::from("Concatenate all inputs"),
            name: String::from("Concatenate"),
            prevalence: 1.0,

        });
    } //Enables all string modification node types

    pub fn disable_string_modification(&mut self){
        self.node_type_mapping_descriptors.remove(&301);
        self.node_type_mapping_descriptors.remove(&302);
        self.node_type_mapping_descriptors.remove(&303);
        self.node_type_mapping_descriptors.remove(&304);
    } //Disables all string modification node types

    pub fn enable_special_functions(&mut self){
        self.node_type_mapping_descriptors.insert(401, NodeSelectionDescriptor {
            minimum_connections: Some(2),
            maximum_connections: Some(2),
            description: String::from("Generate a random integer between two values"),
            name: String::from("Random Number"),
            prevalence: 1.0,

        });
    } //Enables all node types with "special" characteristics

    pub fn disable_special_functions(&mut self){
        self.node_type_mapping_descriptors.remove(&401);
    } //Disables all node types with "special" characteristics

    pub fn enable_by_id(&mut self, id: usize){
        let mut temporary_template_table = NodeSelectionTable::new();
        temporary_template_table.enable_all();
        let mut descriptor = match temporary_template_table.node_type_mapping_descriptors.get(&id) {
            Some(descriptor) => descriptor,
            None => panic!("Error while retrieving node from template map."),
        };
        self.node_type_mapping_descriptors.insert(id, NodeSelectionDescriptor {
            minimum_connections: descriptor.minimum_connections,
            maximum_connections: descriptor.maximum_connections,
            description: descriptor.description.clone(),
            name: descriptor.name.clone(),
            prevalence: descriptor.prevalence,

        });

    } //Enables a certain node by ID

    pub fn disable_by_id(&mut self, id: usize){
        self.node_type_mapping_descriptors.remove(&id);
    } //Disables a certain node by ID

    pub fn set_prevalence_by_id(&mut self, id: usize, prevalence: f32) -> Result<(), Error>{
        todo!();
        Ok(()) //temp to make my IDE stop whining
    } //Sets the prevalence of a certain node type

    pub fn get_prevalence_by_id(id: usize) -> f32{
        todo!();
        1.0 //again, a temp to make my IDE stop whining
    }

}



#[derive(Debug)]
struct Node<'a> { //Container for node type, data, state, so on
    node_type_id: usize,
    connections: Vec<ConnectionType<'a>>,
    state: f64,
}

impl<'a> Node<'a> {
    fn new(node_type_id: usize) -> Node<'a> {
        Node {
            node_type_id,
            connections: Vec::new(),
            state: 0.0
        }
    }
}


#[derive(Debug)]
enum ConnectionType<'a> {
    Connection(&'a NodeConnection<'a>),
    Constant(&'a Constant),
    Input(&'a Input),
}

pub trait Connection {
    fn get_value(&self) -> f64;
}

impl Connection for NodeConnection<'_> {
    fn get_value(&self) -> f64 {
        self.value
    }
}

impl Connection for Constant {
    fn get_value(&self) -> f64 {
        self.value
    }
}

impl Connection for Input {
    fn get_value(&self) -> f64 {
        self.value
    }
}

#[derive(Debug)]
struct NodeConnection<'a> { //Container for Connection state, connecting Node, and so on
    value: f64,
    source_node: &'a Node<'a>,
}

#[derive(Debug)]
struct Constant { //A similar type (will be united under one trait) to Connection, but represents a constant
    value: f64,
}

#[derive(Debug)]
struct Input { //A similar type (will be united under one trait) to Connection, but represents a connection to one of the networks inputs
    value: f64,
    input_id: usize,
}

#[derive(Debug)]
struct Layer<'a> { //Holds a certain amount of nodes.
    nodes: Vec<Node<'a>>
}

impl<'a> Layer<'a> {
    fn new() -> Layer<'a> {
        Layer {
            nodes: Vec::new()
        }
    }
}

#[derive(Debug)]
struct Network<'a> { //The overall container for the networks, its layers, connections, and nodes
    layers: Vec<Layer<'a>>,
    inputs: Vec<f64>,
    node_selection_table: NodeSelectionTable,
}




impl<'a> Network<'a> {
    fn new() -> Network<'a> {
        let mut network = Network {
            layers: Vec::new(),
            inputs: Vec::new(),
            node_selection_table: NodeSelectionTable::new()
        };
        network
    }
    fn generate_node_to_layer(&mut self, node_type: usize, layer_index: usize) -> Result<&mut Node<'a>, Error> {
        if !(layer_index < self.layers.len()) { //Check to make sure layer index is valid
            return Err(Error::InvalidLayer)
        }
        let mut layer;
        match self.layers.get_mut(layer_index) {
            Some(retrieved_layer) => {layer = retrieved_layer},
            None => {return Err(Error::NodeAddition)}
        }

        let mut new_node: Node = Node::new(node_type);

        layer.nodes.push(new_node);

        return Ok(layer.nodes.last_mut().unwrap());

    }

    fn generate_layer_to_network(&mut self) -> Result<&mut Layer<'a>, Error> {
        let mut new_layer: Layer = Layer::new();

        self.layers.push(new_layer);

        Ok(self.layers.last_mut().unwrap())
    }

    fn get_layer(&mut self, layer_index: usize) -> Result<&mut Layer<'a>, Error> {
        if !(layer_index < self.layers.len()) { //Check to make sure layer index is valid
            return Err(Error::InvalidLayer);
        }
        Ok(self.layers.get_mut(layer_index).unwrap())
    }

    fn get_node(&mut self, layer_index: usize, node_index: usize) -> Result<&mut Node<'a>, Error> {
        if !(layer_index < self.layers.len()) { //Check to make sure layer index is valid
            return Err(Error::InvalidLayer);
        }
        if !(node_index < self.layers.len()) { //Check to make sure node index is valid
            return Err(Error::InvalidNode);
        }
        let mut layer = self.get_layer(layer_index).unwrap();
        return Ok(layer.nodes.get_mut(node_index).unwrap());

    }


}


//Tests
#[cfg(test)]
mod tests {
    use crate::core_models::dannn::run_node;

    //DANNN Core Model Tests
    #[test]
    fn initialize() { //init test
        let node_selection_table = super::NodeSelectionTable::new();
    }

    #[test]
    fn node_selection_table_test() {
        let mut table = super::NodeSelectionTable::new();
        table.enable_all();
        for (key, value) in &table.node_type_mapping_descriptors {
            println!("{} - {}, {}", key, value.name, value.description);
        }
    }

    #[test]
    fn descriptor_enabling_by_id_test() {
        let mut table = super::NodeSelectionTable::new();
        table.enable_by_id(201);
        for (key, value) in &table.node_type_mapping_descriptors {
            let mut min: usize = 0;
            let mut max: usize = 0;
            if let Some(minimum_connections) = value.minimum_connections {
                min = minimum_connections;
            } else {
                min = 0;
            }
            if let Some(maximum_connections) = value.maximum_connections {
                max = maximum_connections;
            } else {
                max = 0;
            }
            println!("{} - {}, {}, {}, {}, {}", key, value.name, value.description, max, min, value.prevalence);
        }
    }

    #[test]
    fn generate_layer_to_network() {
        let mut network = super::Network::new();
        let _ = network.generate_layer_to_network();
        let _ = network.generate_node_to_layer(201, 0);
        assert_eq!(format!("{:?}", network), "Network { layers: [Layer { nodes: [Node { node_type_id: 201, connections: [], state: 0.0 }] }], inputs: [], node_selection_table: NodeSelectionTable { node_type_mapping_descriptors: {} } }")
    }

    #[test]
    fn node_tests() {
        //101
        assert_eq!(run_node(101, vec![10.0, 20.0, 30.0, 40.0]).unwrap(), 10f64);
        assert_eq!(run_node(101, vec![10.0, 30.0, 30.0, 40.0]).unwrap(), 40f64);

        //102
        assert_eq!(run_node(102, vec![10.0, 20.0, 20.0, 40.0]).unwrap(), 10f64);
        assert_eq!(run_node(102, vec![10.0, 30.0, 30.0, 40.0]).unwrap(), 40f64);

        //103
        assert_eq!(run_node(103, vec![10.0, 20.0, 20.0, 40.0]).unwrap(), 40f64);
        assert_eq!(run_node(103, vec![10.0, 30.0, 30.0, 40.0]).unwrap(), 10f64);

    }

}

fn get_mean_of_lowest_and_highest(inputs: &Vec<f64>) -> (f64, f64, f64) {
    if inputs.len() < 1 {
        return (0.0, 0.0, 0.0);
    }
    let mut largest_input = *inputs.get(0).unwrap();
    let mut smallest_input = *inputs.get(0).unwrap();

    for input in inputs {
        if *input > largest_input {
            largest_input = *input;
        }
        if *input < smallest_input {
            smallest_input = *input;
        }
    }

    return ((smallest_input + largest_input)/2.0, smallest_input, largest_input)
}

pub fn run_node(node_type: usize, inputs: Vec<f64>) -> Option<f64> {
    match node_type {
        101 => { //AND Node
            let (logic_mean, smallest_input, largest_input) = get_mean_of_lowest_and_highest(&inputs);
            for input in inputs {
                if input < logic_mean && smallest_input != input{
                    return Some(smallest_input);
                }

            }
            return Some(largest_input);

        } //end ADD Node

        102 => { //OR Node
            let (logic_mean, smallest_input, largest_input) = get_mean_of_lowest_and_highest(&inputs);
            for input in inputs {
                if input >= logic_mean && smallest_input != input && largest_input != input{
                    return Some(largest_input);
                }

            }
            return Some(smallest_input);

        } //end OR Node

        103 => { //NOR Node
            let (logic_mean, smallest_input, largest_input) = get_mean_of_lowest_and_highest(&inputs);
            for input in inputs {
                if input >= logic_mean && smallest_input != input && largest_input != input{
                    return Some(smallest_input);
                }

            }
            return Some(largest_input);
        } //End NOR Node

        104 => { //NOT Node: Since it cannot exactly determine the mapped truth value parameters, it just inverts the number
            for input in inputs {
                return Some(input * -1f64);
            }
            return None;
        } //End NOT Node

        105 => { //NAND Node
            let (logic_mean, smallest_input, largest_input) = get_mean_of_lowest_and_highest(&inputs);
            for input in inputs {
                if input < logic_mean && smallest_input != input {
                    return Some(largest_input);
                }

            }
            return Some(smallest_input);
        } //End NAND Node

        106 => { //XOR Node

        } //End XOR Node
        _ => None
    }
}

