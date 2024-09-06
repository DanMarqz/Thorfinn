use std::collections::HashMap;  // Importa el tipo HashMap de la librería estándar.

pub struct Node {
    children: Vec<Node>,  // Cada nodo puede tener hijos, almacenados en un vector de otros nodos.
    node_type: NodeType,  // El tipo de nodo se define mediante la enumeración NodeType.
}

pub enum NodeType {
    Text(String),  // Un nodo puede ser de tipo texto, almacenando una cadena de texto.
    Element(ElementData),  // O un nodo puede ser un elemento, que tiene más información.
}

pub struct ElementData {
    tag_name: String,  // El nombre de la etiqueta del elemento (por ejemplo, "div", "span").
    attrs: AttrMap,  // Los atributos del elemento, almacenados en un mapa clave-valor.
}

// Alias para un HashMap que mapea Strings a Strings, utilizado para los atributos del elemento.
type AttrMap = HashMap<String, String>;

fn text(data: String) -> Node {
    // Crea un nodo de tipo texto con una cadena de texto proporcionada.
    Node {
        children: Vec::new(),  // Los nodos de texto no tienen hijos.
        node_type: NodeType::Text(data),  // Define el nodo como de tipo Text con el contenido.
    }
}

pub fn elem(tag_name: String, attrs: AttrMap, children: Vec<Node>) -> Node {
    // Crea un nodo de tipo elemento con un nombre de etiqueta, atributos y posibles nodos hijos.
    Node {
        // Los hijos del nodo se pasan como argumento.
        children,
        // Define el nodo como de tipo Element con sus datos.
        node_type: NodeType::Element(ElementData { tag_name, attrs })
    }
}
