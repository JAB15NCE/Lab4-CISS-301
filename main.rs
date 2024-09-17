/*
Name: Jon Bennett
Date: 02/19/2024
Class: CISS 301

Discription: 
This Program takes a user input of two tables of their choosing whether that is table A, B or C. 
Creates a Key and runs the two sets of elements against each other using Hashets on a 8 byte 64 bit process. 
This only will run any Real numbers, and the user gets to decied how many elements there can be in each set. 
You can not run one or all three tables at the same time. Only two Tables at a time.  

*/

use std::collections::HashSet;
use std::io;

//This is the function that grabs the user input for each table. 

fn table_input(label: &str) -> HashSet<String> {
    println!("\nEnter each element on a new line. Type 'done' to be finished with that table.");
    println!("once two of the tables are filled and only two then program will compare them.");
    println!("\nEnter elements for Table set {} now", label);
    let mut table = HashSet::new();
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim();
        if input == "done" {
            break;
        }
        if let Ok(num) = input.parse::<f64>() {
            table.insert(num.to_string());
        } else {
            println!("Invalid input. Please enter a valid real number or 'done' to finish.");
        }
    }
    table
}

//This will grab the sets and before out put will sort them from smallest to Biggest number for user
//experiance. 

fn to_sort(table: &HashSet<String>) -> Vec<String> {
    let mut nums: Vec<f64> = table.iter()
        .filter_map(|s| s.parse().ok())
        .collect();
    nums.sort_by(|a, b| a.partial_cmp(b).unwrap());
    nums.iter().map(|&num| format!("{}", num as i64)).collect()
}

//This peforms a Right join between the sets that the user inputted for the two tables of their choice.

fn right_join(_table_a: &HashSet<String>, table_b: &HashSet<String>) -> HashSet<String> {
    table_b.clone()
}

//This performs a Left Join between the sets that the user inputted for the two tables of their choice. 

fn left_join(table_a: &HashSet<String>, _table_b: &HashSet<String>) -> HashSet<String> {
    table_a.clone()
}

//This Performs a outer join between the sets that the user inputted for the two tabels of their choice. 

fn outer_join(table_a: &HashSet<String>, table_b: &HashSet<String>) -> HashSet<String> {
    let mut result = table_a.union(table_b).cloned().collect::<Vec<_>>();
    result.sort_by(|a, b| a.partial_cmp(b).unwrap());
    result.into_iter().collect()
}

//This performs a inner join between the sets that the user inputted for the two tables of their choice. 

fn inner_join(table_a: &HashSet<String>, table_b: &HashSet<String>) -> HashSet<String> {
    let mut result = table_a.intersection(table_b).cloned().collect::<Vec<_>>();
    result.sort_by(|a, b| a.partial_cmp(b).unwrap());
    result.into_iter().collect()
}

//This performs a outter difference against the sets that the user inputted for the two tables of their choice. 

fn outer_diff(table_a: &HashSet<String>, table_b: &HashSet<String>) -> HashSet<String> {
    let mut result = table_a.symmetric_difference(table_b).cloned().collect::<Vec<_>>();
    result.sort_by(|a, b| a.partial_cmp(b).unwrap());
    result.into_iter().collect()
}

//Thise performs a Right difference against the sets that the user inputted for the two tabels of their choice.

fn right_diff(table_a: &HashSet<String>, table_b: &HashSet<String>) -> HashSet<String> {
    let mut result = table_b.difference(table_a).cloned().collect::<Vec<_>>();
    result.sort_by(|a, b| a.partial_cmp(b).unwrap());
    result.into_iter().collect()
}

//This performs a Left difference against the sets that the user inputted for the two tables of their choice. 

fn left_diff(table_a: &HashSet<String>, table_b: &HashSet<String>) -> HashSet<String> {
    let mut result = table_a.difference(table_b).cloned().collect::<Vec<_>>();
    result.sort_by(|a, b| a.partial_cmp(b).unwrap());
    result.into_iter().collect()
}

//This is the Main function. This will display the user output for the two sets and the different
//combinations of all the SQL join types. After will provide the user the options to enter other 
//set information or exit the program. 

fn main() {
   'main_loop: loop {
        // Input table A
        let table_a: HashSet<String> = table_input("A");

        // Input table B
        let table_b: HashSet<String> = table_input("B");

        // Input table C
        let table_c: HashSet<String> = table_input("C");

        // Check which tables are filled
        let filled_tables = vec![!table_a.is_empty(), !table_b.is_empty(), !table_c.is_empty()];
        let filled_count = filled_tables.iter().filter(|&&x| x).count();

        // Compare tables as per the inputs 
        match filled_count {
            0 => println!("Please enter elements for at least one table."),
            1 => println!("Please enter elements for one more table."),
            2 => {
                // Inner join
                if !table_a.is_empty() && !table_b.is_empty() {
                    let inner_join = inner_join(&table_a, &table_b);
                    println!("Inner Join ({} ∩ {}): {:?}", "A", "B", to_sort(&inner_join));
                }
                if !table_a.is_empty() && !table_c.is_empty() {
                    let inner_join = inner_join(&table_a, &table_c);
                    println!("Inner Join ({} ∩ {}): {:?}", "A", "C", to_sort(&inner_join));
                }
                if !table_b.is_empty() && !table_c.is_empty() {
                    let inner_join = inner_join(&table_b, &table_c);
                    println!("Inner Join ({} ∩ {}): {:?}", "B", "C", to_sort(&inner_join));
                }
                // Outer join
                if !table_a.is_empty() && !table_b.is_empty() {
                    let outer_join = outer_join(&table_a, &table_b);
                    println!("Outer Join ({} ∪ {}): {:?}", "A", "B", to_sort(&outer_join));
                }
                if !table_a.is_empty() && !table_c.is_empty() {
                    let outer_join = outer_join(&table_a, &table_c);
                    println!("Outer Join ({} ∪ {}): {:?}", "A", "C", to_sort(&outer_join));
                }
                if !table_b.is_empty() && !table_c.is_empty() {
                    let outer_join = outer_join(&table_b, &table_c);
                    println!("Outer Join ({} ∪ {}): {:?}", "B", "C", to_sort(&outer_join));
                }
                // Left join
                if !table_a.is_empty() && !table_b.is_empty() {
                    let left_join = left_join(&table_a, &table_b);
                    println!("Left Join ({}): {:?}", "A", to_sort(&left_join));
                }
                if !table_a.is_empty() && !table_c.is_empty() {
                    let left_join = left_join(&table_a, &table_c);
                    println!("Left Join ({}): {:?}", "A", to_sort(&left_join));
                }
                if !table_b.is_empty() && !table_c.is_empty() {
                    let left_join = left_join(&table_b, &table_c);
                    println!("Left Join ({}): {:?}", "B", to_sort(&left_join));
                }
                // Right join
                if !table_a.is_empty() && !table_b.is_empty() {
                    let right_join = right_join(&table_a, &table_b);
                    println!("Right Join ({}): {:?}", "B", to_sort(&right_join));
                }
                if !table_a.is_empty() && !table_c.is_empty() {
                    let right_join = right_join(&table_a, &table_c);
                    println!("Right Join ({}): {:?}", "C", to_sort(&right_join));
                }
                if !table_b.is_empty() && !table_c.is_empty() {
                    let right_join = right_join(&table_b, &table_c);
                    println!("Right Join ({}): {:?}", "C", to_sort(&right_join));
                }
                // Outer difference (elements present in either table but not in both, switches out with C depending)
                if !table_a.is_empty() && !table_b.is_empty() {
                    let outer_diff = outer_diff(&table_a, &table_b);
                    println!("Outer Difference ({} Δ {}): {:?}", "A", "B", to_sort(&outer_diff));
                }
                if !table_a.is_empty() && !table_c.is_empty() {
                    let outer_diff = outer_diff(&table_a, &table_c);
                    println!("Outer Difference ({} Δ {}): {:?}", "A", "C", to_sort(&outer_diff));
                }
                if !table_b.is_empty() && !table_c.is_empty() {
                    let outer_diff = outer_diff(&table_b, &table_c);
                    println!("Outer Difference ({} Δ {}): {:?}", "B", "C", to_sort(&outer_diff));
                }
                // Right difference (elements present in table B but not in table A, switches out with C depending)
                if !table_a.is_empty() && !table_b.is_empty() {
                    let right_diff = right_diff(&table_a, &table_b);
                    println!("Right Difference ({} \\ {}): {:?}", "B", "A", to_sort(&right_diff));
                }
                if !table_a.is_empty() && !table_c.is_empty() {
                    let right_diff = right_diff(&table_a, &table_c);
                    println!("Right Difference ({} \\ {}): {:?}", "C", "A", to_sort(&right_diff));
                }
                if !table_b.is_empty() && !table_c.is_empty() {
                    let right_diff = right_diff(&table_b, &table_c);
                    println!("Right Difference ({} \\ {}): {:?}", "C", "B", to_sort(&right_diff));
                }
                // Left difference (elements present in table A but not in table B, switches out with C depending)
                if !table_a.is_empty() && !table_b.is_empty() {
                    let left_diff = left_diff(&table_a, &table_b);
                    println!("Left Difference ({} \\ {}): {:?}", "A", "B", to_sort(&left_diff));
                }
                if !table_a.is_empty() && !table_c.is_empty() {
                    let left_diff = left_diff(&table_a, &table_c);
                    println!("Left Difference ({} \\ {}): {:?}", "A", "C", to_sort(&left_diff));
                }
                if !table_b.is_empty() && !table_c.is_empty() {
                    let left_diff = left_diff(&table_b, &table_c);
                    println!("Left Difference ({} \\ {}): {:?}", "B", "C", to_sort(&left_diff));
                }
            }
            _ => println!("Only two tables at a time please."),
        }
        
        //Exiting user interface for user experiance. 
        // Ask if the user wants to enter another set or exit
        'input_loop: loop {
            println!("Do you want to enter another set? (y/n)");
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            let input = input.trim().to_lowercase();

            match input.as_str() {
                "y" => break 'input_loop, // Break out of the nested loop and continue to the next iteration
                "n" => break 'main_loop, // Break out of the main loop and exit the program
                _ => {
                    println!("Try again please. \nPlease enter 'y' to continue or 'n' to exit.");
                    continue; // Go back to the start of the nested loop to ask for input again
                }
            }
        }
    }
}