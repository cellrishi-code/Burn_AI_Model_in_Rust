enum Shape{
    Circle(f32),
    Square(f32)
}

fn main(){
    let s1 =Shape::Circle(4.0);
    let s2 = Shape::Square(7.0);
    let ans = area(s1);
    println!("{}", ans)

}
fn area(s:Shape)->f32{
    let x = match s {
        Shape::Circle(r) => 3.14*r*r,
        Shape::Square(l) =>l*l,
    };
    return x;

} 
