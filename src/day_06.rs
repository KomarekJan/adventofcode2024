use crate::day_06::VisitedType::{Cross, Horizontally, Vertically};
use std::cmp::PartialEq;
use std::fmt;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum VisitedType {
    Horizontally,
    Vertically,
    Cross,
}

impl fmt::Display for VisitedType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let symbol = match self {
            VisitedType::Horizontally => '-',
            VisitedType::Vertically => '|',
            VisitedType::Cross => '+',
        };
        write!(f, "{}", symbol)
    }
}

#[derive(Debug, PartialOrd, Ord, Eq, Clone, Copy, PartialEq)]
enum MapCell {
    Empty,
    Obstacle(bool),
    PatrolPosition,
    Visited(VisitedType),
}

impl fmt::Display for MapCell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let symbol = match self {
            MapCell::Empty => '.',
            MapCell::Obstacle(false) => '#',
            MapCell::Obstacle(true) => 'O',
            MapCell::PatrolPosition => '^',
            MapCell::Visited(visited_type) => visited_type.to_string().chars().next().unwrap(),
        };
        write!(f, "{}", symbol)
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Patrol {
    x: usize,
    y: usize,
    direction: Direction,
}
type Map = Vec<Vec<MapCell>>;

fn load_data() -> (Map, (usize, usize)) {
    let data = include_str!("../data/day_06_basic.txt");
    let data: Vec<_> = data.lines().map(|line| line.to_string()).collect();
    let mut patrol = (0, 0);
    let map: Map = data
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => MapCell::Empty,
                    '#' => MapCell::Obstacle(false),
                    '^' => MapCell::PatrolPosition,
                    c => panic!("Unknown map value {}", c),
                })
                .collect()
        })
        .collect();

    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == MapCell::PatrolPosition {
                patrol = (x, y);
            }
        }
    }

    (map, patrol)
}

fn simulate(map: Map, start_position: (usize, usize)) -> (Map, bool) {
    let mut map = map.clone();
    let mut step_counter: Vec<Vec<u8>> = vec![vec![0; map[0].len()]; map.len()];
    let mut patrol = Patrol {
        x: start_position.0,
        y: start_position.1,
        direction: Direction::Up,
    };

    loop {
        // print!("{}", CLEAR);
        // print_map(&map);
        // sleep(std::time::Duration::from_millis());

        map[patrol.y][patrol.x] = match map[patrol.y][patrol.x] {
            MapCell::Empty | MapCell::PatrolPosition => match patrol.direction {
                Direction::Up => MapCell::Visited(Vertically),
                Direction::Down => MapCell::Visited(Vertically),
                Direction::Left => MapCell::Visited(Horizontally),
                Direction::Right => MapCell::Visited(Horizontally),
            },
            MapCell::Obstacle(_) => {
                return (map, false);
            }
            MapCell::Visited(_) => MapCell::Visited(Cross),
        };

        match patrol.direction {
            Direction::Up => {
                if patrol.y == 0 {
                    break;
                }

                if let MapCell::Obstacle(_) = map[patrol.y - 1][patrol.x] {
                    patrol.direction = Direction::Right;
                } else {
                    patrol.y -= 1;
                    step_counter[patrol.y][patrol.x] += 1;
                }
            }
            Direction::Down => {
                if patrol.y == map.len() - 1 {
                    break;
                }

                if let MapCell::Obstacle(_) = map[patrol.y + 1][patrol.x] {
                    patrol.direction = Direction::Left;
                } else {
                    patrol.y += 1;
                    step_counter[patrol.y][patrol.x] += 1;
                }
            }
            Direction::Left => {
                if patrol.x == 0 {
                    break;
                }

                if let MapCell::Obstacle(_) = map[patrol.y][patrol.x - 1] {
                    patrol.direction = Direction::Up;
                } else {
                    patrol.x -= 1;
                    step_counter[patrol.y][patrol.x] += 1;
                }
            }

            Direction::Right => {
                if patrol.x == map[0].len() - 1 {
                    break;
                }

                if let MapCell::Obstacle(_) = map[patrol.y][patrol.x + 1] {
                    patrol.direction = Direction::Down;
                } else {
                    patrol.x += 1;
                    step_counter[patrol.y][patrol.x] += 1;
                }
            }
        }

        if step_counter[patrol.y][patrol.x] >= 3 {
            return (map, true);
        }
    }

    (map, false)
}

fn print_map(map: &Map) {
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            print!("{}", map[y][x])
        }
        println!();
    }
}

fn count_visited(map: &Map) -> usize {
    let mut count = 0;
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if let MapCell::Visited(_) = map[y][x] {
                count += 1;
            }
        }
    }
    count
}

fn find_obstacles(map: Map, start_position: (usize, usize)) -> usize {
    let mut counter = 0;
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            let mut map_clone = map.clone();
            map_clone[y][x] = MapCell::Obstacle(true);
            let (map_res, cycle) = simulate(map_clone, start_position);
            if cycle {
                print_map(&map_res);
                println!();
                counter += 1;
            }
        }
    }
    counter
}

#[allow(unused)]
pub fn solution() {
    let (mut map, start_position) = load_data();

    let (simulated_map, _) = simulate(map.clone(), start_position);
    println!("Part 1: {}", count_visited(&simulated_map));

    let obstacles = find_obstacles(map.clone(), start_position);
    println!("Part 2: {}", obstacles);
}
