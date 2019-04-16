use linear::{M4, Scalar, V2, V3, V4};
// use {closePath, lineTo, moveTo};

pub type Tri2<S> = [V2<S>; 3];
pub type Tri3<S> = [V3<S>; 3];
pub type Tri4<S> = [V4<S>; 3];

// #[derive(Clone)]
// pub enum Face {
//     Quad([[usize; 3]; 4]),
//     Tri([[usize; 3]; 3]),
// }

// pub fn draw_triangles<S: Scalar>(
//     draw_translation: &M4<S>,
//     eye_translation: &M4<S>,
//     triangles: &Vec<[V3<S>; 3]>,
// ) {
//     triangles.iter().for_each(|t| {
//         let e = to_eye_coordinates(eye_translation, t);
//         if is_clipped(&e) { return }
//         let d = to_draw_coordinates(draw_translation, e);
//         if is_clockwise(&d) { return }
//         draw(&d);
//     })
// }

fn to_eye_coordinates<S: Scalar>(m: &M4<S>, t: &Tri3<S>) -> Tri4<S> {
    [
        V4::norm(&M4::mul(m, V3::to_v4(&t[0]))),
        V4::norm(&M4::mul(m, V3::to_v4(&t[1]))),
        V4::norm(&M4::mul(m, V3::to_v4(&t[2]))),
    ]
}

pub fn to_eye_coordinates_2<S: Scalar>(m: &M4<S>, t: Vec<V3<S>>) -> Vec<V4<S>> {
    t.iter()
        .map(|t| V4::norm(&M4::mul(m, V3::to_v4(&t))))
        .collect()
    // vec![]
    // [
    //     V4::norm(&M4::mul(m, V3::to_v4(&t[0]))),
    //     V4::norm(&M4::mul(m, V3::to_v4(&t[1]))),
    //     V4::norm(&M4::mul(m, V3::to_v4(&t[2]))),
    // ]
}

fn is_clipped<S: Scalar>(t: &Tri4<S>) -> bool {
    (S::lte(t[0].x, S::neg(S::ONE)) && S::lte(t[1].x, S::neg(S::ONE)) && S::lte(t[2].x, S::neg(S::ONE)))
        || (S::gte(t[0].x, S::ONE) && S::gte(t[1].x, S::ONE) && S::gte(t[2].x, S::ONE))
        || (S::lte(t[0].y, S::neg(S::ONE)) && S::lte(t[1].y, S::neg(S::ONE)) && S::lte(t[2].y, S::neg(S::ONE)))
        || (S::gte(t[0].y, S::ONE) && S::gte(t[1].y, S::ONE) && S::gte(t[2].y, S::ONE))
        || (S::lte(t[0].z, S::neg(S::ONE)) && S::lte(t[1].z, S::neg(S::ONE)) && S::lte(t[2].z, S::neg(S::ONE)))
        || (S::gte(t[0].z, S::ONE) && S::gte(t[1].z, S::ONE) && S::gte(t[2].z, S::ONE))
}

pub fn is_clipped_2<S: Scalar>(t: &Vec<V4<S>>) -> bool {
    t.iter().all(|i| S::lte(i.x, S::neg(S::ONE))) ||
        t.iter().all(|i| S::gte(i.x, S::ONE)) ||
        t.iter().all(|i| S::lte(i.y, S::neg(S::ONE))) ||
        t.iter().all(|i| S::gte(i.y, S::ONE)) ||
        t.iter().all(|i| S::lte(i.z, S::neg(S::ONE))) ||
        t.iter().all(|i| S::gte(i.z, S::ONE))
}

fn to_draw_coordinates<S: Scalar>(m: &M4<S>, t: Tri4<S>) -> Tri2<S> {
    impl<S: Scalar> M4<S> {
        fn convert(&self, rhs: &V4<S>) -> V2<S> {
            let w = V4::dot(&self.r4, &rhs);
            V2::new(S::div(V4::dot(&self.r1, &rhs), w), S::div(V4::dot(&self.r2, &rhs), w))
        }
    }

    [m.convert(&t[0]), m.convert(&t[1]), m.convert(&t[2])]
}

pub fn to_draw_coordinates_2<S: Scalar>(m: &M4<S>, t: Vec<V4<S>>) -> Vec<V2<S>> {
    impl<S: Scalar> M4<S> {
        fn convert2(&self, rhs: &V4<S>) -> V2<S> {
            let w = V4::dot(&self.r4, &rhs);
            V2::new(S::div(V4::dot(&self.r1, &rhs), w), S::div(V4::dot(&self.r2, &rhs), w))
        }
    }

    t.iter()
        .map(|i| {
            m.convert2(i)
        })
        .collect()
    // [m.convert2(&t[0]), m.convert2(&t[1]), m.convert2(&t[2])]
}

fn is_clockwise<S: Scalar>(d: &Tri2<S>) -> bool {
    let v1 = V2::sub(&d[1], &d[0]);
    let v2 = V2::sub(&d[2], &d[1]);

    // Because of mirroring of viewport this is left handed.
    S::gt(S::sub(S::mul(v1.x, v2.y), S::mul(v2.x, v1.y)), S::ZERO)
}

pub fn is_clockwise_2<S: Scalar>(d: &Vec<V2<S>>) -> bool {
    let v1 = V2::sub(&d[1], &d[0]);
    let v2 = V2::sub(&d[2], &d[1]);

    // Because of mirroring of viewport this is left handed.
    S::gt(S::sub(S::mul(v1.x, v2.y), S::mul(v2.x, v1.y)), S::ZERO)
}

type Parser<'a, A> = dyn Fn(&'a [u8]) -> Result<(A, &'a [u8]), &'a [u8]>;

fn parse_a<'a>(source: &'a [u8]) -> Result<((), &'a [u8]), &'a [u8]> {
    if source.len() > 0 && source[0] as char == 'a' {
        Result::Ok(((), &source[1..]))
    } else {
        Err(source)
    }
}

// fn map<'a, A, B, F: Fn(A) -> B>(p: Parser<'a, A>, f: F) -> impl Fn(&'a [u8]) -> Result<(B, &'a [u8]), &'a [u8]> {
//     ()
// }

fn foo() {}

// fn draw<S: Scalar>(d: &Tri2<S>) {
//     moveTo(S::f32(d[0].x), S::f32(d[0].y));
//     lineTo(S::f32(d[1].x), S::f32(d[1].y));
//     lineTo(S::f32(d[2].x), S::f32(d[2].y));
//     closePath();
// }

// pub fn draw_2<S: Scalar>(d: &Vec<V2<S>>) {
//     moveTo(S::f32(d[0].x), S::f32(d[0].y));

//     (1..d.len()).for_each(|i| {
//         lineTo(S::f32(d[i].x), S::f32(d[i].y));
//         // lineTo(S::f32(d[2].x), S::f32(d[2].y));
//     });

//     closePath();
// }

// use std::str::FromStr;
