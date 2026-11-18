fn main(){
    let mut v = Vec::new();
    v.push(2);
    v.push(5);
    println!("{:?}",v);
    println!("{:?}",even_filter(v));
}
fn even_filter(v:Vec<i32>)->Vec<i32>{
    let mut new_vec=Vec::new();
    for val in v{
        if val%2==0{
            new_vec.push(val);
        }
    }
    return new_vec;

}
