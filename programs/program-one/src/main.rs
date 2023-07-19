use std::time::{Instant};
use csv::Writer;
use std::error::Error;

mod plotting;

fn write_to_csv(data: Vec<Vec<u32>>, file_path: &str) -> Result<(), Box<dyn Error>>{
    // open file in write mode
    let file = std::fs::File::create(file_path)?;
    let mut writer = Writer::from_writer(file);

    // write headers
    // writer.write_record(headers.iter().map(|&header| header.to_string()))?;

    // write data as columns x and y
    for i in 0..data[0].len() {
        writer.write_record(data.iter().map(|column| column[i].to_string()))?;
    }

    writer.flush()?;
    Ok(())

}

fn main() -> Result<() , Box<dyn Error>> {
    let max_fibonacci = [5, 15, 20, 25, 30, 35, 40, 45];
    let mut container_of_values: Vec<u32> = Vec::new();
    let mut container_of_counters: Vec<u32> = Vec::new();
    let mut container_of_times: Vec<u32> = Vec::new();
    let plotting = false;

    for value in max_fibonacci.iter() {
        let start_time = Instant::now();
        for i in 1..*value{
            // print them on the same line
            container_of_values.push(fibonacci(i));
        }
        container_of_counters.push(*value);
        let end_time = Instant::now();
        let elapsed_time = end_time - start_time;
        container_of_times.push(elapsed_time.as_millis() as u32);
    }   
   

    println!("Fibonacci values: {:?}", container_of_counters);
    println!("Elapsed times: {:?}", container_of_times);

    let data = vec![container_of_counters, container_of_times];
    write_to_csv(data, "fibonacci.csv")?;
    


    // After processing the CSV data, call the plotting function
    // create a vector of x values and y values to plot using container_of_counters and container_of_times
    if plotting == true {
        let x = [1.0, 2.0, 3.0, 4.0, 5.0];
        let y = [1.0, 2.0, 3.0, 4.0, 5.0];
        println!("x: {:?}", x);
        if let Err(e) = plotting::plot_csv_data(&x, &y) {
            eprintln!("Error while plotting: {}", e);
        }
    } 
    Ok(())
    
}


fn fibonacci(number: u32) -> u32 {
    if number <= 1 {
        number
    }else {
        fibonacci(number - 1) + fibonacci(number - 2)
    }
}

