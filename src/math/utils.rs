use crate::math::{
    Vector3f,
    Vector2f,
};

pub struct BarycentricCoords;

impl BarycentricCoords {
    /**
     * Takes a point in the plane of the triangle (therefore just 2 dim vector) and returns the barycentric coords of that point
     */
    pub fn to_barycentric_coords(point: &Vector2f, a: &Vector3f, b: &Vector3f, c: &Vector3f) -> Vector3f {
        let det_t = (a.x - c.x)*(b.y - c.y) + (c.x - b.x)*(a.y - c.y);
        let l1 = ((point.x - c.x)*(b.y - c.y) + (c.x - b.x)*(point.y - c.y)) / det_t;
        let l2 = ((point.x - c.x)*(c.y - a.y) + (a.x - c.x)*(point.y - c.y)) / det_t;
        let l3 = 1.0 - l1 - l2;
        Vector3f::new(l1, l2, l3)
    }

    pub fn from_barycentric_coords(bary_coords: &Vector3f, a: &Vector3f, b: &Vector3f, c: &Vector3f) -> Vector3f {
        let x = bary_coords.x * a.x + bary_coords.y * b.x + bary_coords.z * c.x;
        let y = bary_coords.x * a.y + bary_coords.y * b.y + bary_coords.z * c.y;
        let z = bary_coords.x * a.z + bary_coords.y * b.z + bary_coords.z * c.z;
        Vector3f::new(x, y, z)
    }
}

pub fn gram_schmidt_orthogonalize(v1: &Vector3f, v2: Vector3f, v3: Vector3f) -> (Vector3f, Vector3f) {
    let proj_u1_v2 = v1.onto_project(&v2);
    let u2 = v2 - proj_u1_v2;
    let proj_u1_v3 = v1.onto_project(&v3);
    let proj_u2_v3 = u2.onto_project(&v3);
    let u3 = v3 - proj_u1_v3 - proj_u2_v3;
    (u2, u3)
}

pub fn distance(p1: &Vector3f, p2: &Vector3f) -> f32 {
    let sq_sum = (p2.x - p1.x)*(p2.x - p1.x) + (p2.y - p1.y)*(p2.y - p1.y) + (p2.z - p1.z)*(p2.z - p1.z);
    sq_sum.sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn orthogonalize_test_1() {
        let v1 = Vector3f::new(3.0, 2.1, -4.0);
        let v2 = Vector3f::new(0.5, -1.1, -2.0);
        let v3 = Vector3f::new(5.0, 5.0, 2.0);
        let (u2, u3) = gram_schmidt_orthogonalize(&v1, v2, v3);     
        
        assert!(v1.dot_product(&u2).abs() < 1e-3, format!("Was {}", v1.dot_product(&u2)));
        assert!(v1.dot_product(&u3).abs() < 1e-3, format!("Was {}", v1.dot_product(&u3)));
        assert!(u2.dot_product(&u3).abs() < 1e-3, format!("Was {}", u2.dot_product(&u3)));
    }
}