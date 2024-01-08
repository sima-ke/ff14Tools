use std::io;
  
    ///Made by Li Zhang
    ///Determine whether it is cheaper to go for a 70% or 30% sac plate
    ///
    ///#Examples
    ///
    ///```
    ///let input = "200 50 65 99";
    ///
    ///```
    ///
    ///let logogramTest = Logogram {
    ///     protective: 200
    ///     conceptual: 50
    ///     fundamental: 65
    ///     obscure: 99,
    ///     seventPercentCost: 1.0*200+2.0*50+1.0*99,
    ///     thirtyPercentCost: 3.0*50+2.0*65+1.0*99,
    ///     isSeventyBetter: (200) < (6.67*65+24.5*50+2.67*99),
    ///}
    ///assert_eq!(logogramTest.seventyPercentCost, 399);
    ///assert_eq!(logogramTest.thirtyPercentCost, 379);
    ///assert_eq!(logogramTest.isSeventyBetter, false)
    ///

    ///Take in the cost of logograms
    ///Using the format("protective
    ///conceptual fundamental obscure")
    ///
    ///
pub fn SacCalc(){
    let mut costs = prompt("Enter the cost of protective, conceptual, fundamental and obscure");
    let costVector = costs.split(" ")
        .filter_map(|cost| cost.parse::<f32>().ok()) //filter_map will ignore elements that can't
                                                     //become f32
        .collect::<Vec<_>>();
    if costVector.len() < 4{
        panic!("Not enough prices")
    }   
    
    let logogramCost = Logogram{
        protective:  costVector[0],
        conceptual:  costVector[1],
        fundamental: costVector[2],
        //Implement 70% chance is 1protective+2fundamental+1obscure
        //and 30% chance is       3conceptual+2fundamental+1obscure
        obscure:     costVector[3],
        thirtyPercentCost: (1.0*costVector[0]+2.0*costVector[1]+1.0*costVector[3]),
        seventyPercentCost: (3.0*costVector[1]+2.0*costVector[2]+1.0*costVector[3]),
        //Determine if 70% is better using
        //protective < 6.67*fundamental+24.5*conceptual+2.67*obscure
        isSeventyBetter: (costVector[0]) < (6.67*costVector[2]+24.5*costVector[1]+2.67*costVector[3]),
    };
    println!("Print the whole logogram  {:?}", logogramCost);
    println!("isSeventyBetter I wonder {:?}", logogramCost.isSeventyBetter)
  }

fn prompt(prompt: &str) -> String {
    println!("Enter the cost of protective, conceptual, fundamental and obscure");
    let mut costs = String::new();
    io::stdin()
      .read_line(&mut costs)
      .expect("Failed to read line");
    return costs;
}
#[derive(Debug)]
 pub struct Logogram {
    protective: f32,
    conceptual: f32,
    fundamental: f32,
    obscure:f32,
    thirtyPercentCost:f32,
    seventyPercentCost:f32,
    isSeventyBetter:bool,
}

