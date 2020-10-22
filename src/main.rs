enum Kind {
    M, P, T, None
}

fn main() {

    println!("Finished");

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

    match h {
        Kind::M => k = d + (d * e as f32 / 10.0),
        Kind::P => k = d + (d * (e - f) as f32/ 25.5),
        Kind::T => k = d - (d * f as f32 / 30.0),
        _ => k = 0.0
    }

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

}