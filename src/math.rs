use bevy::math::Vec2;

const FLOAT_PRECISION: f32 = 1.0;

fn slope(lhs: &Vec2, rhs: &Vec2) -> f32 {
    (lhs.x - rhs.x) / (lhs.y - rhs.y)
}

fn between(p: f32, lhs: f32, rhs: f32) -> bool {
    let min = lhs.min(rhs);
    let max = lhs.max(rhs);
    min - FLOAT_PRECISION <= p && p <= max + FLOAT_PRECISION
}

fn point_on_path(p: &Vec2, path: (&Vec2, &Vec2)) -> bool {
    assert_ne!(path.0, path.1);

    if p.abs_diff_eq(*path.0, 0.1) || p.abs_diff_eq(*path.1, FLOAT_PRECISION) {
        return true;
    }

    let main_slope = slope(path.0, path.1);
    let sub_slope = slope(path.0, p);
    if (main_slope - sub_slope).abs() > FLOAT_PRECISION {
        return false;
    }

    between(p.x, path.0.x, path.1.x) && between(p.y, path.0.y, path.1.y)
}

pub(crate) fn calculate_next_position_on_path(
    point: &Vec2,
    path: &Vec<Vec2>,
    distance: f32,
) -> Vec2 {
    for i in 0..path.len() - 1 {
        // Going reverse so later path is recognized first in case of shared points.
        let rev_i = path.len() - 2 - i;
        let path_start = &path[rev_i];
        let path_end = &path[rev_i + 1];

        if point_on_path(point, (path_start, path_end)) {
            let dist_to_end = point.distance(*path_end);

            if dist_to_end < distance {
                // path on the next segment
                let distance_left = distance - dist_to_end;
                if rev_i >= path.len() - 2 {
                    // No more paths left.
                    return *path_end;
                } else {
                    // Go to the next path.
                    let path_next_end = &path[rev_i + 2];
                    return path_end.move_towards(*path_next_end, distance_left);
                }
            } else {
                return point.move_towards(*path_end, distance);
            }
            // Unreachable.
        }
    }

    panic!(
        "Could not calculate next position on path. Current: {:?} Path: {:?} Distance: {:?}",
        point, path, distance
    );
}

pub(crate) fn path_completed(point: &Vec2, path: &Vec<Vec2>) -> bool {
    path.last().unwrap().abs_diff_eq(*point, 0.1)
}

#[cfg(test)]
mod test {
    use crate::math::point_on_path;

    #[test]
    fn test_point_on_path() {
        let path = (&(1.0, -8.0).into(), &(-2.0, -2.0).into());

        assert!(point_on_path(&(1.0, -8.0).into(), path));
        assert!(point_on_path(&(-2.0, -2.0).into(), path));

        assert!(point_on_path(&(-1.0, -4.0).into(), path));
        assert!(point_on_path(&(-1.5, -3.0).into(), path));

        assert!(!point_on_path(&(-3.0, -0.0).into(), path));
        assert!(!point_on_path(&(1.0, 4.0).into(), path));
    }
}
