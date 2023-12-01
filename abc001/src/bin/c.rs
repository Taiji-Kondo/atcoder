use proconio::input;

fn main() {
    input! {
        deg: f32,
        dis: f32,
    }

    let dis = ((dis / 60.0) * 10.0).round() / 10.0;
    let w = match dis {
        _ if dis >= 32.7 => 12,
        _ if dis >= 28.5 && dis <= 32.6 => 11,
        _ if dis >= 24.5 && dis <= 28.4 => 10,
        _ if dis >= 20.8 && dis <= 24.4 => 9,
        _ if dis >= 17.2 && dis <= 20.7 => 8,
        _ if dis >= 13.9 && dis <= 17.1 => 7,
        _ if dis >= 10.8 && dis <= 13.8 => 6,
        _ if dis >= 8.0 && dis <= 10.7 => 5,
        _ if dis >= 5.5 && dis <= 7.9 => 4,
        _ if dis >= 3.4 && dis <= 5.4 => 3,
        _ if dis >= 1.6 && dis <= 3.3 => 2,
        _ if dis >= 0.3 && dis <= 1.5 => 1,
        _ => 0,
    };

    let mut dir  = match deg {
        _ if deg >= 326.25 * 10.0 && deg < 348.75 * 10.0 => "NNW",
        _ if deg >= 303.75 * 10.0 && deg < 326.25 * 10.0 => "NW",
        _ if deg >= 281.25 * 10.0 && deg < 303.75 * 10.0 => "WNW",
        _ if deg >= 258.75 * 10.0 && deg < 281.25 * 10.0 => "W",
        _ if deg >= 236.25 * 10.0 && deg < 258.75 * 10.0 => "WSW",
        _ if deg >= 213.75 * 10.0 && deg < 236.25 * 10.0 => "SW",
        _ if deg >= 191.25 * 10.0 && deg < 213.75 * 10.0 => "SSW",
        _ if deg >= 168.75 * 10.0 && deg < 191.25 * 10.0 => "S",
        _ if deg >= 146.25 * 10.0 && deg < 168.75 * 10.0 => "SSE",
        _ if deg >= 123.75 * 10.0 && deg < 146.25 * 10.0 => "SE",
        _ if deg >= 101.25 * 10.0 && deg < 123.75 * 10.0 => "ESE",
        _ if deg >= 78.75 * 10.0 && deg <  101.2 * 10.05=> "E",
        _ if deg >= 56.25 * 10.0 && deg < 78.75 * 10.0 => "ENE",
        _ if deg >= 33.75 * 10.0 && deg < 56.25 * 10.0 => "NE",
        _ if deg >= 11.25 * 10.0 && deg < 33.75 * 10.0 => "NNE",
        _  => "N",
    };
    if w == 0 {
        dir = "C";
    }

    println!("{} {}", dir, w);
}
