use std::ops::{Index};//,Add,Sub,Mul,Div,BitXor};
use std::num;


////////////////////////////////
////        Point3D         ////
////////////////////////////////
pub struct Point3D {
    position:       ~[f64] //f64 is rust's double
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
        ~Point3D { position: ~[ coord1,
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

////////////////////////////////
////         Ray3D          ////
////////////////////////////////
pub struct Ray3D {
    position:       ~Point3D,
    direction:      ~Point3D
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

////////////////////////////////
////        Plane3D         ////
////////////////////////////////
pub struct Plane3D {
    normal:         ~Point3D,
    distance:       f64
}

impl Plane3D {
    //double evaluate(const Point3D& p) const;
    //double operator()(const Point3D& p) const;
    //void makePositive(const Point3D& p);
}

////////////////////////////////
////   RayIntersectionInfo  ////
////////////////////////////////
struct RayIntersectionInfo {
    /*The material of the intersected surface*/
    //material: RayMaterial,

    /*Position, in world coordinates, of the interstion*/
    iCoordinate: Point3D,

    /*The normal of the shape at the point of the intersection*/
    normal: Point3D

    /*The texture coordinates of the shape at the point of intersection*/
    //texCoordinates: Point2D
}

////////////////////////////////
////       RayCamera        ////
////////////////////////////////
struct RayCamera {
    color:          ~Point3D,
    heightAngle:    f64,
    aspectRatio:    f64,
    position:       ~Point3D,
    direction:      ~Point3D,
    up:             ~Point3D,
    right:          ~Point3D
}

impl RayCamera {
    //int read(FILE* fp);
    //void write(FILE* fp=stdout);
    //void drawOpenGL(void);
    //Void rotateUp(Point3D center,float angle);
    //void rotateRight(Point3D center, float angle);
    //void moveForward(float dist);
    //void moveRight(float dist);
    //void moveUp(float dist);
}

////////////////////////////////
////  RayDirectionalLight   ////
////////////////////////////////
struct RayDirectionalLight {
    direction:      ~Point3D
}

impl RayDirectionalLight {
    //int read(FILE* fp);
    //void write(FILE* fp=stdout);
    //Point3D getDiffuse(Point3D cameraPosition, struct RayIntersectionInfo& iInfo)
    //Point3D getSpecular(Point3D cameraPosition, struct RayIntersectionInfo& iInfo)
    //int isInShadow(struct RayIntersectionInfo& iInfo, class RayShape* shape, int& isectCount)
    //Point3D transparency(struct RayIntersectionInfo& iInfo, class RayShape* shape, Point3D cLimit)
}

////////////////////////////////
////        RaySphere       ////
////////////////////////////////
struct RaySphere {
    center:         ~Point3D,
    radius:         f64
}

impl RaySphere {
    //int read(FILE* fp, int* materialIndex, RayVertex* vList, int vSize);
    // fn read
    //void write(int indent, FILE* fp=stdout);

    //double intersect(Ray3D ray, struct RayIntersectionInfo& iInfo, double mx=-1);
    fn intersect(&mut self, ray: Ray3D, iInfo: &RayIntersectionInfo, mx: f64) -> f64{
        let length  = (self.center.sub_copy(ray.position)).length();
        let mut tempRay = ray.operator(length).copy().clone();
        let check   = (self.center.sub_copy(tempRay)).length();
        0.0
    }
}

////////////////////////////////
////      RayTriangle       ////
////////////////////////////////
struct RayTriangle {
    v1:             ~Point3D,
    v2:             ~Point3D,
    plane:          ~Plane3D
}

impl RayTriangle {
    //int read(FILE* fp, int* materialIndex, RayVertex* vList, int vSize);
    //void write(int indent, FILE* fp=stdout);
    //double intersect(Ray3D ray, struct RayIntersectionInfo& iInfo, double mx=-1);
}

fn main(){

}





