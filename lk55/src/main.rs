fn main(){
    //create an instance of the structure
    let c = Circle  {
       radius : 2.0,
    };
    let r = Rectangle  {
       width : 2.0,
       height : 2.0,
    };
    println!("Area of Circle: {}", c.shape_area());
    println!("Area of Rectangle:{}", r.shape_area());
 }
 //declare a structure
 struct Circle {
    radius : f32,
 }
 struct Rectangle{
     width : f32,
     height : f32,
 }
 //declare a trait
 trait Area {
    fn shape_area(&self)->f32;
 }
 //implement the trait
 impl Area for Circle {
    fn shape_area(&self)->f32{
       3.13 * self.radius * self.radius
    }
 }
 impl Area for Rectangle {
    fn shape_area(&self)->f32{
       self.width * self.height
    }
 }