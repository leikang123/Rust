use std :: collections::HashMap;

fn main() {
    let mut  g_map :HashMap<&str,i8> = HashMap::new();
    g_map.insert("xdd",23);
    g_map.insert("kki", 12);
    let str = g_map.get("kki");
    print!("is {:?}",str);

}
