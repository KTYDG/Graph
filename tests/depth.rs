use graph::graph_func::*;
use graph::search::*;

fn create_matrix(start: usize, end: usize, graph: String) -> bool {
    let graph = fill_graph(graph);
    depth_search(&graph, start, end)
}

#[test]
fn empty_graph() {
    assert_eq!(
        false,
        create_matrix(
            0,
            3,
            "0 0 0 0
        0 0 0 0
        0 0 0 0
        0 0 0 0"
                .to_string()
        )
    );
}

#[test]
fn full_graph() {
    assert_eq!(
        true,
        create_matrix(
            0,
            3,
            "0 1 1 1
        1 0 1 1
        1 1 0 1
        1 1 1 0"
                .to_string()
        )
    );
}

#[test]
fn from_first_to_last_straight() {
    assert_eq!(
        true,
        create_matrix(
            0,
            3,
            "0 0 0 1
        0 0 0 0
        0 0 0 0
        0 0 0 0"
                .to_string()
        )
    );
}

#[test]
fn from_first_to_last_through_all() {
    assert_eq!(
        true,
        create_matrix(
            0,
            3,
            "0 1 0 0
        0 0 1 0
        0 0 0 1
        0 0 0 0"
                .to_string()
        )
    );
}

#[test]
fn from_first_to_last_without_path() {
    assert_eq!(
        false,
        create_matrix(
            0,
            3,
            "0 1 0 0
        0 0 1 0
        1 1 0 0
        0 0 0 0"
                .to_string()
        )
    );
}
