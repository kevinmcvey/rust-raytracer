extern mod extra;


use std::ops::{Index};//,Add,Sub,Mul,Div,BitXor};
use std::num;

use std::io;
use std::io::*;
use std::io::file_reader;
use std::io::file_writer;
use std::path;

use extra::arc;


////////////////////////////////
////        Point3D         ////
////////////////////////////////
#[deriving(Clone)]
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

    fn scale<'a>(&'a self, scale: f64) -> Point3D {
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

    fn dotproduct<'a>(&'a self, other: &Point3D) -> f64 {
        self.index(0) * other.index(0) +
        self.index(1) * other.index(1) +
        self.index(2) * other.index(2)
    }

    fn squareNorm<'a>(&'a self) -> f64 {
        self.index(0) * self.index(0) + 
        self.index(1) * self.index(1) +
        self.index(2) * self.index(2)
    }

    fn length<'a>(&'a self) -> f64 {
        num::sqrt(self.squareNorm())
    }

    fn unit<'a>(&'a mut self) -> Point3D {
        let length = self.length();
        self.scale(1.0/length)
    }

    fn clamp<'a>(&'a mut self) -> Point3D {
        let mut coord1: f64 = self.position[0];
        let mut coord2: f64 = self.position[1];
        let mut coord3: f64 = self.position[2];

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

    fn add_copy<'a>(&'a self, other: &Point3D) -> @mut Point3D {
        let coord1: f64 = self.index(0) + other.index(0);
        let coord2: f64 = self.index(1) + other.index(1);
        let coord3: f64 = self.index(2) + other.index(2);
        let new_point: @mut Point3D = @mut Point3D { position: ~[ coord1,
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

    fn sub_copy<'a>(&'a self, other: &Point3D) -> Point3D {
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

    fn mul_copy<'a>(&'a self, other: &Point3D) -> Point3D {
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

    fn div_copy<'a>(&'a self, other: &Point3D) -> Point3D {
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
        let coord2: f64 = -self.index(0) * other.index(2) + self.index(2) * other.index(0);
        let coord3: f64 = self.index(0) * other.index(1) - self.index(1) * other.index(0);
        *self.index_mutref(0) = coord1;
        *self.index_mutref(1) = coord2;
        *self.index_mutref(2) = coord3;
        self
    }

    fn xproduct_copy<'a>(&'a self, other: &Point3D) -> Point3D {
        let coord1: f64 = self.index(1) * other.index(2) - self.index(2) * other.index(1);
        let coord2: f64 = -self.index(0) * other.index(2) + self.index(2) * other.index(0);
        let coord3: f64 = self.index(0) * other.index(1) - self.index(1) * other.index(0);
        let new_point: Point3D = Point3D { position: ~[ coord1,
                                                        coord2,
                                                        coord3]};
        new_point
    }

    fn makeNegative<'a>(&'a mut self) {
        *self.index_mutref(0) = -self.index(0);
        *self.index_mutref(1) = -self.index(1);
        *self.index_mutref(2) = -self.index(2);
    }
}

////////////////////////////////
////         Ray3D          ////
////////////////////////////////
#[deriving(Clone)]
pub struct Ray3D {
    position:       ~Point3D,
    direction:      ~Point3D
}

impl Ray3D {
    fn index<'a>(&'a self, scale: f64) -> @mut Point3D{
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
#[deriving(Clone)]
pub struct Plane3D {
    normal:         ~Point3D,
    distance:       f64
}

impl Plane3D {
    //double evaluate(const Point3D& p) const;
    fn evaluate<'a>(&'a self, p: &Point3D) -> f64 {
        self.normal.dotproduct(p) + self.distance
    }
    //void makePositive(const Point3D& p);
    fn makePositive<'a>(&'a mut self, p: & Point3D) {
        if (self.evaluate(p) < 0.0) {
            self.normal.makeNegative();
            self.distance = -self.distance;
        }
    }
}

////////////////////////////////
////       RayVertex        ////
////////////////////////////////
#[deriving(Clone)]
struct RayVertex{
    index:      int,
    position:   ~Point3D,
    normal:     ~Point3D
}

////////////////////////////////
////       RayMaterial      ////
////////////////////////////////
#[deriving(Clone)]
struct RayMaterial {
    emissive:   ~Point3D,
    ambient:    ~Point3D,
    diffuse:    ~Point3D,
    specular:   ~Point3D,
    specularFallOff: f64,
}

////////////////////////////////
////   RayIntersectionInfo  ////
////////////////////////////////
#[deriving(Clone)]
struct RayIntersectionInfo {
    /*The material of the intersected surface*/
    material: RayMaterial,

    /*Position, in world coordinates, of the interstion*/
    iCoordinate:    ~Point3D,

    /*The normal of the shape at the point of the intersection*/
    normal:         ~Point3D

    /*The texture coordinates of the shape at the point of intersection*/
    //texCoordinates: Point2D
}

////////////////////////////////
////       RayCamera        ////
////////////////////////////////
#[deriving(Clone)]
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
#[deriving(Clone)]
struct RayDirectionalLight {
    direction:      ~Point3D,
    color:          ~Point3D
}

impl RayDirectionalLight {
    
    fn getDiffuse<'a>(&'a self, iInfo: & mut RayIntersectionInfo) -> Point3D{
        let mut dDotP = (self.direction.scale(-1.0)).dotproduct(iInfo.normal);
        /*
            Sphere
            Emissive 0 0 0
            Ambient  0 0 0 
            Diffuse  0.25 0.25 0.25
            Specular 1 1 1
            SFO 128
            colorobj 0.75 0.75 0.75

            Triangle
            0 0 0
            0.3 0.3 0.3
            0.1 0.1 0.5
            1 1 1
            100
            0 0 0
        */
        if (dDotP < 0.0) { 
            dDotP = 0.0;
        }
        self.color.mul_copy(iInfo.material.diffuse).scale(dDotP)
    }

    fn getSpecular<'a>(&'a self, cameraPosition: Point3D, iInfo: & mut RayIntersectionInfo) -> Point3D {
        let alpha: f64 = iInfo.material.specularFallOff;

        let r: Point3D = ((iInfo.normal.scale(
                                (self.direction.scale(-1.0).dotproduct(iInfo.normal)) * 2.0)).sub_copy(
                                &self.direction.scale(-1.0))).unit();
        let v: Point3D = cameraPosition.sub_copy(iInfo.iCoordinate).unit();
        let mut sDotP: f64 = r.dotproduct(&v);

        if (sDotP < 0.0){
            sDotP = 0.0;
        }

        self.color.mul_copy(iInfo.material.specular).scale(num::pow(sDotP, alpha))
    }

    //int isInShadow(struct RayIntersectionInfo& iInfo, class RayShape* shape, int& isectCount)
    //Point3D transparency(struct RayIntersectionInfo& iInfo, class RayShape* shape, Point3D cLimit)
}

////////////////////////////////
////        RaySphere       ////
////////////////////////////////
#[deriving(Clone)]
struct RaySphere {
    center:         ~Point3D,
    radius:         f64,
    material:       ~RayMaterial,
}

impl RaySphere {
    //double intersect(Ray3D ray, struct RayIntersectionInfo& iInfo, double mx=-1);
    fn intersect<'a>(&'a mut self, ray: Ray3D, iInfo: & mut RayIntersectionInfo, mx: f64) -> f64{
        let length:     f64 = (self.center.sub_copy(ray.position)).length();
        let check:      f64 = (self.center.sub_copy(ray.index(length))).length();

        if(check > self.radius) {
            return -1.0;
        }
        else {
            
            let dist = length - num::sqrt((self.radius * self.radius) - (check * check));

            if(dist > 0.0 && (dist < mx || mx <= 0.0)){

                iInfo.iCoordinate = ray.index(dist).copy();
                iInfo.normal = (iInfo.iCoordinate.sub_copy(self.center)).unit().copy();
		iInfo.material = *self.material.clone();
            }
            dist
        }        
    }
}

////////////////////////////////
////      RayTriangle       ////
////////////////////////////////
#[deriving(Clone)]
struct RayTriangle {
    // v1:             ~Point3D,
    // v2:             ~Point3D,
    normal:         ~Point3D,
    distance:       f64,
    vertexes:       ~[~RayVertex],
    //plane:          ~Plane3D,
    material:       ~RayMaterial,
}

impl RayTriangle {
    //int read(FILE* fp, int* materialIndex, RayVertex* vList, int vSize);
    //void write(int indent, FILE* fp=stdout);
    //double intersect(Ray3D ray, struct RayIntersectionInfo& iInfo, double mx=-1);
    fn intersect<'a>(&'a mut self, ray: Ray3D, iInfo: & mut RayIntersectionInfo, mx: f64) -> f64{
        let t: f64 = -( (ray.position.dotproduct(self.normal) + self.distance) / 
                        (ray.direction.dotproduct(self.normal)));

        if (t > 0.0 && (t < mx || mx < 0.0)){
            let point: @mut Point3D = ray.index(t);
           
            let sN0: Point3D = ((&self.vertexes[1].position.sub_copy(ray.position)).xproduct_copy(
                                     &self.vertexes[0].position.sub_copy(ray.position))).unit();
            let sN1: Point3D = ((&self.vertexes[2].position.sub_copy(ray.position)).xproduct_copy(
                                     &self.vertexes[1].position.sub_copy(ray.position))).unit();
            let sN2: Point3D = ((&self.vertexes[0].position.sub_copy(ray.position)).xproduct_copy(
                                     &self.vertexes[2].position.sub_copy(ray.position))).unit();
            let dot0: f64 = (point.sub_copy(ray.position).unit().dotproduct(&sN0));
            let dot1: f64 = (point.sub_copy(ray.position).unit().dotproduct(&sN1));
            let dot2: f64 = (point.sub_copy(ray.position).unit().dotproduct(&sN2));

            if (dot0 > 0.0 && dot1 > 0.0 && dot2 > 0.0){
                iInfo.iCoordinate = point.copy();
                iInfo.normal = self.normal.copy();
		iInfo.material = *self.material.clone();
                t
            }
            else {
                -1.0
            }
        }
        else{
            0.0
        }
    }
}


struct color 
{
	// colors range 0..1
	red: float,
	green: float,
	blue: float
}

enum Attribute {
	attr_camera,
	attr_background,
	attr_ambient,
	attr_shape_sphere,
	attr_attr_vertex_num,
	attr_vertex,
	attr_shape_triangle,
	attr_light_num,
	attr_light_dir,
	attr_materials,
	none
}

fn main() {
	//////////////////////////////////////////////////
	// USE THE RAY FILE TO GENERATE THE SCENE GRAPH //
	//////////////////////////////////////////////////

	let mut cam: ~[f64] = ~[];//attr_camera values
	let mut bg: ~[f64] = ~[];//attr_background values
	let mut amb: ~[f64] = ~[];//attr_ambient light values
	let mut ss: ~[f64] = ~[];//attr_shape_sphere values
	let mut vn: ~[f64] = ~[];//attr_vertex number values
	let mut vert: ~[f64] = ~[];//attr_vertex values
	let mut st: ~[f64] = ~[];//attr_shape_triangle values
	let mut ln: ~[f64] = ~[];//light number values
	let mut ld: ~[f64] = ~[];//light direction values
	let mut mat: ~[f64] = ~[];//attr_materials values
	let data = load_file(~"load.ray");
	let mut tag: Attribute = none;
	for i in data.iter() {
		//println(fmt!("%s", *i));
		if(i.contains("#camera")){
			//println("found attr_camera line");
			tag = attr_camera;
		} else if (i.contains("#background")){
			//println("found attr_background line");
			tag = attr_background;
		} else if (i.contains("#ambient")){
			//println("found attr_ambient line");
			tag = attr_ambient;
		} else if (i.contains("#shape_sphere")){
			//println("found attr_shape_sphere line");
			for num in i.split_iter(' ') {
					//println(num);
					//cam.push( std::float::from_str_hex(num).unwrap());
					match std::from_str::FromStr::from_str(num.trim_right()) {
      					Some(v) => {ss.push(v);
      						//println(fmt!("pushing float %f", v as float));
      					},
      					None    => {//println(fmt!("found non-float %s", num))
				}
    				}
    				
				}
			tag = attr_shape_sphere;
		} else if (i.contains("#attr_vertex_num")){
			//println("found attr_attr_vertex_num line");
			tag = attr_attr_vertex_num;
		} else if (i.contains("#vertex")){
			//println("found attr_vertex line");
			tag = attr_vertex;
		} else if (i.contains("#shape_triangle")){
			//println("found attr_shape_triangle line");
			for num in i.split_iter(' ') {
					//println(num);
					//cam.push( std::float::from_str_hex(num).unwrap());
					match std::from_str::FromStr::from_str(num.trim_right()) {
      					Some(v) => {st.push(v);
      						//println(fmt!("pushing float %f", v as float));
      					},
      					None    => {//println(fmt!("found non-float %s", num))
				}
    				}
    				
				}
			tag = attr_shape_triangle;
		} else if (i.contains("#light_num")){
			//println("found attr_light_num line");
			tag = attr_light_num;
		} else if (i.contains("#light_dir")){
			//println("found attr_light_dir line");
			tag = attr_light_dir;
		} else if (i.contains("#material")){
			//println("found attr_light_dir line");
			tag = attr_materials;
		} else if (i.contains("#material_num")){
			//println("found attr_light_dir line");
			tag = none;
		} else {
			match tag {
				attr_camera => {//println("cam");
				//split line and convert into numbers
				for num in i.split_iter(' ') {
					//println(num);
					//cam.push( std::float::from_str_hex(num).unwrap());
					match std::from_str::FromStr::from_str(num.trim_right()) {
      					Some(v) => {cam.push(v);
      						//println(fmt!("pushing float %f", v as float));
      					},
      					None    => {//println(fmt!("found non-float %s", num))
				}
    				}
    				
				}
				
				//cam = i.split_iter(' ').filter_map(std::f32::from_str).collect();

				
				}
				attr_background => {//println("bg");
				for num in i.split_iter(' ') {
					//println(num);
					//cam.push( std::float::from_str_hex(num).unwrap());
					match std::from_str::FromStr::from_str(num.trim_right()) {
      					Some(v) => {bg.push(v);
      						//println(fmt!("pushing float %f", v as float));
      					},
      					None    => {//println(fmt!("found non-float %s", num))
				}
    				}
    				
				}
				}
				attr_ambient => {//println("amb");
					for num in i.split_iter(' ') {
					//println(num);
					//cam.push( std::float::from_str_hex(num).unwrap());
					match std::from_str::FromStr::from_str(num.trim_right()) {
      					Some(v) => {amb.push(v);
      						//println(fmt!("pushing float %f", v as float));
      					},
      					None    => {//println(fmt!("found non-float %s", num))
				}
    				}
    				
				}
				}
				attr_shape_sphere => {//println("ss");
					for num in i.split_iter(' ') {
					//println(num);
					//cam.push( std::float::from_str_hex(num).unwrap());
					match std::from_str::FromStr::from_str(num.trim_right()) {
      					Some(v) => {ss.push(v);
      						//println(fmt!("pushing float %f", v as float));
      					},
      					None    => {//println(fmt!("found non-float %s", num))
				}
    				}
    				
				}
				}
				attr_attr_vertex_num => {//println("vn");
					for num in i.split_iter(' ') {
					//println(num);
					//cam.push( std::float::from_str_hex(num).unwrap());
					match std::from_str::FromStr::from_str(num.trim_right()) {
      					Some(v) => {vn.push(v);
      						//println(fmt!("pushing float %f", v as float));
      					},
      					None    => {//println(fmt!("found non-float %s", num))
				}
    				}
    				
				}
				}
				attr_vertex => {//println("vert");
					for num in i.split_iter(' ') {
					//println(num);
					//cam.push( std::float::from_str_hex(num).unwrap());
					match std::from_str::FromStr::from_str(num.trim_right()) {
      					Some(v) => {vert.push(v);
      						//println(fmt!("pushing float %f", v as float));
      					},
      					None    => {//println(fmt!("found non-float %s", num))
				}
    				}
    				
				}
				}
				attr_shape_triangle => {//println("st");
				for num in i.split_iter(' ') {
					//println(num);
					//cam.push( std::float::from_str_hex(num).unwrap());
					match std::from_str::FromStr::from_str(num.trim_right()) {
      					Some(v) => {st.push(v);
      						//println(fmt!("pushing float %f", v as float));
      					},
      					None    => {//println(fmt!("found non-float %s", num))
				}
    				}
    				
				}
				}
				attr_light_num => {//println("ln");
					for num in i.split_iter(' ') {
					//println(num);
					//cam.push( std::float::from_str_hex(num).unwrap());
					match std::from_str::FromStr::from_str(num.trim_right()) {
      					Some(v) => {ln.push(v);
      						//println(fmt!("pushing float %f", v as float));
      					},
      					None    => {//println(fmt!("found non-float %s", num))
				}
    				}
    				
				}
				}
				attr_light_dir => {//println("ld");
					for num in i.split_iter(' ') {
					//println(num);
					//cam.push( std::float::from_str_hex(num).unwrap());
					match std::from_str::FromStr::from_str(num.trim_right()) {
      					Some(v) => {ld.push(v);
      						//println(fmt!("pushing float %f", v as float));
      					},
      					None    => {//println(fmt!("found non-float %s", num))
				}
    				}
    				
				}
				}
				attr_materials => {//println("mat");
					for num in i.split_iter(' ') {
					//println(num);
					//cam.push( std::float::from_str_hex(num).unwrap());
					match std::from_str::FromStr::from_str(num.trim_right()) {
      					Some(v) => {mat.push(v);
      						//println(fmt!("pushing float %f", v as float));
      					},
      					None    => {//println(fmt!("found non-float %s", num))
				}
    				}
    				
				}
				}
				none => {//println("problems");
				}
				
			}
		}
	}

	/////////////////////////////////////////////////////
	// AND NOW BACK TO OUR REGULARLY SCHEDULED PROGRAM //
	/////////////////////////////////////////////////////

	// Camera
	let camera: RayCamera = RayCamera {
		color: ~Point3D { position: ~[0.25, 0.25, 0.25] },
		heightAngle: cam[12],
		aspectRatio: 1.0,
		position: ~Point3D { position: ~[cam[0], cam[1], cam[2]] },
		direction: ~Point3D { position: ~[cam[3], cam[4], cam[5]] },
		up: ~Point3D { position: ~[cam[6], cam[7], cam[8]] },
		right: ~Point3D { position: ~[cam[9], cam[10], cam[11]] }
	};

	/////////////////
	// Image Plane //
	/////////////////
	let image_width: f64 = 250.0;
	let image_height: f64 = 250.0;

	////////////////
	//   Scene    //
	////////////////
	let background = Point3D { position: ~[bg[0], bg[1], bg[2]] };
	let ambient = Point3D { position: ~[amb[0], amb[1], amb[2]] };
	let mut scene_triangles: ~[~RayTriangle] = ~[];
	let mut scene_spheres: ~[~RaySphere] = ~[];
	let mut scene_materials: ~[RayMaterial] = ~[];
	let mut scene_vertices: ~[RayVertex] = ~[];

	// Load materials
	let material_count = mat.len() / 13;
	let mut cur_mat_load = 0;
	let mut offset = 0;
	while(cur_mat_load < material_count) {
		scene_materials.push(RayMaterial {
			emissive: ~Point3D { position: ~[mat[offset + 0], mat[offset + 1], mat[offset + 2]] },
			ambient:  ~Point3D { position: ~[mat[offset + 3], mat[offset + 4], mat[offset + 5]] },
			diffuse:  ~Point3D { position: ~[mat[offset + 6], mat[offset + 7], mat[offset + 8]] },
			specular: ~Point3D { position: ~[mat[offset + 9], mat[offset + 10], mat[offset + 11]] },
			specularFallOff: mat[offset + 12]
		});
		cur_mat_load += 1;
		offset += 13;
	}

	// Load spheres
	let sphere_count = ss.len() / 5;
	let mut cur_sphere_load = 0;
	offset = 0;
	while(cur_sphere_load < sphere_count) {
		scene_spheres.push(~RaySphere {
			center: ~Point3D { position: ~[ss[offset + 1], ss[offset + 2], ss[offset + 3]] },
			radius: ss[offset + 4],
			material: ~scene_materials[ss[offset + 0] as int].clone()
		});
		cur_sphere_load += 1;
		offset += 5;
	}

	// Load vertices
	let vertex_count = vert.len() / 6;
	let mut cur_vertex_load = 0;
	offset = 0;
	while(cur_vertex_load < vertex_count) {
		scene_vertices.push(RayVertex {
			index: 0,
			position: ~Point3D { position: ~[vert[offset + 0], vert[offset + 1], vert[offset + 2]] },
			normal: ~Point3D { position: ~[vert[offset + 3], vert[offset + 4], vert[offset + 5]] }
		});
		cur_vertex_load += 1;
		offset += 6;
	}

	// Load triangles
	let triangle_count = st.len() / 4;
	let mut cur_triangle_load = 0;
	offset = 0;
	while(cur_triangle_load < triangle_count) {
		let t_vertices = [	scene_vertices[st[offset + 1] as int].clone().position,
					scene_vertices[st[offset + 2] as int].clone().position,
					scene_vertices[st[offset + 3] as int].clone().position ];
		let normal = (t_vertices[1].sub_copy(t_vertices[0]).xproduct_copy(&(t_vertices[2].sub_copy(t_vertices[0])))).unit();
		let distance = normal.dotproduct(&(t_vertices[1].sub_copy(t_vertices[0]).unit()));
		
		scene_triangles.push(~RayTriangle {
			normal: ~normal,
			distance: distance,
			vertexes: ~[ ~scene_vertices[st[offset + 1] as int].clone(), ~scene_vertices[st[offset + 2] as int].clone(), ~scene_vertices[st[offset + 3] as int].clone() ],
			material: ~scene_materials[st[offset] as int].clone()
		});
		cur_triangle_load += 1;
		offset += 4;
	}

	// Load light
	let dir_light: RayDirectionalLight = RayDirectionalLight {
		color: ~Point3D { position: ~[ld[0], ld[1], ld[2]] },
		direction: ~Point3D { position: ~[ld[3], ld[4], ld[5]] }	
	};
	
	///////////////////////////////////
	// Cast rays through image plane //
	///////////////////////////////////
	let mut x = image_width;
	let mut y = image_height;

    let colormap: ~[color] = ~[];
    let shared_colormap = arc::RWArc::new(colormap);
    let shared_scene_spheres = arc::Arc::new(scene_spheres);
    let shared_scene_triangles = arc::Arc::new(scene_triangles);
    let shared_camera = arc::Arc::new(camera);
    let shared_dir_light = arc::Arc::new(dir_light);
    let shared_ambient = arc::Arc::new(ambient);
    let shared_background = arc::Arc::new(background);

	// Now we iterate through the image plane
	while(y > 0.0)	{
		while(x > 0.0) {

            let task_colormap = shared_colormap.clone();
            let task_scene_spheres = shared_scene_spheres.clone();
            let task_scene_triangles = shared_scene_triangles.clone();
            let task_camera = shared_camera.clone();            
            let task_dir_light = shared_dir_light.clone();
            let task_ambient = shared_ambient.clone();
            let task_background = shared_background.clone();
            let y_temp = y.clone();
            let x_temp = x.clone();

            do spawn {

                let d = 1.0;
                
                // let theta = task_camera.get().heightAngle / 2.0;
                // let phi = (task_camera.get().aspectRatio * task_camera.get().heightAngle) / 2.0;
                let rUp = Ray3D { position: task_camera.get().position.clone(), direction: ~task_camera.get().up.clone().unit() };
                let rRight = Ray3D { position: task_camera.get().position.clone(), direction: ~task_camera.get().right.clone().unit() };
                let rForward = Ray3D { position: task_camera.get().position.clone(), direction: ~task_camera.get().direction.clone().unit() };


                let mut destX: @mut Point3D;
                let mut destY: @mut Point3D;
                let mut rayOut: Ray3D;

                // Must construct ray intersection info and temporary info for intersection calculations.
                //   Populating with blank values is necessary to keep it working as we do a lot of cloning
                //   up ahead.
                let rii = &mut RayIntersectionInfo {
                            material: RayMaterial {
                                emissive: ~Point3D { position: ~[0.0, 0.0, 0.0] },
                                ambient: ~Point3D { position: ~[0.0, 0.0, 0.0] },
                                diffuse: ~Point3D { position: ~[0.0, 0.0, 0.0] },
                                specular: ~Point3D { position: ~[0.0, 0.0, 0.0] },
                                specularFallOff: 0.0
                            },
                            iCoordinate: ~Point3D { position: ~[0.0, 0.0, 0.0] },
                            normal: ~Point3D { position: ~[0.0, 0.0, 0.0] }
                        };
                let rii_temp = &mut RayIntersectionInfo {
                            material: RayMaterial {
                                emissive: ~Point3D { position: ~[0.0, 0.0, 0.0] },
                                ambient: ~Point3D { position: ~[0.0, 0.0, 0.0] },
                                diffuse: ~Point3D { position: ~[0.0, 0.0, 0.0] },
                                specular: ~Point3D { position: ~[0.0, 0.0, 0.0] },
                                specularFallOff: 0.0
                            },
                            iCoordinate: ~Point3D { position: ~[0.0, 0.0, 0.0] },
                            normal: ~Point3D { position: ~[0.0, 0.0, 0.0] }
                        };


    			destY = rUp.index((((y_temp + 0.5) - (image_height / 2.0)) / image_height)).add_copy(&rForward.index(d).sub_copy(task_camera.get().position.clone()));
    			destX = rRight.index((((x_temp + 0.5) - (image_width / 2.0)) / image_width)).add_copy(&rForward.index(d).sub_copy(task_camera.get().position.clone()));
    			rayOut = Ray3D { position: task_camera.get().position.clone(),
    					 direction: ~destY.sub_copy(task_camera.get().position.clone()).add_copy(&destX.sub_copy(task_camera.get().position.clone())).unit() };

    			let mut int_length = -1.0;

    			// Intersect all spheres
    			for sph in task_scene_spheres.get().iter() {
    				let mut item = sph.clone();
    				let temp_intersection = item.intersect(rayOut.clone(), rii_temp, -1.0);

    				if(int_length == -1.0 || (temp_intersection < int_length && temp_intersection != -1.0)) {
    					rii.material = rii_temp.material.clone();
    					rii.iCoordinate = rii_temp.iCoordinate.clone();
    					rii.normal = rii_temp.normal.clone();
    					int_length = temp_intersection.clone();
    				}
    			}

    			// Intersect all triangles
    			for tri in task_scene_triangles.get().iter() {
    				let mut item = tri.clone();
    				let temp_intersection = item.intersect(rayOut.clone(), rii_temp, -1.0);

    				if(int_length == -1.0 || (temp_intersection < int_length && temp_intersection != -1.0)) {
    					rii.material = rii_temp.material.clone();
    					rii.iCoordinate = rii_temp.iCoordinate.clone();
    					rii.normal = rii_temp.normal.clone();
    					int_length = temp_intersection.clone();
    				}
    			}

                do task_colormap.write |colormap| {
        			if(int_length > 0.0) {
        				// Hit, get color
        				let hit_ambient = rii.material.ambient.mul_copy(task_ambient.get());
        				let hit_emissive = rii.material.emissive.mul_copy(&(Point3D { position: ~[1.0, 1.0, 1.0] }));
        				let hit_diffuse = task_dir_light.get().getDiffuse(rii);
        				let hit_specular = task_dir_light.get().getSpecular(*task_camera.get().position.clone(), rii);
        				let hit_color = hit_ambient.add_copy(&hit_emissive).add_copy(&hit_diffuse).add_copy(&hit_specular).clamp();

        				let tmp: color = color { red: hit_color.position[0] as float, blue: hit_color.position[2] as float, green: hit_color.position[1] as float };
        				colormap.push(tmp);
        			} else {
        				// No hit, render background
        				let tmp: color = color { red: task_background.get().position[0] as float, blue: task_background.get().position[1] as float, green: task_background.get().position[2] as float };
        				colormap.push(tmp);
        			}
                }
            }
			x -= 1.0;
		}
		x = image_width;
		y -= 1.0;
	}

	// reset x and y
	y = 0.0;
	x = 0.0;

	let filewriter: Result<@Writer, ~str> = io::file_writer(~path::Path("render.ppm"), &[Create]);

	match filewriter {
		Ok(writer) => {
			// output image as Netppm pixmap
			writer.write_line("P3");
			writer.write_str(fmt!("%? ",image_width as int));
			writer.write_line(fmt!("%?", image_height as int));
			writer.write_line("255");
			while y < image_height {
				while x < image_width {
                    do shared_colormap.write |colormap| {
    					let red = (colormap[(y*image_width+x) as int].red * (255 as float) );
    					let green = (colormap[(y*image_width+x) as int].green * (255 as float) );
    					let blue = (colormap[(y*image_width+x) as int].blue * (255 as float) );
    					writer.write_str(fmt!("%? ", red as int));
    					writer.write_str(fmt!("%? ", green as int));
    					writer.write_str(fmt!("%? ", blue as int));
    					x += 1.0;
                    }
				}
				writer.write_line("");
				x = 0.0;
				y += 1.0;
			}
		},
		Err(msg) => fail!("Cannot open file: " + msg)
	}
}

fn load_file(pathname : ~str) -> ~[~str] {
    let filereader : Result<@Reader, ~str> = io::file_reader(~path::Path(pathname));
    match filereader {
        Ok(reader) => reader.read_lines(),
        Err(msg) => fail!("Cannot open file: " + msg),
    }
}
