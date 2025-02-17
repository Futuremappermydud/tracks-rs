pub const fn ease_linear(p: f32) -> f32 {
    p
}

pub fn ease_step(p: f32) -> f32 {
    p.floor()
}

pub const fn ease_out_quad(p: f32) -> f32 {
    -(p * (p - 2.0))
}

pub const fn ease_in_quad(p: f32) -> f32 {
    p * p
}

pub const fn ease_in_out_quad(p: f32) -> f32 {
    if p < 0.5 {
        2.0 * p * p
    } else {
        (-2.0 * p * p) + (4.0 * p) - 1.0
    }
}

pub const fn ease_in_cubic(p: f32) -> f32 {
    p * p * p
}

pub const fn ease_out_cubic(p: f32) -> f32 {
    let f = p - 1.0;
    (f * f * f) + 1.0
}

pub const fn ease_in_out_cubic(p: f32) -> f32 {
    if p < 0.5 {
        4.0 * p * p * p
    } else {
        let f = (2.0 * p) - 2.0;
        (0.5 * f * f * f) + 1.0
    }
}

pub const fn ease_in_quart(p: f32) -> f32 {
    p * p * p * p
}

pub const fn ease_out_quart(p: f32) -> f32 {
    let f = p - 1.0;
    (f * f * f * (1.0 - p)) + 1.0
}

pub const fn ease_in_out_quart(p: f32) -> f32 {
    if p < 0.5 {
        8.0 * p * p * p * p
    } else {
        let f = p - 1.0;
        (-8.0 * f * f * f * f) + 1.0
    }
}

pub const fn ease_in_quint(p: f32) -> f32 {
    p * p * p * p * p
}

pub const fn ease_out_quint(p: f32) -> f32 {
    let f = p - 1.0;
    (f * f * f * f * f) + 1.0
}

pub const fn ease_in_out_quint(p: f32) -> f32 {
    if p < 0.5 {
        16.0 * p * p * p * p * p
    } else {
        let f = (2.0 * p) - 2.0;
        (0.5 * f * f * f * f * f) + 1.0
    }
}

pub fn ease_in_sine(p: f32) -> f32 {
    ((p - 1.0) * std::f32::consts::FRAC_PI_2).sin() + 1.0
}

pub fn ease_out_sine(p: f32) -> f32 {
    (p * std::f32::consts::FRAC_PI_2).sin()
}

pub fn ease_in_out_sine(p: f32) -> f32 {
    0.5 * (1.0 - (p * std::f32::consts::PI).cos())
}

pub fn ease_in_circ(p: f32) -> f32 {
    1.0 - (1.0 - (p * p)).sqrt()
}

pub fn ease_out_circ(p: f32) -> f32 {
    ((2.0 - p) * p).sqrt()
}

pub fn ease_in_out_circ(p: f32) -> f32 {
    if p < 0.5 {
        0.5 * (1.0 - (4.0 * p * p).sqrt())
    } else {
        0.5 * ((-((2.0 * p) - 3.0) * ((2.0 * p) - 1.0)).sqrt() + 1.0)
    }
}

pub fn ease_in_expo(p: f32) -> f32 {
    if p == 0.0 {
        p
    } else {
        2.0f32.powf(10.0 * (p - 1.0))
    }
}

pub fn ease_out_expo(p: f32) -> f32 {
    if p == 1.0 {
        p
    } else {
        1.0 - 2.0f32.powf(-10.0 * p)
    }
}

pub fn ease_in_out_expo(p: f32) -> f32 {
    if p == 0.0 || p == 1.0 {
        p
    } else if p < 0.5 {
        0.5 * 2.0f32.powf((20.0 * p) - 10.0)
    } else {
        (-0.5 * 2.0f32.powf((-20.0 * p) + 10.0)) + 1.0
    }
}

pub fn ease_in_elastic(p: f32) -> f32 {
    (13.0 * std::f32::consts::FRAC_PI_2 * p).sin() * 2.0f32.powf(10.0 * (p - 1.0))
}

pub fn ease_out_elastic(p: f32) -> f32 {
    ((-13.0 * std::f32::consts::FRAC_PI_2 * (p + 1.0)).sin() * 2.0f32.powf(-10.0 * p)) + 1.0
}

pub fn ease_in_out_elastic(p: f32) -> f32 {
    if p < 0.5 {
        0.5 * (13.0 * std::f32::consts::FRAC_PI_2 * (2.0 * p)).sin()
            * 2.0f32.powf(10.0 * ((2.0 * p) - 1.0))
    } else {
        0.5 * (((-13.0 * std::f32::consts::FRAC_PI_2 * (2.0 * p)).sin()
            * 2.0f32.powf(-10.0 * ((2.0 * p) - 1.0)))
            + 2.0)
    }
}

pub fn ease_in_back(p: f32) -> f32 {
    (p * p * p) - (p * (p * std::f32::consts::PI).sin())
}

pub fn ease_out_back(p: f32) -> f32 {
    let f = 1.0 - p;
    1.0 - ((f * f * f) - (f * (f * std::f32::consts::PI).sin()))
}

pub fn ease_in_out_back(p: f32) -> f32 {
    if p < 0.5 {
        let f = 2.0 * p;
        0.5 * ((f * f * f) - (f * (f * std::f32::consts::PI).sin()))
    } else {
        let f = 1.0 - ((2.0 * p) - 1.0);
        (0.5 * (1.0 - ((f * f * f) - (f * (f * std::f32::consts::PI).sin())))) + 0.5
    }
}

pub const fn ease_out_bounce(p: f32) -> f32 {
    if p < 4.0 / 11.0 {
        (121.0 * p * p) / 16.0
    } else if p < 8.0 / 11.0 {
        (363.0 / 40.0 * p * p) - (99.0 / 10.0 * p) + (17.0 / 5.0)
    } else if p < 9.0 / 10.0 {
        (4356.0 / 361.0 * p * p) - (35442.0 / 1805.0 * p) + (16061.0 / 1805.0)
    } else {
        (54.0 / 5.0 * p * p) - (513.0 / 25.0 * p) + (268.0 / 25.0)
    }
}

pub const fn ease_in_bounce(p: f32) -> f32 {
    1.0 - ease_out_bounce(1.0 - p)
}

pub const fn ease_in_out_bounce(p: f32) -> f32 {
    if p < 0.5 {
        0.5 * ease_in_bounce(2.0 * p)
    } else {
        (0.5 * ease_out_bounce((2.0 * p) - 1.0)) + 0.5
    }
}
