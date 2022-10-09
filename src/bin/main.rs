use {
    graph::{graph_func::*, list::*, search::*},
    std::{
        env,
        fs::{self},
    },
};

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let path = if args.len() < 2 {
        "matrix.txt".to_string()
    } else {
        args[1].clone().to_string()
    };
    let buffer = fs::read_to_string(path).expect("Файл не найден");

    let graph = fill_graph(buffer);
    print_graph(&graph);

    let start = 0;
    let end = 1;

    print!("\nПоиск в глубину: ");
    if depth_search(&graph, start, end) {
        print!(" путь найден");
    } else {
        print!("пути нет");
    }

    print!("\nПоиск в глубину, рекурсия: ");
    let mut path: Vec<(usize, usize)> = Vec::new();
    if depth_search_rec(
        &graph,
        &mut List::new(),
        &mut path,
        end,
        start,
        -1,
    ) {
        print!(" путь найден");
    } else {
        print!("пути нет");
    }

    print!("\nПоиск в ширину: ");
    if breadth_search(&graph, start, end) {
        print!(" путь найден");
    } else {
        print!("пути нет");
    }
}
