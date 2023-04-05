use std::io::BufRead;

#[derive(Copy, Clone, PartialEq)]
enum CellValue {
  Empty,
  Player1,
  Player2,
}

impl std::fmt::Display for CellValue {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      match self {
        CellValue::Player1 => write!(f, "Player 1",),
        CellValue::Player2 => write!(f, "Player 2",),
        CellValue::Empty => write!(f, "Empty",),
      }
    }
}

struct Board {
  board: Vec<[CellValue; 3]>,
  turn: CellValue,
  winner: CellValue,
}

#[derive(Copy, Clone)]
struct Position {
  x: usize,
  y: usize,
}

impl Board {
  fn new() -> Board {   
    Board {
      board: vec![
        [CellValue::Empty,CellValue::Empty,CellValue::Empty,],
        [CellValue::Empty,CellValue::Empty,CellValue::Empty,],
        [CellValue::Empty,CellValue::Empty,CellValue::Empty,],
      ],
      turn: CellValue::Player1,
      winner: CellValue:: Empty,
      
    }
  }

  fn to_string(&self) -> String {
    // Build visual representation of the board for terminal
    let mut board_string = String::from("Current Board:\n\n   1   2   3\n");
    for (j, row) in self.board.iter().enumerate() {
      board_string.push_str(format!("{}  ", j+1).as_str());
      for (i, cell) in row.iter().enumerate() {
        match cell {
          CellValue::Empty => board_string.push_str(" "),
          CellValue::Player1 => board_string.push_str("X"),
          CellValue::Player2 => board_string.push_str("O"),
        }
        if i < row.len() -1 {
          board_string.push_str(" | ")
        }
        
      }
      if j < self.board.len() - 1 {
        board_string.push_str("\n  -----------\n")
      }
    }
    board_string
  }
  
  fn check_win(&mut self) -> CellValue {
    let mut result = CellValue::Empty;
    let mut columns: Vec<Vec<CellValue>> = vec![Vec::new(),Vec::new(),Vec::new()];

    // check rows
    for row in self.board.iter() {
      if row.iter().all(|item| item.clone() == row[0].clone()) {
        result = row[0];
      }
      for (i, item) in row.iter().enumerate() {
        columns[i].push(item.clone());
      }
    }

    //check columns
    for col in columns {
      if col.iter().all(|item| item.clone() == col[0].clone()) {
        result = col[0];
      }
    }

    //check diagonals
    if self.board[0][0] == self.board[1][1] && self.board[1][1] == self.board[2][2] {
      result = self.board[1][1];
    }
    if self.board[2][0] == self.board[1][1] && self.board[1][1] == self.board[0][2] {
      result = self.board[1][1];
    }
    
    self.winner = result;
    result
    
  }

  fn validate_move(&self, pos: Position) -> bool {
    match pos {
      Position { x, y } if (x > 2) | (y > 2) => false,
      _ => true      
    }
  }

  fn record_move(&mut self, position: Position, player: CellValue) -> Result<(), &str> {
    match self.board[position.x][position.y] {
      CellValue::Empty => {self.board[position.x][position.y] = player.clone(); Ok(())},
      _ => Err("Cannot make a move in this space")
    }
  }

  fn next_player(&self) -> CellValue {
    match self.turn {
      CellValue::Player1 => CellValue::Player2,
      CellValue::Player2 => CellValue::Player1,
      CellValue::Empty => CellValue::Empty,
    }
  }
}


fn handle_turn(board: &mut Board) {
  println!("{}, make your move: x,y", board.turn);
  let mut position =  String::new();
  std::io::stdin().lock().read_line(&mut position).ok().expect("nothing here");

  let loc = position.split(",")
            .collect::<Vec<_>>()
            .iter()
            .map(|item| match item.trim().parse::<usize>() {
              Ok(x) => x,
              Err(_) => 6,
            })
            .collect::<Vec<_>>();

  let parsed_position = Position {x: loc[1]-1, y: loc[0]-1};
  if !board.validate_move(parsed_position) {
    println!("Move not valid, {} enter a move that is within x: 1-3, y: 1-3", board.turn);
    return
  }
  
  match board.record_move(parsed_position, board.turn) {
    Ok(_) => board.turn = board.next_player(),
    Err(x) => println!("{}", x),
  }
}

fn play_again() -> bool {
  println!("Keep playing? (y/n):");
  let mut keep_playing =  String::new();
  std::io::stdin().lock().read_line(&mut keep_playing).ok().expect("nothing here");
  if keep_playing.trim().to_lowercase() != "y" {
    return false;
  }
  true
}

fn main() {
  let mut game_over = false;
  let mut playing = true;

  while playing {
    let mut game = Board::new();
    println!("{}\n\n", game.to_string());
    
    while !game_over {
      handle_turn(&mut game);
      game.check_win();
      match game.winner {
        CellValue::Empty => println!("{}\n\n", game.to_string()),
        _ => {
          game_over = true; 
          println!("The winner is {}", game.winner)
          }
      }    
    }
    
    playing = play_again();
    game_over = !playing;
  }
}