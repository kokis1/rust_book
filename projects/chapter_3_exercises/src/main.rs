fn main() {
    let temperature: f32 = 100.0;
    let celcius = farenheit_to_celcius(temperature);
    println!("The value of {}F is {}C", temperature, celcius);
    let n = 3;
    let nth_fib = n_th_fib(n);
    println!("the {n}th Fibonacci number is {nth_fib}");
}

fn farenheit_to_celcius(temperature: f32) -> f32 {
    (temperature - 32.0) * 0.55555
}

fn n_th_fib(n: u32) -> u32 {
    let mut first = 0;
    let mut second = 1;
    let mut temporary: u32;

    for _ in 2..=n {
        temporary = first;
        first = second;
        second = temporary + second;
        
    }
    second
}