// neural_network.rs

struct Model {
    
}

struct NeuralNetwork {
    input_nodes: u16,
    hidden_nodes: u16,
    output_nodes: u16,
    learning_rate: f32,
}

impl NeuralNetwork {
    pub fn new(input_nodes: u16, hidden_nodes: u16, output_nodes: u16, learning_rate: f32) -> Self {
        NeuralNetwork {
            input_nodes: input_nodes,
            hidden_nodes: hidden_nodes,
            output_nodes: output_nodes,
            learning_rate: learning_rate,
        }
    }

    pub fn from_model(model: Model) -> Self {
        NeuralNetwork {
            input_nodes: 0,
            hidden_nodes: 0,
            output_nodes: 0,
            learning_rate: 0.0,
        }
    }

    fn train(&self) {

    }

    fn query(&self) {
        
    }
}