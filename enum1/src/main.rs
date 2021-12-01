
#[derive(Copy,Clone,Debug,PartialEq)]
enum RoughTime{
    InThePast(TimeUnit,u32),
    JustNon,
    InTheFuture(TimeUnit,u32)
}
fn r_t_e(rt:RoughTime) ->String{
    match rt {
        RoughTime::InTheFuture(units,count) =>
        format!("{} {} ago",count,units.plural()),
        RoughTime::InThePast(units,count) =>
        format!("{} {} from now",count,units.plural()),
        RoughTime::JustNon =>
        format!("just now")
        
    }
}

fn main(){
    
}