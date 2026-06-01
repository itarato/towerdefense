use bevy::math::Vec2;

fn slope(lhs: Vec2, rhs: Vec2) -> f32 {
    (lhs.x - rhs.x) / (lhs.y - rhs.y)
}

fn between(p: f32, lhs: f32, rhs: f32) -> bool {
    let min = lhs.min(rhs);
    let max = lhs.max(rhs);
    min <= p && p <= max
}

fn point_on_path(p: Vec2, path: (Vec2, Vec2)) -> bool {
    assert_ne!(path.0, path.1);

    if p.abs_diff_eq(path.0, 0.1) || p.abs_diff_eq(path.1, 0.1) {
        return true;
    }

    let main_slope = slope(path.0, path.1);
    let sub_slope = slope(path.0, p);

    between(p.x, path.0.x, path.1.x) && between(p.y, path.0.y, path.1.y)
}

pub(crate) fn calculate_next_position_on_path(
    path: &Vec<Vec2>,
    x: f32,
    y: f32,
    time_delta_secs: f32,
    speed: f32,
) -> Vec2 {
    for i in 0..path.len() - 1 {
        // Going reverse so later path is recognized first in case of shared points.
        let rev_i = path.len() - 2 - i;
        if point_on_path((x, y).into(), (path[rev_i], path[rev_i + 1])) {
            todo!()

            break;
        }
    }

    unimplemented!()
}

#[cfg(test)]
mod test {
    use crate::math::point_on_path;

    #[test]
    fn test_point_on_path() {
        let path = ((1.0, -8.0).into(), (-2.0, -2.0).into());

        assert!(point_on_path((1.0, -8.0).into(), path));
        assert!(point_on_path((-2.0, -2.0).into(), path));

        assert!(point_on_path((-1.0, -4.0).into(), path));
        assert!(point_on_path((-1.5, -3.0).into(), path));

        assert!(!point_on_path((-3.0, -0.0).into(), path));
        assert!(!point_on_path((1.0, 4.0).into(), path));
    }
}
