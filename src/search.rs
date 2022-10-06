use crate::family::*;
use crate::list::List;

pub fn depth_search(graph: &Vec<Vec<u8>>, start: usize, end: usize) -> bool {
    let mut open: List = List::new_one(start);
    let mut closed: List = List::new();
    let mut path: Vec<(usize, usize)> = Vec::new(); // Найденный путь от старта к цели
    let mut counter: i64 = -1; // Число шагов
    let mut found = false; // так как цикл не может прерваться возвращением return true, используется переменная

    while !open.is_empty() {
        counter += 1;
        let x = open.pop_front().expect("Проблемы с pop_front");

        if x == end {
            print!("сделано {} шагов, {}", counter, create_path(path));
            found = true;
            break;
        }

        closed.push_back(x.clone());
        for (node, exist) in graph[x].iter().enumerate() {
            if *exist > 0 && !(closed.exist(node) || open.exist(node)) {
                open.push_front(node);
                path.push((x, node));
            }
        }
    }
    found
}

pub fn depth_search_rec(
    graph: &Vec<Vec<u8>>,
    closed: &mut List,
    path: &mut Family,
    start: usize,
    end: usize,
    i: usize, // Узел на котором находится текущая рекурсия
    counter: &mut i64,
) -> bool {
    if i == end {
        path.push(end);
        return true;
    }

    closed.push_back(i.clone());
    for (node, exist) in graph[i].iter().enumerate() {
        if *exist > 0 && !closed.exist(node) {
            if depth_search_rec(graph, closed, path, start, end, node, counter) {
                *counter += 1;
                path.push(i);
                if i == start {
                    print!("сделано {} шагов, {}", counter, path.path(true));
                }
                return true;
            }
        }
    }
    false
}

pub fn breadth_search(graph: &Vec<Vec<u8>>, start: usize, end: usize) -> bool {
    let mut open: List = List::new_one(start);
    let mut closed: List = List::new();
    let mut path: Vec<(usize, usize)> = Vec::new();
    let mut counter: i64 = -1;
    let mut found = false;

    while !open.is_empty() {
        counter += 1;
        let x = open.pop_front().expect("Проблемы с pop_front");

        if x == end {
            print!("сделано {} шагов, {}", counter, create_path(path));
            found = true;
            break;
        }

        closed.push_back(x.clone());
        for (node, exist) in graph[x].iter().enumerate() {
            if *exist > 0 && !(closed.exist(node) || open.exist(node)) {
                open.push_back(node);
                path.push((x, node));
            }
        }
    }
    found
}

// Для получения пути метода используется вектор кортежей, где первый элемент - родитель
// а второй элемент - потомок
fn create_path(mut path: Vec<(usize, usize)>) -> String {
    // С конца идет восстановление правильного пути
    // Если родительский узел не совпал дочерним кортежа левее от родительского
    // То не совпавший дочерний узел удаляется
    for e in (0..path.len() - 1).rev() {
        if path[e].1 != path[e + 1].0 {
            path.remove(e);
        }
    }
    // Превращение вектора кортежей в строку пути
    let mut str = format!("{}", path.iter().next().unwrap().0);
    for e in path {
        str = format!("{}->{}", str, e.1);
    }
    str
}
