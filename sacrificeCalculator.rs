use std::io::{self, BufRead, Write};
///Made by Li Zhang
///Determine whether it is cheaper to go for a 70% or 30% sac plate
///
///#Examples
///
///```
///let logogramTest = Logogram {
///     protective: 200
///     conceptual: 50
///     fundamental: 65
///     obscure: 99,
///     seventPercentCost: 1.0*200+2.0*50+1.0*99,
///     thirtyPercentCost: 3.0*50+2.0*65+1.0*99,
///     isSeventyBetter: (200) < (6.67*65+24.5*50+2.67*99),
///}
///let input = "200 50 65 99";
///let mut output = Vec::new();
///
///
///sacCalc(input,output,"Testing Logograms")
///
///let answer = {
///let mut iohandlertest = ioHandler {
///reader: &input[..],
///writer: &mut output,
///};
///iohandlertest.prompt("Testing logograms")
///};
///let output = String::from_utf8(output).expect("Not UTF-8");
///
///assert_eq!("Testing logograms", output);
///assert_eq!("200 50 65 99", answer);
///}
///```
///Take in the cost of logograms
///Using the format("protective
///conceptual fundamental obscure")
///
///
pub fn SacCalc<R, W>(mut reader: R, mut writer: W, prompt: &str)
where
    R: BufRead,
    W: Write,
{
    let mut iohandler = ioHandler { reader, writer };
    let mut costs = iohandler.prompt(prompt);
    let costVector = costs
        .split(" ")
        .filter_map(|cost| cost.parse::<f32>().ok()) //filter_map will ignore elements that can't
        //become f32
        .collect::<Vec<_>>();
    if costVector.len() < 4 {
        panic!("Not enough prices")
    }

    let logogramCost = Logogram {
        protective: costVector[0],
        conceptual: costVector[1],
        fundamental: costVector[2],
        //Implement 70% chance is 1protective+2fundamental+1obscure
        //and 30% chance is       3conceptual+2fundamental+1obscure
        obscure: costVector[3],
        thirtyPercentCost: (1.0 * costVector[0] + 2.0 * costVector[1] + 1.0 * costVector[3]),
        seventyPercentCost: (3.0 * costVector[1] + 2.0 * costVector[2] + 1.0 * costVector[3]),
        //Determine if 70% is better using
        //protective < 6.67*fundamental+24.5*conceptual+2.67*obscure
        isSeventyBetter: (costVector[0])
            < (6.67 * costVector[2] + 24.5 * costVector[1] + 2.67 * costVector[3]),
    };
    println!("Print the whole logogram  {:?}", logogramCost);
    println!(
        "isSeventyBetter I wonder {:?}",
        logogramCost.isSeventyBetter
    );
}
///////////////////////////////////////////////
struct ioHandler<R, W> {
    reader: R,
    writer: W,
}
impl<R, W> ioHandler<R, W>
where
    R: io::BufRead,
    W: io::Write,
{
    fn prompt(&mut self, prompt: &str) -> String {
        write!(&mut self.writer, "{}", prompt).expect("Failed to write");
        let mut costs = String::new();
        self.reader
            .read_line(&mut costs)
            .expect("Failed to read line");
        return costs;
    }
}
#[derive(Debug)]
pub struct Logogram {
    protective: f32,
    conceptual: f32,
    fundamental: f32,
    obscure: f32,
    thirtyPercentCost: f32,
    seventyPercentCost: f32,
    isSeventyBetter: bool,
}
