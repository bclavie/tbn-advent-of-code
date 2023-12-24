use itertools::Itertools;
use z3::ast::{Ast, Int};
use z3::{Config, Context, SatResult, Solver};

fn main() {
    let inputs = include_str!("inputs/24");
    let hailstones: Vec<Hailstone> = inputs.lines().map(Hailstone::from_input).collect_vec();

    let mut total_count = 0;
    for (i, h1) in hailstones.iter().enumerate() {
        for (j, h2) in hailstones.iter().enumerate() {
            // Ignore duplicates
            if j <= i {
                continue;
            }

            if h1.intersects_x_y(h2, 200000000000000, 400000000000000) {
                total_count += 1;
            }
        }
    }

    println!("part1: {}", total_count);

    // Part2 feels extremely silly to do without using a solver like wolfram alpha.
    // It's 6 simultaneous equations with 6 unknowns (Start (xyz), Velocity(xyz)).
    // Or 9 if you want to not eliminate the times from the equations. Either way
    // Using 3 of the inputs is enough to solve the problem:
    // E.g. slapping
    // 19 - 2i = x + i*a;  13+i = y + i*b; 30 - 2*i = z + i*c ;  18- j = x + j*a;  19-j = y + j*b; 22 - 2*j  = z + j*c;  20 -2*k = x + k*a;  25-2*k = y + k*b; 34 - 4*k  = z + k*c;
    // into wolfram alpha gives the solution for the example.
    // After trying to reduce 2 of the 6 equations by hand I decided I could not be arsed, so decided to throw the un-reduced
    // forms into a solver instead.
    // Unfortunately, the actual input has too many characters for the Wolfram alpha to accept :(.
    // So the options are either to:
    // Muck around with a solver like z3: https://docs.rs/z3/latest/z3/
    // Download Wolfram Mathematica and chuck the the equations in.
    // FWIW the Wolfram input for mine was:
    // Solve[327367788702047 + 20 i == x + i a &&
    // 294752664325632 + 51 i == y + i b &&
    // 162080199489440 + 36 i == z + i c &&
    // 349323332347395 - 96 j == x + j a &&
    // 429135322811787 - 480 j == y + j b &&
    // 397812423558610 - 782 j == z + j c &&
    // 342928768632768 -69 k == x + k a &&
    // 275572250031810+ 104 k == y + k b &&
    // 310926883862869- 510 k  == z + k c, {a,b,c,x,y,z,i,j,k}, Integers]

    // For completeness I messed around enough with Z3's API to get the answer albeit still needing
    // to use a 3rd party lib:
    let cfg = Config::new();
    let ctx = Context::new(&cfg);

    let solver = Solver::new(&ctx);
    // Our known constants:
    let point0 = (
        Int::from_i64(&ctx, hailstones[0].start.0),
        Int::from_i64(&ctx, hailstones[0].start.1),
        Int::from_i64(&ctx, hailstones[0].start.2),
    );
    let point1 = (
        Int::from_i64(&ctx, hailstones[1].start.0),
        Int::from_i64(&ctx, hailstones[1].start.1),
        Int::from_i64(&ctx, hailstones[1].start.2),
    );
    let point2 = (
        Int::from_i64(&ctx, hailstones[2].start.0),
        Int::from_i64(&ctx, hailstones[2].start.1),
        Int::from_i64(&ctx, hailstones[2].start.2),
    );

    let velocity0 = (
        Int::from_i64(&ctx, hailstones[0].diff.0),
        Int::from_i64(&ctx, hailstones[0].diff.1),
        Int::from_i64(&ctx, hailstones[0].diff.2),
    );

    let velocity1 = (
        Int::from_i64(&ctx, hailstones[1].diff.0),
        Int::from_i64(&ctx, hailstones[1].diff.1),
        Int::from_i64(&ctx, hailstones[1].diff.2),
    );
    let velocity2 = (
        Int::from_i64(&ctx, hailstones[2].diff.0),
        Int::from_i64(&ctx, hailstones[2].diff.1),
        Int::from_i64(&ctx, hailstones[2].diff.2),
    );

    // Our unknowns:
    let (t0, t1, t2) = (
        Int::new_const(&ctx, "t0"),
        Int::new_const(&ctx, "t1"),
        Int::new_const(&ctx, "t2"),
    );
    let (x, y, z, vx, vy, vz) = (
        Int::new_const(&ctx, "x"),
        Int::new_const(&ctx, "y"),
        Int::new_const(&ctx, "z"),
        Int::new_const(&ctx, "dx"),
        Int::new_const(&ctx, "dy"),
        Int::new_const(&ctx, "dz"),
    );

    // The equations / constraints;
    // Time must be > 0;
    solver.assert(&t0.gt(&Int::from_i64(&ctx, 0)));
    solver.assert(&t1.gt(&Int::from_i64(&ctx, 0)));
    solver.assert(&t2.gt(&Int::from_i64(&ctx, 0)));

    // The 9 equations:
    solver.assert(
        &(&x.clone() + &vx.clone() * &t0.clone())
            ._eq(&(&point0.0.clone() + &velocity0.0.clone() * &t0.clone())),
    );
    solver.assert(
        &(&y.clone() + &vy.clone() * &t0.clone())
            ._eq(&(&point0.1.clone() + &velocity0.1.clone() * &t0.clone())),
    );
    solver.assert(
        &(&z.clone() + &vz.clone() * &t0.clone())
            ._eq(&(&point0.2.clone() + &velocity0.2.clone() * &t0.clone())),
    );

    solver.assert(
        &(&x.clone() + &vx.clone() * &t1.clone())
            ._eq(&(&point1.0.clone() + &velocity1.0.clone() * &t1.clone())),
    );
    solver.assert(
        &(&y.clone() + &vy.clone() * &t1.clone())
            ._eq(&(&point1.1.clone() + &velocity1.1.clone() * &t1.clone())),
    );
    solver.assert(
        &(&z.clone() + &vz.clone() * &t1.clone())
            ._eq(&(&point1.2.clone() + &velocity1.2.clone() * &t1.clone())),
    );

    solver.assert(
        &(&x.clone() + &vx.clone() * &t2.clone())
            ._eq(&(&point2.0.clone() + &velocity2.0.clone() * &t2.clone())),
    );
    solver.assert(
        &(&y.clone() + &vy.clone() * &t2.clone())
            ._eq(&(&point2.1.clone() + &velocity2.1.clone() * &t2.clone())),
    );
    solver.assert(
        &(&z.clone() + &vz.clone() * &t2.clone())
            ._eq(&(&point2.2.clone() + &velocity2.2.clone() * &t2.clone())),
    );

    assert_eq!(solver.check(), SatResult::Sat);
    let m = solver.get_model().unwrap();

    let x_value = m.eval(&x, true).unwrap().as_i64().unwrap();
    let y_value = m.eval(&y, true).unwrap().as_i64().unwrap();
    let z_value = m.eval(&z, true).unwrap().as_i64().unwrap();

    println!("part2: {:?}", x_value + y_value + z_value);
}

#[derive(Debug, Clone, Copy)]
struct Hailstone {
    start: Vector3,
    diff: Vector3,
}

impl Hailstone {
    fn from_input(s: &str) -> Self {
        // 19, 13, 30 @ -2,  1, -2
        let (start_str, diff_str) = s.split_once(" @ ").unwrap();
        let start = start_str
            .splitn(3, ", ")
            .map(|s| s.trim().parse::<i64>().unwrap())
            .collect_tuple()
            .unwrap();
        let diff = diff_str
            .splitn(3, ", ")
            .map(|s| s.trim().parse::<i64>().unwrap())
            .collect_tuple()
            .unwrap();

        Self { start, diff }
    }

    fn intersects_x_y(&self, other: &Hailstone, lower_bound: i64, upper_bound: i64) -> bool {
        // Long winded formula:
        // if a stone has formula xn = cxn + tdxn
        // yn = xyn + tdyn
        // where t is time
        //
        // Then the two paths of h0, h1 interset of there is a t0 and t1 at which x and y values are equal for h0 and h1. (t0 could be equal to t1)
        // i.e. we have the following simultaneous equations:
        // x intersect:
        // cx0 + t0*dx0
        // cx1 + t1*dx1
        // for some values of t1, t0.
        // x intersect: cx0 + t0*dx0 == x1 = xc1 + t1*dx1
        // y intersect: cy0 + t0*dy0 == y1 = yc1 + t1*dy1
        //
        // Rearranging for t0 and t1 to find the times at which the paths can intersect.
        // Two things intersect at t0 after some rearranging at:
        // t0 = (dy1*kx - dx1*ky) / (dx0*dy1 - dx1*dy0)
        //
        // where kx = cx1 - xc0
        // ky = cy1 - cy0
        // In particular if the velocities are parallel:
        // dx1 == dx0, dy1 == dy0. Then the divisor is 0 and no intersections occur.
        // **ALSO assumes velocities are never zero** (as we divide by velocities in the formula rearranging).
        //
        // Similarly for the t1 intersect:
        //
        // t1 = = (- dy0*kx - dx0*ky) / (dx1*dy0 - dx0*dy1)
        //

        // Other == 1, this == 0;
        if other.diff.0 * self.diff.1 - self.diff.0 * other.diff.1 == 0 {
            // Parallel.
            return false;
        }
        let c_x = other.start.0 - self.start.0;
        let c_y = other.start.1 - self.start.1;

        let denominator0 = (other.diff.1 * self.diff.0 - self.diff.1 * other.diff.0) as f64;
        let t_0 = (other.diff.1 * c_x - other.diff.0 * c_y) as f64 / denominator0;

        let (x, y) = (
            self.start.0 as f64 + self.diff.0 as f64 * t_0,
            self.start.1 as f64 + self.diff.1 as f64 * t_0,
        );
        // Debug assert that this is the same as if we took the formula for t1..
        let denominator1 = (other.diff.1 * self.diff.0 - self.diff.1 * other.diff.0) as f64;
        let t_1 = (c_x * self.diff.1 - c_y * self.diff.0) as f64 / denominator1;

        // let (x1, y1) = (
        //     other.start.0 as f64 + other.diff.0 as f64 * t_1,
        //     other.start.1 as f64 + other.diff.1 as f64 * t_1,
        // );
        // let x_str = format!("{:.3}", x);
        // let y_str = format!("{:.3}", y);
        // let x1_str = format!("{:.3}", x1);
        // let y1_str = format!("{:.3}", y1);
        // Round up to 3 decimal strings.. This is necessary due to floating point
        // inprecisions.
        // debug_assert_eq!((x_str, y_str), (x1_str, y1_str));

        if t_0 < 0.0 || t_1 < 0.0 {
            return false;
        }

        if (x >= lower_bound as f64 && x <= upper_bound as f64)
            && (y >= lower_bound as f64 && y <= upper_bound as f64)
        {
            return true;
        }
        false
    }
}

type Vector3 = (i64, i64, i64);
