use std::f32::consts::PI;
trait Shape{
    fn area(&self) -> f32;
}

struct Rect{
    width:f32,
    height:f32
}

impl Shape for Rect{
    fn area(&self)->f32{
        return self.width*self.height;
    }
}
struct Circle{
    radius:f32
}

impl Shape for Circle{
    fn area(&self)->f32{
        return PI*self.radius*self.radius;
    }
}

fn print_area_of_shape <T:Shape>(s:T){
    println!("{}",s.area());
}
fn main(){
    let _r=Rect{
        width:10.0,
        height:5.0
    };
    let _c=Circle{
        radius:10.0
    };
    print_area_of_shape(_c);
}