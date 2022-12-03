#[cfg(test)]
mod tests{
    use crate::convert_distance;

    #[test]
    fn test_km_to_miles(){
        let km: f32 = 10.0;
        let ktm: f32 = 0.6213712;
        assert_eq!(convert_distance(km, ktm), 6.213712);
    }
    #[test]
    fn test_m_to_km(){
        let m: f32 = 10.0;
        let mtk: f32 = 1.609344;
        assert_eq!(convert_distance(m, mtk), 16.093441);
    }
}