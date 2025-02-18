fn check_guess(guess: i32, secret: i32) -> i32{
    if guess == secret{
        0
    }
    else if guess < secret{
        -1
    }
    else{
        1
    }
}

fn main(){
    let mut secret = 37;
    let mut guess = 40;

    let mut attempts = 0;

    loop{
        attempts +=1;
        if check_guess(guess,secret) == 0{
            println!("Your guess is correct!: {}", guess);
            break;
        }
        else if check_guess(guess,secret) == -1{
            println!("Your guess is too low:{}", guess);
            guess += 1;
        }
        else if check_guess(guess,secret) == 1{
            println!("Your guess is too high:{}", guess);
            guess -= 1;
        }
    }
    println!("{}", attempts);
}