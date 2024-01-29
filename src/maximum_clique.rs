/*
 * 1-7. [5] The maximum clique problem in a graph G = (V, E) asks for the largest
 * subset C of vertices V such that there is an edge in E between every pair of
 * vertices in C. Find a counterexample for the following algorithm: Sort the
 * vertices of G from highest to lowest degree. Considering the vertices in order
 * of degree, for each vertex add it to the clique if it is a neighbor of all vertices
 * currently in the clique. Repeat until all vertices have been considered.
 *
 */

use std::f64::consts::PI;
use std::fmt;
use std::fmt::{Formatter, Write};

// Model for our problem set.

pub struct Vertex {
    x: f64,
    y: f64
}

impl fmt::Debug for Vertex {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{{ x: {}, y: {} }}", self.x, self.y)
    }
}

impl Vertex {
    pub fn new(x: f64, y: f64) -> Vertex {
        Vertex {
            x,
            y
        }
    }
}

pub struct Graph {
    width: f64,
    height: f64,
    vertices: Vec<Vertex>
}

impl Graph {

    pub(crate) fn new(width: f64, height: f64, vertices: Vec<Vertex> ) -> Graph {
        Graph {
            width,
            height,
            vertices
        }
    }
}

pub fn find_maximal_clique_obj(graph: &Graph) -> Vec<Vec<Vertex>> {
    find_maximal_clique(graph.width, graph.height, &graph.vertices)
}

pub fn find_maximal_clique(width: f64, height: f64, vertices: &Vec<Vertex>) -> Vec<Vec<Vertex>> {

    // Will host our problem set environment.
    // Example. Each 'x' is a vertex.
    // |    x       x
    // |        x         x
    // |
    // |     x          x
    // --------------------------------------
    // let graph = Graph::new(width, height, vertices);

    let mut maximal_cliques = vec![];
    let mut vertices_as_degrees: Vec<u32> = vec![];


    for x in 0.. vertices.len() {
        let xy_1 = vertices.get(x).unwrap();
        for y in 0.. vertices.len() {
            if x != y {

                let xy_2 = vertices.get(y).unwrap();
                let degrees: i16 = calculate_degrees(&xy_1, &xy_2) as i16;
                if degrees > 0 {
                    println!("Degrees between xy_1: {:?}, xy_2: {:?}, Degrees: {:?}", xy_1, xy_2, degrees);
                }
            }
        }
    }
    // Scan across the graph
    // for x in 0.. width as u32 {
    //
    //     // Scan down the graph
    //     for y in 0.. height as u32 {
    //
    //         let vertice_at_point = graph.vertices.iter().find(|vert| vert.x as u32 == x && vert.y as u32 == y);
    //
    //
    //
    //         if vertice_at_point.is_some() {
    //
    //             // Scan further down this point to pick up any naturally
    //             // known edged points.
    //
    //             // Scan backwards to make clique determinations once we have
    //             // found a point.
    //             for backwards_scan in 0..= x {
    //
    //             }
    //         }
    //     }
    // }

    maximal_cliques
}

fn calculate_degrees(xy_1: &Vertex, xy_2: &Vertex) -> f64 {
    let radians: f64 = f64::atan2(xy_2.y - xy_1.y, xy_2.x - xy_1.x);
    let degrees: f64 = radians * 180.0 / PI;

    degrees
}

pub fn print_graph(graph: Graph) -> () {

    let vertices = graph.vertices;

    create_border(graph.width as u32);
    for y in 0u32.. graph.height as u32 {
        if y > 0 {println!();}

        print!("|");
        for x in 0u32.. graph.width as u32 {
            if vertices.iter().any(|vertex| vertex.x as u32 == x && vertex.y as u32 == y) {
                print!("X");
            } else {
                print!(" ");
            }
        }
        print!("|");
    }
    create_border(graph.width as u32);
}

fn create_border(width: u32) {
    println!();
    print!("*");
    for x_border in 0.. width as u32 {
        print!("_");
    }
    print!("*");
    println!();
}