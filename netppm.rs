// translated from code here: http://stackoverflow.com/a/3898919

extern mod std;

struct color 
{
	// colors range 0..1
	red: float,
	green: float,
	blue: float
} 

fn main() 
{
	let width = 3;
	let height = 2;
	let mut colormap: ~[color] = ~[];
	let mut x = 0;
	let mut y = 0;

	// Make image in mem as a float vector
	// for y in range (0, height){
	// 	for x in range (0, width){
	// 		let tmp: color = color {red: (x / width), green: 0, blue: (y / height) };
	// 		colormap.push(tmp);
	// 	}
	// }
	while y < height {
		while x < width {
			let tmp: color = color {red: ((x / width) as float), green: (0 as float), blue: ((y / height) as float) };
			colormap.push(tmp);
			x += 1;
		}
		y += 1;
	}

	// reset x and y
	y = 0;
	x = 0;
	// output image as Netppm pixmap
	println("P3");
	print(fmt!("%?",width));
	print(" ");
	println(fmt!("%?", height));
	println("255");
	// for y in range (0, height){
	// 	for x in range(0, width){
	// 		let red = (colormap[y*width+x].red * 255);
	// 		let green = (colormap[y*width+x].green * 255);
	// 		let blue = (colormap[y*width+x].blue * 255);
	// 		print(fmt!("%i", red));
	// 		print(" ");
	// 		print(fmt!("%i", green));
	// 		print(" ");
	// 		print(fmt!("%i", blue));
	// 		print(" ");
	// 	}
	// 	println("");
	// }
	while y < height {
		while x < width {
			let red = (colormap[y*width+x].red * (255 as float) );
			let green = (colormap[y*width+x].green * (255 as float) );
			let blue = (colormap[y*width+x].blue * (255 as float) );
			print(fmt!("%?", red));
			print(" ");
			print(fmt!("%?", green));
			print(" ");
			print(fmt!("%?", blue));
			print(" ");
			x += 1;
		}
		println("");
		y += 1;
	}

}




