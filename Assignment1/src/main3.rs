fn is_even(n: i32) -> bool{
    n%2 == 0
}

fn main(){
    let arr = [1,2,77,15,22,35,12,67,85,9];
    for &n in arr.iter(){
        if n%3==0 && n%5==0{
            println!("{}: FizzBuzz", n);
        }
        else if n%3==0{
            println!("{}: Fizz", n);
        }
        else if n%5==0{
            println!("{}: Buzz", n);
        }
        else if is_even(n){
            println!("{}: is even", n);
        }
        else{
            println!("{}: is odd", n);
        }
    }
    let mut sum = 0;
    let mut i = 0;
    while i < arr.len(){
        sum += arr[i];
        i+=1;
    }
    println!("Sum of all numbers:{}", sum);
    let mut largest = arr[0];
    for &n in arr.iter(){
        if n > largest{
            largest = n;
        }
    }
    println!("Largest number: {}", largest);
}