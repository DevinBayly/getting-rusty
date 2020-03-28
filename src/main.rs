// testing out a first git change

// doing stuff with structs now

struct container {
    el:String
}

fn main() {
    //creating a rectangle and calculating the area
    //
    let shape = (55,50);
    println!("width x height:{}",area(shape));

}

fn area(s:(i32,i32)) -> i32{
    s.0*s.1
}
