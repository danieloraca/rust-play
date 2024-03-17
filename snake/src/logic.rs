use log::info;
use rand::seq::SliceRandom;
use serde_json::{json, Value};
use std::collections::HashMap;

use crate::{Battlesnake, Board, Game, Coord};

#[derive(Debug, Clone, Copy)] 
struct Point {
    x: i32,
    y: i32,
}

pub fn info() -> Value {
    info!("INFO");
    return json!({
        "apiversion": "1",
        "author": "exxonV",
        "color": "#53A1A1",
        "head": "o",
        "tail": "x",
    });
}

// start is called when your Battlesnake begins a game
pub fn start(_game: &Game, _turn: &u32, _board: &Board, _you: &Battlesnake) {
    info!("GAME START");
}

// end is called when your Battlesnake finishes a game
pub fn end(_game: &Game, _turn: &u32, _board: &Board, _you: &Battlesnake) {
    info!("GAME OVER");
}

fn find_closest_food_direction(
    board: &Board,
    my_head: &Coord,
    food: &[Coord],
) -> String {
    let mut closest_distance = (board.width + board.height) as i32 * 2; // Initialize to a large value
    let mut closest_food_direction = String::from("right");
    info! {"food: {:?}", food};
  
    for food_point in food {
      info! {"food_point: {:?}", food_point};
      let diff_x: i32 = my_head.x as i32 - food_point.x as i32;
      let diff_y: i32 = my_head.y as i32 - food_point.y as i32;

      let distance_x: i32 = diff_x.abs();
      let distance_y: i32 = diff_y.abs();

      let min_distance = distance_x.min(distance_y);
      info! {"min_distance: {:?}", min_distance};
      info! {"closest_distance: {:?}", closest_distance};
      if min_distance < closest_distance {
        info! {"closest_distance greater than min dist: {:?}", closest_distance};
          closest_distance = min_distance;
          if distance_x < distance_y {
              // Food is closer horizontally
              closest_food_direction = if food_point.x < my_head.x {
                  String::from("left")
              } else {
                  String::from("right")
              };
          } else {
              // Food is closer vertically
              closest_food_direction = if food_point.y < my_head.y {
                  String::from("down")
              } else {
                  String::from("up")
              };
          }
      }
    }

    closest_food_direction
}

pub fn get_move(
  _game: &Game, 
  turn: &u32, 
  board: &Board, 
  you: &Battlesnake
) -> Value {
    let mut is_move_safe: HashMap<_, _> = vec![
        ("up", true),
        ("down", true),
        ("left", true),
        ("right", true),
    ]
    .into_iter()
    .collect();

    // We've included code to prevent your Battlesnake from moving backwards
    let my_head = &you.body[0]; // Coordinates of your head
    let my_neck = &you.body[1]; // Coordinates of your "neck"

    if my_neck.x < my_head.x {
        is_move_safe.insert("left", false);
    } else if my_neck.x > my_head.x {
        is_move_safe.insert("right", false);
    } else if my_neck.y < my_head.y {
        is_move_safe.insert("down", false);
    } else if my_neck.y > my_head.y {
        is_move_safe.insert("up", false);
    }

    // TODO: Step 1 - Prevent your Battlesnake from moving out of bounds
    let board_width = &board.width;
    let board_height = &board.height;
    println!("board_width: {}", board_width);
    println!("board_height: {}", board_height);
    if my_head.x == 0 {
      is_move_safe.insert("left", false);
    }
    if my_head.x == board_width - 1 {
      is_move_safe.insert("right", false);
    }
    if my_head.y == 0 {
      is_move_safe.insert("down", false);
    }
    if my_head.y == board_height - 1 {
      is_move_safe.insert("up", false);
    }

    // TODO: Step 2 - Prevent your Battlesnake from colliding with itself
    let my_body = &you.body;
    println!("my_body: {:?}", my_body);
    for body_part in my_body.iter() {
      if my_head.x == body_part.x + 1 && my_head.y == body_part.y {
        is_move_safe.insert("right", false);
      }
      if my_head.x + 1 == body_part.x && my_head.y == body_part.y {
        is_move_safe.insert("left", false);
      }
      if my_head.y == body_part.y + 1 && my_head.x == body_part.x {
        is_move_safe.insert("up", false);
      }
      if my_head.y + 1 == body_part.y && my_head.x == body_part.x {
        is_move_safe.insert("down", false);
      }
    }

    // TODO: Step 3 - Prevent your Battlesnake from colliding with other Battlesnakes
    let opponents = &board.snakes;
  println!("opponents: {:?}", opponents);

    // Are there any safe moves left?
    let safe_moves = is_move_safe
        .into_iter()
        .filter(|&(_, v)| v)
        .map(|(k, _)| k)
        .collect::<Vec<_>>();

    // TODO: Step 4 - Move towards food instead of random, to regain health and survive longer
    let food = &board.food;
    println!("food: {:?}", food);

      // Find closest food direction
    let mut closest_food_direction = find_closest_food_direction(
      &board,
      &my_head, 
      &food
    );
  println!("closest_food_direction: {}", closest_food_direction);

  closest_food_direction = String::from("right");
  // Are there any safe moves towards food?
  let safe_food_moves: Vec<&str> = safe_moves
      .iter()
      .filter(|&&m| m == closest_food_direction)
      .map(|&m| m)
      .collect();

  // Choose the move based on priority:
  let chosen = if !safe_food_moves.is_empty() {
    println!("gonna get some food");
      // Prioritize safe moves towards food
      *safe_food_moves.choose(&mut rand::thread_rng()).unwrap()
  } else {
    println!("no food");
      // Otherwise, choose a random safe move
      *safe_moves.choose(&mut rand::thread_rng()).unwrap()
  };

    info!("MOVE {}: {}", turn, chosen);
    return json!({ "move": chosen });
}

