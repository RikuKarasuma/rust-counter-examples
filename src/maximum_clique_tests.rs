/*
 * 1-7. [5] The maximum clique problem in a graph G = (V, E) asks for the largest
 * subset C of vertices V such that there is an edge in E between every pair of
 * vertices in C. Find a counterexample for the following algorithm: Sort the
 * vertices of G from highest to lowest degree. Considering the vertices in order
 * of degree, for each vertex add it to the clique if it is a neighbor of all vertices
 * currently in the clique. Repeat until all vertices have been considered.
 *
 */
#[cfg(test)]
mod tests {
    use crate::maximum_clique::{find_maximal_clique, find_maximal_clique_obj, Graph, print_graph, Vertex};

    #[test]
    fn should_meet_target_universal_set_smallest_first() {

        let vertices = vec![
            Vertex::new(15f64, 3f64),
            Vertex::new(3f64, 8f64),
            Vertex::new(13f64, 10f64),
            Vertex::new(20f64, 11f64),
        ];

        let width = 25f64;
        let height = 12f64;
        let graph = Graph::new(25f64, 12f64, vertices);

        find_maximal_clique_obj(& graph);
        print_graph(graph);
        // Compare the data of both vectors.
        assert_eq!(false, true);
    }

}