// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(non_snake_case)]


fn getPossibleValues(dataTable: Vec<Vec<u8>>, x: usize, y: usize) -> Vec<u8>{
  let mut tempVec: Vec<u8> = vec![0; dataTable.len() * 3];
  let mut IndexTracker: usize = 0;

  // Size of the array is a perfect square, we can assume the height is equal to the width
  // Therefore we handle both y and x based on the same index range
  for Index in 0..dataTable.len(){

    // Check each number in the x columnn
    let value: u8 = dataTable[Index][x];
    if !tempVec.contains(&value){
      tempVec[IndexTracker] = value;
      IndexTracker += 1;
    }

    // Check each number in the y column
    let value: u8 = dataTable[y][Index];
    if !tempVec.contains(&value){
      tempVec[IndexTracker] = value;
      IndexTracker += 1;
    }
  }

  // the predetermined width of box cells
  // error prone sizes, 3, 5, 7, 11, 13, 17
  let preDeterminedBoxSizes: Vec<u8> = vec![0, 1, 2, 0, 2, 0, 3, 0, 4, 3, 5, 0, 4, 0, 7, 5, 4, 0, 6, 0, 5];
  let boxWidth: u8 = preDeterminedBoxSizes[dataTable.len()];
  let boxHeight: u8 = (dataTable.len() as u8) / boxWidth;


  // This broke, so much, and I am very tired. IDC if this is slow and wasteful, this is what is being done.
  let boxColumn: u8;
  let boxRow: u8;
  // 

  boxColumn = f32::floor((x as f32 + 0.9) / boxWidth as f32) as u8;
  let startXIndex: usize = usize::from((boxColumn) * boxWidth);


  boxRow = f32::floor((y as f32 + 0.9) / boxHeight as f32) as u8;
  let startYIndex: usize = usize::from(boxRow * boxHeight);

  // Sweep across entire box

  // Compare values from all other boxes within the main box
  // If all other spots contain a value EXCEPT for current box
  // That value is the only valid answer
  for boxX in 0..boxWidth  as usize{
    for boxY in 0..boxHeight as usize{
      let value: u8 = dataTable[boxY + startYIndex][boxX + startXIndex];
      if !tempVec.contains(&value){
        tempVec[IndexTracker] = value;
        IndexTracker += 1;
      }
    }
  }


  tempVec.retain(|&item| item != 0);

  tempVec

}

fn getUnsolvedSpots(dataTable: &Vec<Vec<u8>>) -> (bool, Vec<Vec<bool>>){
  let mut tempVec: Vec<Vec<bool>> = Vec::new();
  let maxIndex: usize = dataTable.len();
  let mut solved: bool = true;

  for y in 0..maxIndex {
    let mut tempRow: Vec<bool> = vec![false; maxIndex];
    for x in 0..maxIndex {
      if dataTable[y][x] == 0{
        tempRow[x] = true;
        if solved{
          solved = false;
        }
      }
    }
    tempVec.push(tempRow)
  }

  return (solved, tempVec)
}

fn parseResults(results: Vec<u8>, max: u8) -> Vec<u8>{
  let mut possibleValues: Vec<u8> = (1..max+1).collect();

  for value in results.iter(){
    if possibleValues.contains(value){
      possibleValues.retain(|x| x != value)
    }
  }

  possibleValues
}


#[tauri::command]
fn solve(mut dataTable: Vec<Vec<u8>>){
  let maxIndex: usize = dataTable.len();

  let mut unsolved: bool = true;
  let mut unsolvedSpots: Vec<Vec<bool>> = getUnsolvedSpots(&dataTable).1;
  let mut changeCounter: u8 = 0;


  while unsolved{
    println!("ran");
    let mut changed: bool = false;
    for y in 0..maxIndex{
      for x in 0..maxIndex{
        if unsolvedSpots[y][x]{          
          let results: Vec<u8> = parseResults(getPossibleValues(dataTable.clone(), x, y), maxIndex as u8);
          if results.len() == 1 {
            dataTable[y][x] = results[0];
            println!("Selected:{} in position ({}, {})", results[0], x, y);
            changed = true;
            changeCounter = 0;
          }
        }
      }
    }

    let spotResults: (bool, Vec<Vec<bool>>) = getUnsolvedSpots(&dataTable);
    if spotResults.0 == true{
      unsolved = false;
    } else {
      unsolvedSpots = spotResults.1;
      if changed == false{
        if changeCounter == 5{
          println!("Nothing was changed, program stopped");
          unsolved = false;
        }
        changeCounter += 1
      }
    }
  }
  println!("{:?}", dataTable);

}



// Tauri injector
fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![solve])
    .run(tauri::generate_context!())
    .expect("error while building tauri application");
}