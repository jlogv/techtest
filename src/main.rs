enum Kind {
    M, P, T, None
}

fn calc (h : &Kind, d: f32, e: i32, f : i32) -> f32
{
    match h {
        Kind::M => d + (d * e as f32 / 10.0),
        Kind::P => d + (d * (e - f) as f32/ 25.5),
        Kind::T => d - (d * f as f32 / 30.0),
        _ => 0.0
    }
}

fn main() {

    let a : bool = true;
    let b : bool = true;
    let c : bool = true;
    let d : f32 = 0.0;
    let e : i32 = 0;
    let f : i32 = 0;

    let mut k : f32 = 0.0;

    let expression_set = (a,b,c);

    let mut h: Kind = Kind::None;

    match expression_set {
        (a,b,c) if (a && b && !c) =>
            {
                println!("first");
                h = Kind::M;
            },
        (a,b,c) if (a && b && c) =>
            {
                println!("second");
                h = Kind::P;
            },
        (a,b,c) if (!a && b && c) =>
            {
                println!("third");
                h = Kind::T;
            },
        _=> println!("error"),
    }

    k = calc(&h, d, e, f);

    if let Kind::P = h {
        k = 2.0 * d + (d * e as f32 / 100.0);
    }

    match expression_set {
        (a,b,c) if (a && b && !c) => {
            println!("first too");
            h = Kind::T;
        },
        (a,b,c) if (a && !b && c) => {
            println!("second too");
            h = Kind::M;
        },
        _=> {},
    }

    k = calc(&h, d, e, f);

    if let Kind::P = h {
        k = f as f32 + d + (d * e as f32 / 100.0);
    }

}