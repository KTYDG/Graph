use std::{io, io::Write};

// Заполнение графа из файла
pub fn fill_graph(buffer: String) -> Vec<Vec<u8>> {
    let mut graph: Vec<Vec<u8>> = Vec::new();
    for row in buffer.lines() {
        let row = row
            .split_whitespace()
            .map(|element| element.parse().expect("Найдено не числовое значение"))
            .collect();
        graph.push(row);
    }
    graph
}

// Принт матрицы графа в консоль
pub fn print_graph(graph: &Vec<Vec<u8>>) {
    for row in graph {
        for element in row {
            print!("{} ", element);
        }
        print!("\n");
    }
    io::stdout().flush().unwrap();
}