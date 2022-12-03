mod test;

fn main() {
    let km: f32 = 10.0;
    let m: f32 = 10.0;
    let ktm: f32 = 0.6213712;
    let mtk: f32 = 1.609344;

    println!("10km = {}m", convert_distance(km, ktm));
    println!("10m = {}km", convert_distance(m, mtk));
}

fn convert_distance(a: f32, b: f32) -> f32{
    return a * b;
}