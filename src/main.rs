// testing out a first git change

// doing stuff with structs now

struct container {
    el:String
}

#[derive(Debug)]
struct rectangle {
    width:i32,
    height:i32
}

fn main() {
    //creating a rectangle and calculating the area
    //
    let shape = (55,50);
    let s = rectangle {
        width:50,
        height:125
    };
    println!("width x height:{}",area(shape));
    println!("{:#?}",s);
    println!("width x height:{}",area2(s));

}

fn area(s:(i32,i32)) -> i32{
    s.0*s.1
}
fn area2(r:rectangle)-> i32 {
    r.width*r.height
}
