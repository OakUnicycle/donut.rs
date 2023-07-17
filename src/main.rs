use std::time;
use std::thread;

fn main() {
    let mut alpha: f64 = 0.;
    let mut beta: f64 = 0.;
    loop {
        thread::sleep(time::Duration::from_millis(16));
        let frame = render(alpha, beta);
        write(frame);
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

fn normalise(x: [f64; 3]) -> [f64; 3] {
    let oom: f64 = 1. / (x[0].powi(2) + x[1].powi(2) + x[2].powi(2)).sqrt();
    let vec: [f64; 3] = [oom*x[0], oom*x[1], oom*x[2]];
    vec
}


fn render(alpha: f64, beta: f64) -> [[char; SCREEN_WIDTH]; SCREEN_HEIGHT] {
    let light_vec: [f64; 3] = normalise(LIGHT_POS);

    let mut output = [[' '; SCREEN_WIDTH]; SCREEN_HEIGHT];
    let mut zbuffer = [[0.; SCREEN_WIDTH]; SCREEN_HEIGHT];

    let sinalpha = alpha.sin();
    let sinbeta = beta.sin();
    let cosalpha = alpha.cos();
    let cosbeta = beta.cos();

    for phi in (0..628).step_by(2) {
        let phi_radians = (phi as f64) / 100.;
        let sinphi = phi_radians.sin();
        let cosphi = phi_radians.cos();

        for theta in (0..628).step_by(2) {
            let theta_radians = (theta as f64) / 100.;
            let sintheta = theta_radians.sin();
            let costheta = theta_radians.cos();

            let mat_00 = sinalpha*sinbeta*sinphi + cosbeta*cosphi;
            let mat_01 = - cosalpha*sinbeta;
            let mat_10 = sinbeta*cosphi - sinalpha*cosbeta*sinphi;
            let mat_11 = cosalpha*cosbeta;
            let mat_20 = cosalpha*sinphi;
            let mat_21 = sinalpha;

            let normal_x = R2*sintheta;
            let normal_y = R2*costheta;


            let circle_x = R1 + normal_x;
            let circle_y = normal_y;


            let x = mat_00*circle_x + mat_01*circle_y;
            let y = mat_10*circle_x + mat_11*circle_y;
            let z = mat_20*circle_x + mat_21*circle_y;

            let xprime = (x*SCREEN_DISTANCE) / (z+DISTANCE);
            let yprime = (y*SCREEN_DISTANCE) / (z+DISTANCE);


            let x_norm = mat_00*normal_x + mat_01*normal_y;
            let y_norm = mat_10*normal_x + mat_11*normal_y;
            let z_norm = mat_20*normal_x + mat_21*normal_y;
            
            let luminance: f64 = (x_norm*light_vec[0] + y_norm*light_vec[1] + z_norm*light_vec[2]) / R2;
            if luminance > 0. { 
                let luminance_char: usize = (luminance * (CHARS_LEN as f64)).clamp(0., (CHARS_LEN-1) as f64) as usize;
                let char_x = ((xprime+1.)*(SCREEN_WIDTH as f64)/2.).clamp(0., (SCREEN_WIDTH-1) as f64) as usize;  // screenwidth/2 could be precomputed
                let char_y = ((1.-yprime)*(SCREEN_HEIGHT as f64)/2.).clamp(0., (SCREEN_HEIGHT-1) as f64) as usize;
                let ooz = 1. / (z + DISTANCE);
                if zbuffer[char_y][char_x] < ooz {
                    output[char_y][char_x] = CHARACTERS[luminance_char];
                    zbuffer[char_y][char_x] = ooz;
                }
                //output[char_x][char_y] = '.';

            }
            


        }
    }
    output
}



fn write(output: [[char; SCREEN_WIDTH]; SCREEN_HEIGHT]) {
    println!("\r\x1b[H");
    for row in output {
        for character in row {
            print!("{character}");
        }
        println!("");
    }


}
