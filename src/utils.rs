use macroquad::prelude::*;
use crate::car::Car;
use crate::traffic_light::Direction;

pub fn count_cars_by_direction(cars: &Vec<Car>) -> [usize; 4] {
    let mut counts = [0; 4];
    for car in cars {
        match car.start_direction {
            Direction::North => counts[0] += 1,
            Direction::East => counts[1] += 1,
            Direction::South => counts[2] += 1,
            Direction::West => counts[3] += 1,
        }
    }
    counts
}

pub fn find_busiest_direction(cars: &Vec<Car>) -> usize {
    let counts = count_cars_by_direction(cars);
    let mut max_index = 0;
    let mut max_count = counts[0];
    for i in 1..4 {
        if counts[i] > max_count {
            max_count = counts[i];
            max_index = i;
        }
    }
    max_index
}

pub fn is_lane_full(cars: &Vec<Car>, direction: Direction, start_pos: Vec2) -> bool {
    let mut cars_in_lane = 0;
    let lane_capacity = 8;
    for car in cars {
        if car.start_direction == direction {
            let distance = match direction {
                Direction::North => (car.position.y - start_pos.y).abs(),
                Direction::South => (start_pos.y - car.position.y).abs(),
                Direction::East => (car.position.x - start_pos.x).abs(),
                Direction::West => (start_pos.x - car.position.x).abs(),
            };
            if distance < 200.0 {
                cars_in_lane += 1;
            }
        }
    }
    cars_in_lane >= lane_capacity
}
