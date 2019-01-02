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