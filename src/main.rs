use std::io::{self, Write};
use rand::Rng;

fn main() {
    // 1o - X, 11 - O
    let mut board: [u8;9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut is_last_computer = true;
    let mut key_taken;

    let mut rng = rand::thread_rng();

    loop {
        print_board(&board);
        if is_last_computer {
            let mut n = get_number();
            key_taken = is_key_taken(&board, n);
            while key_taken {
                n = get_number();
                key_taken = is_key_taken(&board, n);
            }

            board[n-1] = 10;
            is_last_computer = false;
        } else {
            let mut n = rng.gen_range(1..9);
            key_taken = is_key_taken(&board, n);
            while key_taken {
                n = rng.gen_range(1..9);
                key_taken = is_key_taken(&board, n);
            }
            board[n - 1] = 11;
            is_last_computer = true;
        }

        let (win, val) = check_winner(&board);
        if win {
            println!("{} Is winner", get_value(val));
            print_board(&board);
            break;
        }
    }
}

fn is_key_taken(arr: &[u8;9], n: usize) -> bool {
    arr[n] == 10 || arr[n] == 11 
}

fn get_number() -> usize {
    let mut line = String::new();
    print!("Enter number: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut line).unwrap();
    let trimmed = line.trim();
    trimmed.parse::<usize>().unwrap()
}

fn print_board(arr: &[u8;9]) {
    for i in 0..9 {
        print!("| {} |", get_value(arr[i]));
        if (i + 1) % 3 == 0 {
            println!("\n-------------");
        }
    }
    print!("\n");
}

fn get_value(n: u8) -> String {
    if n == 10 {
        return String::from("X");
    } if n == 11 {
        return String::from("O");
    } else {
        return format!("{}", n);
    }
}

fn check_winner(arr: &[u8;9]) -> (bool, u8) {
    //  First line parrallel case
    if arr[0] == arr[1] && arr[0] == arr[2] {
        return (true, arr[0]);
    }

    // Second line parrallel case
    if arr[3] == arr[4] && arr[3] == arr[5] {
        return (true, arr[3]);
    }

    // Third line parrallel case
    if arr[6] == arr[7] && arr[6] == arr[8] {
        return (true, arr[6]);
    }

    // First vertical case
    if arr[0] == arr[3] && arr[0] == arr[6] {
        return (true, arr[0]);
    }

    // Second vertical case
    if arr[1] == arr[4] && arr[1] == arr[7] {
        return (true, arr[1]);
    }

    // Third vertical case
    if arr[2] == arr[5] && arr[2] == arr[8] {
        return (true, arr[2]);
    }

    // Top left to bottom right diagnoal
    if arr[0] == arr[4] && arr[0] == arr[8] {
        return (true, arr[0]);
    }

    // Top right to bottom left diagnoal
    if arr[2] == arr[4] && arr[2] == arr[8] {
        return (true, arr[2]);
    }


    (false, 0)
}
