
use crate::game::selectors::equal_place;
use crate::game::state::Ball;
use crate::game::state::Board;
use crate::game::state::Place;

pub type Path = Vec<Place>;

pub fn board_static_to_vec(board: &Board) -> Vec<Vec<Option<Ball>>> {
  let mut board_2d_vec: Vec<Vec<Option<Ball>>> = vec![];
  for row_item in board.iter() {
    let mut row_item_vec = vec![];
    for item in row_item.iter() {
      row_item_vec.push(item.clone());
    }
    board_2d_vec.push(row_item_vec);
  }
  board_2d_vec
}

pub fn find_path(board: &Board, start_place: Place, stop_place: Place) -> Option<Path> {
  let board = board_static_to_vec(board);
  find_path_2d(&board, start_place, stop_place)
}

#[derive(Default, Clone)]
struct PathPlace {
  place: Place,
  place_before: Option<Place>,
  step: usize
}

fn fill_map_of_paths<T>(map: &[Vec<Option<T>>]) -> Vec<Vec<Option<PathPlace>>>{
  let mut map_of_paths: Vec<Vec<Option<PathPlace>>> = vec![];
  for row_item in map.iter() {
    let mut row_item_vec = vec![];
    for _item in row_item.iter() {
      row_item_vec.push(None);
    }
    map_of_paths.push(row_item_vec);
  }
  map_of_paths
}

fn find_path_2d<T>(map: &[Vec<Option<T>>], start_place: Place, stop_place: Place) -> Option<Path> {
  let mut map_of_paths: Vec<Vec<Option<PathPlace>>> = fill_map_of_paths(&map);
  let path_place = PathPlace {
    place: start_place,
    place_before: None,
    step: 0
  };
  let Place { row_index, column_index } = start_place;
  let mut path_places_to_search: Vec<PathPlace> = vec![path_place.clone()];
  map_of_paths[row_index][column_index] = Some(path_place);
  while !path_places_to_search.is_empty() {
    let PathPlace { place, step, .. } = path_places_to_search.remove(0);
    if equal_place(place, stop_place) {
      return Some(get_path(stop_place, &map_of_paths));
    }
    let mut places_to_check = vec![];
    if place.row_index > 0 {
      places_to_check.push(Place { row_index: place.row_index - 1, column_index: place.column_index });
    }
    if place.row_index + 1 < map.len() {
      places_to_check.push(Place { row_index: place.row_index + 1, column_index: place.column_index });
    }
    if place.column_index > 0 {
      places_to_check.push(Place { row_index: place.row_index, column_index: place.column_index - 1 });
    }
    if place.column_index + 1 < map[place.row_index].len(){
      places_to_check.push(Place { row_index: place.row_index, column_index: place.column_index + 1 });
    }
    for place_to_check in places_to_check {
      let row_index = place_to_check.row_index;
      let column_index = place_to_check.column_index;

      if is_unchecked_place(&map, &map_of_paths, place_to_check) {
        let path_place = PathPlace {
          place: place_to_check,
          place_before: Some(place),
          step: step + 1
        };
        map_of_paths[row_index][column_index] = Some(path_place.clone());
        path_places_to_search.push(path_place);
      }
    }
  }
  None
}

fn get_path(place: Place, map_of_paths: &[Vec<Option<PathPlace>>]) -> Path {
  let mut path = vec![];
  let mut maybe_path_place = map_of_paths[place.row_index][place.column_index].clone();
  while let Some(path_place) = maybe_path_place {
    path.push(path_place.place);
    match path_place.place_before {
      Some(place) => {
        maybe_path_place = map_of_paths[place.row_index][place.column_index].clone();
      }
      None => {
        maybe_path_place = None
      }
    }
  }
  path.reverse();
  path
}

fn is_unchecked_place<T>(map: &[Vec<Option<T>>], map_of_paths: &[Vec<Option<PathPlace>>], place: Place) -> bool {
  if map[place.row_index][place.column_index].is_some() {
    return false;
  }
  if map_of_paths[place.row_index][place.column_index].is_some() {
    return false;
  }
  true
}

