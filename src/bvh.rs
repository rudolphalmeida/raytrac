use aabb::AABB;
use objects::HitRecord;
use objects::Hittable;
use ray::Ray;

use rand::prelude::*;

use std::cmp::Ordering;

pub struct BvhTree<'a> {
    nodes: Vec<BvhNode<'a>>,
    root: NodeId,
}

struct BvhNode<'a> {
    left: Option<NodeId>,
    right: Option<NodeId>,
    aabb: Option<AABB>,
    hittable: Option<&'a Box<Hittable>>,
}

#[derive(Copy, Clone, Debug)]
pub struct NodeId {
    index: usize,
}

impl<'a> BvhTree<'a> {
    fn hit(&self, id: NodeId, r: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord> {
        let node = &self.nodes[id.index];

        if node.aabb.is_none() || node.aabb.is_some() && node.aabb.unwrap().hit(r, tmin, tmax) {
            match node.hittable {
                Some(ref hitable) => return hitable.hits(r, tmin, tmax),
                None => {}
            }

            let mut hit_left: Option<HitRecord> = None;
            let mut hit_right: Option<HitRecord> = None;

            if let Some(ref left_index) = node.left {
                hit_left = self.hit(*left_index, r, tmin, tmax);
            }

            if let Some(ref right_index) = node.right {
                hit_right = self.hit(*right_index, r, tmin, tmax);
            }

            match hit_left {
                Some(left) => match hit_right {
                    Some(right) => if left.t < right.t {
                        return hit_left;
                    } else {
                        return hit_right;
                    },
                    None => return hit_left,
                },
                None => {}
            }

            match hit_right {
                Some(_right) => return hit_right,
                None => {}
            }
        }

        None
    }
}

impl<'a> Hittable for BvhTree<'a> {
    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<AABB> {
        self.nodes[self.root.index].aabb
    }

    fn hits(&self, r: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord> {
        self.hit(self.root, r, tmin, tmax)
    }
}

impl<'a> BvhTree<'a> {
    pub fn new(l: &'a mut [Box<Hittable>], time0: f64, time1: f64) -> BvhTree<'a> {
        let mut tree = BvhTree {
            nodes: Vec::new(),
            root: NodeId { index: 0 },
        };
        tree.root = tree.build(l, time0, time1);

        tree
    }

    fn build(&mut self, l: &'a mut [Box<Hittable>], time0: f64, time1: f64) -> NodeId {
        let axis = thread_rng().gen_range::<u32>(0, 3);

        match axis {
            0 => l.sort_by(|a, b| box_x_compare(a, b, time0, time1)),
            1 => l.sort_by(|a, b| box_y_compare(a, b, time0, time1)),
            2 => l.sort_by(|a, b| box_z_compare(a, b, time0, time1)),
            _ => panic!("Wait what? How did this happen... "),
        }

        let left: NodeId;
        let right: NodeId;

        if l.len() == 1 {
            return self.new_leaf(&l[0], time0, time1);
        } else if l.len() == 2 {
            left = self.new_leaf(&l[0], time0, time1);
            right = self.new_leaf(&l[1], time0, time1);
        } else {
            let half_len = l.len() / 2;
            let (left_hitables, right_hitables) = l.split_at_mut(half_len);

            left = self.build(left_hitables, time0, time1);
            right = self.build(right_hitables, time0, time1);
        }

        if let Some(left_box) = self.nodes[left.index].aabb {
            if let Some(right_box) = self.nodes[right.index].aabb {
                return self.new_node(
                    left_box.surrounding_box(&right_box),
                    Some(left),
                    Some(right),
                );
            }
        }

        panic!("No bounding box in BvhNode::build");
    }

    fn new_leaf(&mut self, hitable: &'a Box<Hittable>, time0: f64, time1: f64) -> NodeId {
        let index = self.nodes.len();

        self.nodes.push(BvhNode {
            left: None,
            right: None,
            aabb: hitable.bounding_box(time0, time1),
            hittable: Some(hitable),
        });

        NodeId { index }
    }

    fn new_node(&mut self, aabb: AABB, left: Option<NodeId>, right: Option<NodeId>) -> NodeId {
        let index = self.nodes.len();

        self.nodes.push(BvhNode {
            left,
            right,
            aabb: Some(aabb),
            hittable: None,
        });

        NodeId { index }
    }

    fn number_hittables(&self, id: NodeId) -> usize {
        let node = &self.nodes[id.index];
        let local_hitable = if node.hittable.is_some() { 1 } else { 0 };
        let count_left = if let Some(left_index) = node.left {
            self.number_hittables(left_index)
        } else {
            0
        };
        let count_right = if let Some(right_index) = node.right {
            self.number_hittables(right_index)
        } else {
            0
        };

        local_hitable + count_left + count_right
    }
}

fn box_x_compare(a: &Box<Hittable>, b: &Box<Hittable>, time0: f64, time1: f64) -> Ordering {
    if let Some(box_left) = a.bounding_box(time0, time1) {
        if let Some(box_right) = b.bounding_box(time0, time1) {
            if let Some(cmp) = box_left.min.x.partial_cmp(&box_right.min.x) {
                return cmp;
            } else {
                panic!("Can't compare");
            }
        }
    }

    panic!("No bounding box in BvhNode::new");
}

fn box_y_compare(a: &Box<Hittable>, b: &Box<Hittable>, time0: f64, time1: f64) -> Ordering {
    if let Some(box_left) = a.bounding_box(time0, time1) {
        if let Some(box_right) = b.bounding_box(time0, time1) {
            if let Some(cmp) = box_left.min.y.partial_cmp(&box_right.min.y) {
                return cmp;
            } else {
                panic!("Can't compare");
            }
        }
    }

    panic!("No bounding box in BvhNode::new");
}

fn box_z_compare(a: &Box<Hittable>, b: &Box<Hittable>, time0: f64, time1: f64) -> Ordering {
    if let Some(box_left) = a.bounding_box(time0, time1) {
        if let Some(box_right) = b.bounding_box(time0, time1) {
            if let Some(cmp) = box_left.min.z.partial_cmp(&box_right.min.z) {
                return cmp;
            } else {
                panic!("Can't compare");
            }
        }
    }

    panic!("No bounding box in BvhNode::new");
}
