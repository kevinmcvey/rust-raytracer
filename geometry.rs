use std::ops::{Index};//,Add,Sub,Mul,Div,BitXor};
use std::num;


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
    //Point3D getDiffuse(Point3D cameraPosition, struct RayIntersectionInfo& iInfo)
    fn getDiffuse<'a>(&'a mut self, iInfo: & mut RayIntersectionInfo) -> Point3D{
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

    fn getSpecular<'a>(&'a mut self, cameraPosition: Point3D, iInfo: & mut RayIntersectionInfo) -> Point3D {
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

fn main() {
	/////////////////
	// Image Plane //
	/////////////////
	let image_width = 250.0;
	let image_height = 250.0;

	////////////////
	//   Scene    //
	////////////////
	let background = Point3D { position: ~[0.25, 0.25, 0.25] };
	let ambient = Point3D { position: ~[0.5, 0.5, 0.5] };
	let mut scene_triangles: ~[~RayTriangle] = ~[];
	let mut scene_spheres: ~[~RaySphere] = ~[];

	////////////////
	// Triangle 1 //
	////////////////
	let t1_vertices = [	~Point3D { position: ~[-2.0, 0.0, 2.0] },
				~Point3D { position: ~[2.0, 0.0, 2.0] },
				~Point3D { position: ~[2.0, 0.0, -2.0] } ];
	let normal = (t1_vertices[1].sub_copy(t1_vertices[0]).xproduct_copy(&(t1_vertices[2].sub_copy(t1_vertices[0])))).unit();
	let distance = normal.dotproduct(&(t1_vertices[1].sub_copy(t1_vertices[0]).unit()));

	let mut triangle_1 = RayTriangle {
		normal: ~normal,
		distance: distance,
		vertexes: ~[
			~RayVertex { index: 0, position: ~Point3D { position: ~[-2.0, 0.0, 2.0] }, normal: ~Point3D { position: ~[0.0, 1.0, 0.0] } },
			~RayVertex { index: 1, position: ~Point3D { position: ~[2.0, 0.0, 2.0] }, normal: ~Point3D { position: ~[0.0, 1.0, 0.0] } },
			~RayVertex { index: 2, position: ~Point3D { position: ~[2.0, 0.0, -2.0] }, normal: ~Point3D { position: ~[0.0, 1.0, 0.0] } }
			],
		material: ~RayMaterial {
			emissive: ~Point3D { position: ~[0.0, 0.0, 0.0] },
			ambient: ~Point3D { position: ~[0.3, 0.3, 0.3] },
			diffuse: ~Point3D { position: ~[0.1, 0.1, 0.5] },
			specular: ~Point3D { position: ~[1.0, 1.0, 1.0] },
			specularFallOff: 100.0
		}
	};
	
	scene_triangles.push(~triangle_1);

	////////////////
	//   Sphere   //
	////////////////
	let mut sphere: RaySphere = RaySphere {
		center: ~Point3D { position: ~[0.0, 0.0, 0.0] },
		radius: 1.0,
		material: ~RayMaterial {
			emissive: ~Point3D { position: ~[0.0, 0.0, 0.0] },
			ambient: ~Point3D { position: ~[0.0, 0.0, 0.0] },
			diffuse: ~Point3D { position: ~[0.25, 0.25, 0.25] },
			specular: ~Point3D { position: ~[0.75, 0.75, 0.75] },
			specularFallOff: 128.0
		}
	};

	scene_spheres.push(~sphere);

	////////////////
	//   Camera   //
	////////////////
	let camera: RayCamera = RayCamera {
		color: ~Point3D { position: ~[0.5, 0.5, 0.5] },
		heightAngle: 0.523,
		aspectRatio: 1.0,
		position: ~Point3D { position: ~[0.0, 10.0, 10.0] },
		direction: ~Point3D { position: ~[0.0, -1.0, -1.0] },
		up: ~Point3D { position: ~[0.0, 1.0, -1.0] },
		right: ~Point3D { position: ~[-1.0, 0.0, 0.0] }
	};

	////////////////
	//   Light    //
	////////////////
	let mut dir_light: RayDirectionalLight = RayDirectionalLight {
		direction: ~Point3D { position: ~[-1.0, -1.0, 0.0] },
		color: ~Point3D { position: ~[1.0, 1.0, 1.0] }
	};

	///////////////////////////////////
	// Cast rays through image plane //
	///////////////////////////////////
	let mut x = image_width;
	let mut y = image_height;
	let d = 1.0;
	let theta = camera.heightAngle / 2.0;
	let phi = (camera.aspectRatio * camera.heightAngle) / 2.0;
	let rUp = Ray3D { position: camera.position.clone(), direction: ~camera.up.clone().unit() };
	let rRight = Ray3D { position: camera.position.clone(), direction: ~camera.right.clone().unit() };
	let rForward = Ray3D { position: camera.position.clone(), direction: ~camera.direction.clone().unit() };

	let mut destX: @mut Point3D;
	let mut destY: @mut Point3D;
	let mut rayOut: Ray3D;

	// Must construct ray intersection info and temporary info for intersection calculations.
	//   Populating with blank values is necessary to keep it working as we do a lot of cloning
	//   up ahead.
	let mut rii = &mut RayIntersectionInfo {
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
	let mut rii_temp = &mut RayIntersectionInfo {
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

	let mut int_length = 0.0;
	let mut colormap: ~[color] = ~[];

	// Now we iterate through the image plane
	while(y > 0.0)	{
		while(x > 0.0) {
			destY = rUp.index((((y + 0.5) - (image_height / 2.0)) / image_height)).add_copy(&rForward.index(d).sub_copy(camera.position.clone()));
			destX = rRight.index((((x + 0.5) - (image_width / 2.0)) / image_width)).add_copy(&rForward.index(d).sub_copy(camera.position.clone()));
			rayOut = Ray3D { position: camera.position.clone(),
					 direction: ~destY.sub_copy(camera.position.clone()).add_copy(&destX.sub_copy(camera.position.clone())).unit() };

			int_length = -1.0;

			// Intersect all spheres
			for sph in scene_spheres.iter() {
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
			for tri in scene_triangles.iter() {
				let mut item = tri.clone();
				let temp_intersection = item.intersect(rayOut.clone(), rii_temp, -1.0);

				if(int_length == -1.0 || (temp_intersection < int_length && temp_intersection != -1.0)) {
					rii.material = rii_temp.material.clone();
					rii.iCoordinate = rii_temp.iCoordinate.clone();
					rii.normal = rii_temp.normal.clone();
					int_length = temp_intersection.clone();
				}
			}

			if(int_length > 0.0) {
				// Hit, get color
				let hit_ambient = rii.material.ambient.mul_copy(&ambient);
				let hit_emissive = rii.material.emissive.mul_copy(&(Point3D { position: ~[1.0, 1.0, 1.0] }));
				let hit_diffuse = dir_light.getDiffuse(rii);
				let hit_specular = dir_light.getSpecular(*camera.position.clone(), rii);
				let hit_color = hit_ambient.add_copy(&hit_emissive).add_copy(&hit_diffuse).add_copy(&hit_specular).clamp();

				let tmp: color = color { red: hit_color.position[0] as float, blue: hit_color.position[2] as float, green: hit_color.position[1] as float };
				colormap.push(tmp);
			} else {
				// No hit, render background
				let tmp: color = color { red: background.position[0] as float, blue: background.position[1] as float, green: background.position[2] as float };
				colormap.push(tmp);
			}

			x -= 1.0;
		}
		x = image_width;
		y -= 1.0;
	}

	// reset x and y
	y = 0.0;
	x = 0.0;
	// output image as Netppm pixmap
	println("P3");
	print(fmt!("%?",image_width as int));
	print(" ");
	println(fmt!("%?", image_height as int));
	println("255");
	while y < image_height {
		while x < image_width {
			let red = (colormap[(y*image_width+x) as int].red * (255 as float) );
			let green = (colormap[(y*image_width+x) as int].green * (255 as float) );
			let blue = (colormap[(y*image_width+x) as int].blue * (255 as float) );
			print(fmt!("%?", red as int));
			print(" ");
			print(fmt!("%?", green as int));
			print(" ");
			print(fmt!("%?", blue as int));
			print(" ");
			x += 1.0;
		}
		println("");
		x = 0.0;
		y += 1.0;
	}
}
