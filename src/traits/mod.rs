mod area;
mod vehicle;
pub fn render_area_traits() {
    println!(
        "circle - area {:?}",
        area::Calculate::round_area(area::Circle {
            radius: 10.0,
        })
    );
    println!(
        "circle - circumference {:?}",
        area::Calculate::round_circumference(area::Circle {
            radius: 15.0,
        })
    );
    println!(
        "triangle - area {:?}",
        area::Calculate::area(area::Triangle {
            base: 20,
            height: 10,
        })
    );

    println!(
        "triangle - area {:?}",
        area::Calculate::perimeter(area::Triangle {
            base: 20,
            height: 10,
        })
    )
}
