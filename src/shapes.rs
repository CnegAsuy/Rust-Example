#![allow(non_snake_case)]
use std::f32::consts::PI;

pub trait Shape {
    fn area(&self) -> f32;
    fn circumference(&self) -> f32;
}

pub struct Point {
    x: f32,
    y: f32,
}

impl Point {
    pub fn new(x1: f32, y1:f32) -> Point {
        Point {
            x: x1,
            y: y1,
        }
    }

    pub fn origin() -> Point {
        return Point{
            x: 0.0,
            y: 0.0,
        };
    }
}

pub struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    pub fn new(p1: Point, p2: Point) -> Self {
        Rectangle {p1, p2}
    }
}

impl Shape for Rectangle {

    fn area(&self) -> f32 {
        (self.p1.x - self.p2.x).abs() * (self.p1.y - self.p2.y).abs()
    }

    fn circumference(&self) -> f32 {
        (self.p1.x - self.p2.x).abs()*2.0 + (self.p1.y - self.p2.y).abs()*2.0
    }
}

pub struct Circle {
    centerPoint: Point, 
    radius: f32,
}

impl Circle {
    pub fn new(p: Point, r: f32) -> Self {
        Circle {centerPoint: p, radius: r}
    }
}

impl Shape for Circle {
    fn area(&self) -> f32 {
        (PI*self.radius*self.radius).abs()
    }
    fn circumference(&self) -> f32 {
        (2.0*self.radius*PI).abs()
    }
}

pub fn hello<T>(_x: T) {
    let _c: T = _x; 
}