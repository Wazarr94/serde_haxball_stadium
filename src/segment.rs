use std::{
    collections::HashMap,
    f64::consts::PI,
    ops::{Deref, DerefMut},
};

use bevy::{math::DVec2, prelude::Color};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    hx_trait::{Trait, Traitable},
    utils::{parse_collision, parse_color, CollisionFlag},
    vertex::Vertex,
};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SegmentRaw {
    v0: usize,
    v1: usize,
    b_coef: Option<f64>,
    curve: Option<f64>,
    curve_f: Option<f64>,
    bias: Option<f64>,
    c_group: Option<Vec<String>>,
    c_mask: Option<Vec<String>>,
    vis: Option<bool>,
    color: Option<Value>,
    #[serde(rename = "trait")]
    hx_trait: Option<String>,
}

impl Default for SegmentRaw {
    fn default() -> Self {
        SegmentRaw {
            v0: 0,
            v1: 0,
            b_coef: Some(1.0),
            curve: Some(0.0),
            curve_f: Some(0.0),
            bias: Some(0.0),
            c_group: Some(vec!["wall".to_string()]),
            c_mask: Some(vec!["all".to_string()]),
            vis: Some(true),
            color: Some(Value::String("0".to_string())),
            hx_trait: None,
        }
    }
}

impl Traitable for SegmentRaw {
    fn apply_trait(&self, traits: &HashMap<String, Trait>) -> SegmentRaw {
        let tr_def = Trait::default();
        let tr_d = match &self.hx_trait {
            Some(tr_name) => traits.get(tr_name).unwrap(),
            None => &tr_def,
        };
        let b_coef = self.b_coef.or(tr_d.b_coef);
        let curve = self.curve.or(tr_d.curve);
        let curve_f = self.curve_f.or(tr_d.curve_f);
        let bias = self.bias.or(tr_d.bias);
        let c_group = self.c_group.as_ref().or(tr_d.c_group.as_ref()).cloned();
        let c_mask = self.c_mask.as_ref().or(tr_d.c_mask.as_ref()).cloned();
        let vis = self.vis.or(tr_d.vis);
        let color = self.color.as_ref().or(tr_d.color.as_ref()).cloned();
        let hx_trait = self.hx_trait.clone();
        SegmentRaw {
            b_coef,
            curve,
            curve_f,
            bias,
            c_group,
            c_mask,
            vis,
            color,
            hx_trait,
            ..*self
        }
    }
}

impl SegmentRaw {
    pub fn apply_default(&self) -> SegmentRaw {
        let s_def = SegmentRaw::default();
        SegmentRaw {
            b_coef: self.b_coef.or(s_def.b_coef),
            curve: self.curve.or(s_def.curve),
            curve_f: self.curve_f.or(s_def.curve_f),
            bias: self.bias.or(s_def.bias),
            c_group: self.c_group.as_ref().or(s_def.c_group.as_ref()).cloned(),
            c_mask: self.c_mask.as_ref().or(s_def.c_mask.as_ref()).cloned(),
            vis: self.vis.or(s_def.vis),
            color: self.color.as_ref().or(s_def.color.as_ref()).cloned(),
            hx_trait: self.hx_trait.clone(),
            ..*self
        }
    }

    fn to_straight(&self, traits: &HashMap<String, Trait>) -> StraightSegment {
        let segment_raw = self.apply_trait(traits).apply_default();
        let vertex_indices = (segment_raw.v0, segment_raw.v1);
        let b_coef = segment_raw.b_coef.unwrap();
        let bias = segment_raw.bias.unwrap();
        let c_group = parse_collision(&segment_raw.c_group.unwrap());
        let c_mask = parse_collision(&segment_raw.c_mask.unwrap());
        let vis = segment_raw.vis.unwrap();
        let color = parse_color(&segment_raw.color.unwrap(), false);
        StraightSegment {
            vertex_indices,
            b_coef,
            bias,
            c_group,
            c_mask,
            vis,
            color,
        }
    }

    fn to_curved(&self, traits: &HashMap<String, Trait>) -> CurvedSegment {
        CurvedSegment::new(self, traits)
    }

    pub fn to_segment(&self, traits: &HashMap<String, Trait>) -> Segment {
        match self.curve_f {
            Some(curve_f) if curve_f != 0.0 => Segment::Curved(self.to_curved(traits)),
            _ => match self.curve {
                Some(curve) if curve != 0.0 => Segment::Curved(self.to_curved(traits)),
                _ => Segment::Straight(self.to_straight(traits)),
            },
        }
    }
}

#[derive(Debug)]
pub struct StraightSegment {
    pub vertex_indices: (usize, usize),
    pub b_coef: f64,
    pub bias: f64,
    pub c_group: CollisionFlag,
    pub c_mask: CollisionFlag,
    pub vis: bool,
    pub color: Color,
}

#[derive(Debug)]
pub struct CurvedSegment {
    pub base: StraightSegment,
    curve: f64,
}

impl Deref for CurvedSegment {
    type Target = StraightSegment;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl DerefMut for CurvedSegment {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl CurvedSegment {
    pub fn new(raw_segment: &SegmentRaw, traits: &HashMap<String, Trait>) -> CurvedSegment {
        let base = raw_segment.to_straight(traits);
        let mut curved_segment = CurvedSegment { base, curve: 0.0 };

        let curve = raw_segment.curve.unwrap_or(0.0);
        let curve_f = raw_segment.curve_f.unwrap_or(0.0);
        let curve_final = curved_segment.get_curve(curve, curve_f);
        curved_segment.curve = curve_final;

        curved_segment
    }

    fn get_curve(&mut self, curve: f64, curve_f: f64) -> f64 {
        if curve_f != 0.0 {
            return curve_f;
        }
        let mut curve_value = curve;
        if curve_value < 0.0 {
            curve_value *= -1.0;
            self.bias *= -1.0;
            let tmp = self.vertex_indices.0;
            self.vertex_indices.0 = self.vertex_indices.1;
            self.vertex_indices.1 = tmp;
        }
        curve_value *= PI / 180.0;
        let lim_inf = 10.0 * PI / 180.0;
        let lim_sup = 340.0 * PI / 180.0;
        if curve_value > lim_inf && curve_value < lim_sup {
            curve_value = 1.0 / (curve_value / 2.0).tan();
        }
        return curve_value;
    }

    pub fn circle_center(&self, vertexes: &[Vertex]) -> DVec2 {
        let pos_0 = vertexes[self.vertex_indices.0].position;
        let pos_1 = vertexes[self.vertex_indices.1].position;
        let vec_center = (pos_1 - pos_0) / 2.0;
        let circle_center_x = pos_0.x + vec_center.x - vec_center.y * self.curve;
        let circle_center_y = pos_0.y + vec_center.y + vec_center.x * self.curve;
        DVec2::new(circle_center_x, circle_center_y)
    }

    pub fn circle_radius(&self, vertexes: &[Vertex]) -> f64 {
        let pos_0 = vertexes[self.vertex_indices.0].position;
        let center = self.circle_center(vertexes);
        let radius = (pos_0 - center).length();
        radius
    }

    pub fn circle_tangeants(&self, vertexes: &[Vertex]) -> (DVec2, DVec2) {
        let pos_0 = vertexes[self.vertex_indices.0].position;
        let pos_1 = vertexes[self.vertex_indices.1].position;
        let center = self.circle_center(vertexes);
        (pos_0 - center, pos_1 - center)
    }

    pub fn circle_angles(&self, vertexes: &[Vertex]) -> (f64, f64) {
        let tangeants = self.circle_tangeants(vertexes);
        let circle_center = self.circle_center(vertexes);
        let angle_0 = tangeants.0.angle_between(circle_center);
        let mut angle_1 = tangeants.1.angle_between(circle_center);
        while angle_1 < angle_0 {
            angle_1 += 2.0 * PI;
        }
        (angle_0, angle_1)
    }
}

#[derive(Debug)]
pub enum Segment {
    Straight(StraightSegment),
    Curved(CurvedSegment),
}
