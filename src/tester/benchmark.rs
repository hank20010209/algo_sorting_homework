use crate::sorter;
use crate::tester;
use std::error::Error;
use csv::Writer;
use std::time::{Duration, Instant};

pub fn benchmark (n: usize){
    let case_size_vec: Vec<usize> = vec![25000];
    //let mut wtr = Writer::from_path("log/record.csv")?;
    for case_size in &case_size_vec {
        println!("Case: {}", case_size);
        //wtr.write_record([case_size])?;
        for i in 0..n {
            let arr = tester::generate_vector(*case_size);
            //let mut each_record = Vec::new();
            println!("Run {} --------------------------------------", i);
            
            

            let start:Instant = Instant::now();
            sorter::bubble_sort(&mut (arr.clone()));
            let duration: Duration = start.elapsed();
            println!("Time elapsed in bubble_sort() is: {:?}", duration);
            //each_record.push(duration);

            let start: Instant = Instant::now();
            sorter::insertion_sort(&mut (arr.clone()));
            let duration: Duration = start.elapsed();
            println!("Time elapsed in insertion_sort() is: {:?}", duration);
            //each_record.push(duration);

            let start: Instant = Instant::now();
            sorter::selection_sort(&mut (arr.clone()));
            let duration: Duration = start.elapsed();
            println!("Time elapsed in selection_sort() is: {:?}", duration);
            //each_record.push(duration);

            let start: Instant = Instant::now();
            sorter::quick_sort(&mut (arr.clone()));
            let duration: Duration = start.elapsed();
            println!("Time elapsed in quick_sort() is: {:?}", duration);
            //each_record.push(duration);

            let start: Instant = Instant::now();
            sorter::heap_sort(&mut (arr.clone()));
            let duration: Duration = start.elapsed();
            println!("Time elapsed in heap_sort() is: {:?}", duration);
            //each_record.push(duration);

            // wtr.write_record(&[each_record[0], each_record[1], each_record[2], each_record[3], each_record[4]]);
            // wtr.flush()?;
        }
    }
    //Ok(())
}