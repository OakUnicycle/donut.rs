use std::time;
use std::thread;

fn main() {
    // initialise the orientation of the torus
    let mut alpha: f64 = 0.;
    let mut beta: f64 = 0.;
    loop {
        // wait for next frame
        thread::sleep(time::Duration::from_millis(16));
        //get and render frame
        let frame = render(alpha, beta);
        write(frame);
        // increment the orientation
        alpha += 0.07;
        beta += 0.03;
    }
}



const SCREEN_WIDTH: usize = 75;
const SCREEN_HEIGHT: usize = 75;
const DISTANCE: f64 = 5.;
const SCREEN_DISTANCE: f64 = 1.;
const R1: f64 = 2.;
const R2: f64 = 1.;
const LIGHT_POS: [f64; 3] = [0., 1., -1.];
const CHARACTERS: [char; 13] = ['.', ',', '-', '~', ':', ';', '=', '!', '*', '#', '$', '@', '■'];
const CHARS_LEN: usize = 12;

// for normalising vectors - making their length 1
fn normalise(x: [f64; 3]) -> [f64; 3] {
    let oom: f64 = 1. / (x[0].powi(2) + x[1].powi(2) + x[2].powi(2)).sqrt();
    let vec: [f64; 3] = [oom*x[0], oom*x[1], oom*x[2]];
    vec
}


fn render(alpha: f64, beta: f64) -> [[char; SCREEN_WIDTH]; SCREEN_HEIGHT] {
    let light_vec: [f64; 3] = normalise(LIGHT_POS); // get normalised light vector

    //initialise the output and zbuffer
    let mut output = [[' '; SCREEN_WIDTH]; SCREEN_HEIGHT];
    let mut zbuffer = [[0.; SCREEN_WIDTH]; SCREEN_HEIGHT];

    // precompute sin and cos
    let sinalpha = alpha.sin();
    let sinbeta = beta.sin();
    let cosalpha = alpha.cos();
    let cosbeta = beta.cos();

    for phi in (0..628).step_by(2) {
        // precompute sin and cos
        let phi_radians = (phi as f64) / 100.;
        let sinphi = phi_radians.sin();
        let cosphi = phi_radians.cos();

        for theta in (0..628).step_by(2) {
            // precompute sin and cos
            let theta_radians = (theta as f64) / 100.;
            let sintheta = theta_radians.sin();
            let costheta = theta_radians.cos();

            // transformation matrix of the circle 
            let mat_00 = sinalpha*sinbeta*sinphi + cosbeta*cosphi;
            let mat_01 = - cosalpha*sinbeta;
            let mat_10 = sinbeta*cosphi - sinalpha*cosbeta*sinphi;
            let mat_11 = cosalpha*cosbeta;
            let mat_20 = cosalpha*sinphi;
            let mat_21 = sinalpha;

            // vector normal to the surface
            let normal_x = R2*sintheta;
            let normal_y = R2*costheta;

            
            // position on the circle of the current point
            let circle_x = R1 + normal_x;
            let circle_y = normal_y;

            // find the position of the current point
            let x = mat_00*circle_x + mat_01*circle_y;
            let y = mat_10*circle_x + mat_11*circle_y;
            let z = mat_20*circle_x + mat_21*circle_y;
            
            // translate the point to the screen
            let xprime = (x*SCREEN_DISTANCE) / (z+DISTANCE);
            let yprime = (y*SCREEN_DISTANCE) / (z+DISTANCE);


            // translate the normal
            let x_norm = mat_00*normal_x + mat_01*normal_y;
            let y_norm = mat_10*normal_x + mat_11*normal_y;
            let z_norm = mat_20*normal_x + mat_21*normal_y;
            
            //light at given position using the dot product
            let luminance: f64 = (x_norm*light_vec[0] + y_norm*light_vec[1] + z_norm*light_vec[2]) / R2;

            if luminance > 0. { 
                // work out position in the output
                let char_x = ((xprime+1.)*(SCREEN_WIDTH as f64)/2.).clamp(0., (SCREEN_WIDTH-1) as f64) as usize; 
                let char_y = ((1.-yprime)*(SCREEN_HEIGHT as f64)/2.).clamp(0., (SCREEN_HEIGHT-1) as f64) as usize;
                // work "out one over z", which is used in the zbuffer to figure out if it is
                // closer than the character previosly placed there.
                let ooz = 1. / (z + DISTANCE);
                //if it is the closest character in that position
                if zbuffer[char_y][char_x] < ooz {
                    //figure out which character to use
                    let luminance_char: usize = (luminance * (CHARS_LEN as f64)).clamp(0., (CHARS_LEN-1) as f64) as usize;
                    //add the character to the output
                    output[char_y][char_x] = CHARACTERS[luminance_char];
                    // change the zbuffer in that position to the current character
                    zbuffer[char_y][char_x] = ooz;
                }

            }
            


        }
    }
    output
}



fn write(output: [[char; SCREEN_WIDTH]; SCREEN_HEIGHT]) {
    // clear the screen
    println!("\r\x1b[H");
    // output the donut
    for row in output {
        for character in row {
            print!("{character}");
        }
        println!("");
    }


}
