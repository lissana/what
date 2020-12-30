use geo::{Coordinate, LineString, MultiPolygon, Polygon};

use geo_booleanop::boolean::BooleanOp;

//boolean is a trait to Polygon
//https://github.com/21re/rust-geo-booleanop/blob/master/lib/src/boolean/mod.rs#L52
//where
//    F: Float,
//ok, check why doesn't the trait apply to Polygon<f64>
//pub trait BooleanOp1<F, Rhs = Self>
//where
//    F: Float, //this Float is also a trait... that encompasses f64 and f64, defined on the lib
    // https://github.com/21re/rust-geo-booleanop/blob/master/lib/src/boolean/helper.rs#L8

pub fn xy<X: Into<f64>, Y: Into<f64>>(x: X, y: Y) -> Coordinate<f64> {
    Coordinate {
        x: x.into(),
        y: y.into(),
    }
}

fn generate_rect_centered(center: Coordinate<f64>, w: f64, h: f64) -> Polygon<f64> {
    let w_half = w / 2.0;
    let h_half = h / 2.0;
    Polygon::new(
        LineString(vec![
            xy(center.x - w_half, center.y - h_half),
            xy(center.x + w_half, center.y - h_half),
            xy(center.x + w_half, center.y + h_half),
            xy(center.x - w_half, center.y + h_half),
            xy(center.x - w_half, center.y - h_half),
        ]),
        vec![],
    )
}

fn main() {
    let poly1 : geo::Polygon<f64> = generate_rect_centered(Coordinate { x: 10., y: 10. }, 20., 20.);
    let poly2 = generate_rect_centered(Coordinate { x: 15., y: 15. }, 20., 20.);
    println!("so, polygons are created, where is the intersection extention?");
    println!("{:?}", poly1);
    let something = poly1.union(&poly2); 
    println!("{:?}", something);
}
