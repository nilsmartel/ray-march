mod scene;
use scene::*;

fn main() {
    println!("Hello, world!");
}

// start by writing the code for the cpu, porting later

struct Ray {
    pos: [f32; 2],
    z: f32,
    color: [f32; 3],
}

#[derive(Copy, Clone)]
struct Global {
    thread_x: usize,
    max_thread_x: usize,
    thread_y: usize,
    may_thread_y: usize,
}

fn render(global: Global, mut ret: &mut [Ray], scene: &Scene) {
    // using the thread_x for choosing where to cast the ray is stupid
    // huge potential in shooting ray in noisy area!
    // also norm is fucked. todo!

    let Camera {
        pos,
        direction,
        aspect_ratio,
    } = scene.camera;

    let direction = {
        // x ration
        let rx = global.max_thread_x as f32 / global.thread_x as f32;

        // solve this like a decent being using matrix multiplication
    };
}

type Mat2 = (f32, f32, f32, f32);

type Vec2 = (f32, f32);
// all matrices in row by row order
fn mat2_mul(mat: Mat2, vec: Vec2) -> Vec2 {
    (vec.0 * mat.0 + vec.1 * mat.1, vec.0 * mat.2 + vec.1 * mat.3)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn matrix_op() {
        let I = (1.0, 0.0, 0.0, 1.0);
        let v = (7.0, 23.0);
        assert_eq!(mat2_mul(I, v), v);
    }
}
