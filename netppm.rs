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
	let width = 10;
	let height = 10;
	let mut colormap: ~[color] = ~[];
	let mut x = 0;
	let mut y = 0;

	// Make image in mem as a float vector
	while y < height {
		while x < width {
			let tmp: color = color {red: (x as float / width as float), green: (0 as float), blue: (y as float / height as float) };
			colormap.push(tmp);
			x += 1;
		}
		x = 0;
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
	while y < height {
		while x < width {
			let red = (colormap[y*width+x].red * (255 as float) );
			let green = (colormap[y*width+x].green * (255 as float) );
			let blue = (colormap[y*width+x].blue * (255 as float) );
			print(fmt!("%?", red as int));
			print(" ");
			print(fmt!("%?", green as int));
			print(" ");
			print(fmt!("%?", blue as int));
			print(" ");
			x += 1;
		}
		println("");
		x = 0;
		y += 1;
	}

}




