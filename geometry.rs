use std::ops::{Index};//,Add,Sub,Mul,Div,BitXor};
use std::num;

struct Point3D {
    position: ~[f64] //f64 is rust's double
}

impl Point3D {    
    fn index_mutref<'a>(&'a mut self, index: uint) -> &'a mut f64 {
        return &mut self.position[index];
    }

    fn index(&self, index: uint) -> f64 {
        self.position[index]
    }

    fn scale<'a>(&'a mut self, scale: f64) -> Point3D {
        let new_point: Point3D = 
                                Point3D { position: ~[  self.index(0) * scale,
                                                        self.index(1) * scale,
                                                        self.index(2) * scale]};
        new_point
    }

    fn copy<'a>(&'a mut self) -> ~Point3D {
        let coord1: f64 = self.index(0);
        let coord2: f64 = self.index(1);
        let coord3: f64 = self.index(2);
        ~Point3D { position: ~[  coord1,
                                coord2,
                                coord3]}
    }
    fn dotproduct<'a>(&'a mut self, other: &Point3D) -> f64 {
        self.index(0) * other.index(0) +
        self.index(1) * other.index(1) +
        self.index(2) * other.index(2)
    }

    fn squareNorm<'a>(&'a mut self) -> f64 {
        self.index(0) * self.index(0) + 
        self.index(1) * self.index(1) +
        self.index(2) * self.index(2)
    }
    
    fn length<'a>(&'a mut self) -> f64 {
        num::sqrt(self.squareNorm())
    }
    
    fn unit<'a>(&'a mut self) -> Point3D {
        let length = self.length();
        self.scale(1.0/length)
    }
    
    fn clamp<'a>(&'a mut self) -> Point3D {
        let mut coord1: f64 = 0.0;
        let mut coord2: f64 = 0.0;
        let mut coord3: f64 = 0.0;

        if (self.index(0) > 1.0) { coord1 = 1.0; }
        else if (self.index(0) < 0.0) { coord1 = 0.0;}
        if (self.index(1) > 1.0) { coord2 = 1.0; }
        else if (self.index(1) < 0.0) { coord2 = 0.0;}
        if (self.index(2) > 1.0) { coord3 = 1.0; }
        else if (self.index(2) < 0.0) { coord3 = 0.0;}

        let new_point: Point3D = Point3D { position: ~[ coord1,
                                                        coord2,
                                                        coord3]};
        new_point
    }

    fn add<'a>(&'a mut self, other: &Point3D) -> &'a mut Point3D {
        *self.index_mutref(0) += other.index(0);
        *self.index_mutref(1) += other.index(1);
        *self.index_mutref(2) += other.index(2);
        self
    }
    fn add_copy<'a>(&'a mut self, other: &Point3D) -> Point3D {
        let coord1: f64 = self.index(0) + other.index(0);
        let coord2: f64 = self.index(1) + other.index(1);
        let coord3: f64 = self.index(2) + other.index(2);
        let new_point: Point3D = Point3D { position: ~[ coord1,
                                                        coord2,
                                                        coord3]};
        new_point
    }

    fn sub<'a>(&'a mut self, other: &Point3D) -> &'a mut Point3D {
        *self.index_mutref(0) -= other.index(0);
        *self.index_mutref(1) -= other.index(1);
        *self.index_mutref(2) -= other.index(2);
        self
    }
    fn sub_copy<'a>(&'a mut self, other: &Point3D) -> Point3D {
        let coord1: f64 = self.index(0) - other.index(0);
        let coord2: f64 = self.index(1) - other.index(1);
        let coord3: f64 = self.index(2) - other.index(2);
        let new_point: Point3D = Point3D { position: ~[ coord1,
                                                        coord2,
                                                        coord3]};
        new_point
    }

    fn mul<'a>(&'a mut self, other: &Point3D) -> &'a mut Point3D {
        *self.index_mutref(0) *= other.index(0);
        *self.index_mutref(1) *= other.index(1);
        *self.index_mutref(2) *= other.index(2);
        self
    }
    fn mul_copy<'a>(&'a mut self, other: &Point3D) -> Point3D {
        let coord1: f64 = self.index(0) * other.index(0);
        let coord2: f64 = self.index(1) * other.index(1);
        let coord3: f64 = self.index(2) * other.index(2);
        let new_point: Point3D = Point3D { position: ~[ coord1,
                                                        coord2,
                                                        coord3]};
        new_point
    }

    fn div<'a>(&'a mut self, other: &Point3D) -> &'a mut Point3D {
        *self.index_mutref(0) /= other.index(0);
        *self.index_mutref(1) /= other.index(1);
        *self.index_mutref(2) /= other.index(2);
        self
    }
    fn div_copy<'a>(&'a mut self, other: &Point3D) -> Point3D {
        let coord1: f64 = self.index(0) / other.index(0);
        let coord2: f64 = self.index(1) / other.index(1);
        let coord3: f64 = self.index(2) / other.index(2);
        let new_point: Point3D = Point3D { position: ~[ coord1,
                                                        coord2,
                                                        coord3]};
        new_point
    }

    fn xproduct<'a>(&'a mut self, other: &Point3D) -> &'a mut Point3D {
        let coord1: f64 = self.index(1) * other.index(2) - self.index(2) * other.index(1);
        let coord2: f64 = self.index(0) * other.index(2) + self.index(2) * other.index(0);
        let coord3: f64 = self.index(0) * other.index(1) - self.index(1) * other.index(0);
        *self.index_mutref(0) = coord1;
        *self.index_mutref(1) = coord2;
        *self.index_mutref(2) = coord3;
        self
    }
    fn xproduct_copy<'a>(&'a mut self, other: &Point3D) -> Point3D {
        let coord1: f64 = self.index(1) * other.index(2) - self.index(2) * other.index(1);
        let coord2: f64 = self.index(0) * other.index(2) + self.index(2) * other.index(0);
        let coord3: f64 = self.index(0) * other.index(1) - self.index(1) * other.index(0);
        let new_point: Point3D = Point3D { position: ~[ coord1,
                                                        coord2,
                                                        coord3]};
        new_point
    }
}

struct Ray3D {
    position: ~Point3D,
    direction: ~Point3D
}

impl Ray3D {
    fn operator<'a>(&'a mut self, scale: f64) -> Point3D{
        let temp = &self.direction.scale(scale);
        self.position.add_copy(temp)
    }

    fn add<'a>(&'a mut self, other: &Ray3D) -> &'a mut Ray3D {
        self.position.add(other.position);
        self
    }
    fn add_copy<'a>(&'a mut self, other: &Ray3D) -> Ray3D {
        let mut new_ray = Ray3D{position: self.position.copy(), 
                                direction: self.direction.copy()};
        new_ray.position.add(other.position);
        new_ray
    }

    fn subtract<'a>(&'a mut self, other: &Ray3D) -> &'a mut Ray3D {
        self.position.sub(other.position);
        self
    }
    fn subtract_copy<'a>(&'a mut self, other: &Ray3D) -> Ray3D {
        let mut new_ray = Ray3D{position: self.position.copy(), 
                                direction: self.direction.copy()};
        new_ray.position.sub(other.position);
        new_ray
    }
}

fn main(){

}

