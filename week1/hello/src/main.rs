fn main() {
    println!("1+...+100={}",get_sum(100));
}

fn get_sum(n:u32)->u32{
    let mut sum:u32=0;

    for i in 1..=n{
        sum+=i;
    }

    return sum;
}
