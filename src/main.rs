use rand::RngExt;
use std::time::Instant;

fn quick_sort(arr: &mut [i32]) {
    if arr.len() <= 1 {
        return;
    }
    let pivot_idx = partition(arr);
    quick_sort(&mut arr[0..pivot_idx]);
    quick_sort(&mut arr[pivot_idx + 1..]);
}

fn partition(arr: &mut [i32]) -> usize {
    let pivot_idx = arr.len() - 1;
    let mut i = 0;
    
    for j in 0..pivot_idx {
        if arr[j] <= arr[pivot_idx] {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, pivot_idx);
    i
}

fn main() {
    println!("Rozmiar danych;Czas [ms]");
    
    let sizes = [10_000, 50_000, 100_000, 250_000, 500_000, 1_000_000, 5_000_000, 10_000_000];
    
    let mut rng = rand::rng();

    for &size in &sizes {
        let mut data: Vec<i32> = (0..size).map(|_| rng.random_range(1..1_000_000)).collect();

        let start = Instant::now();
        
        quick_sort(&mut data);
        
        let duration = start.elapsed();

        println!("{};{}", size, duration.as_millis());
    }
}