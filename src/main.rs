use rand::Rng;
use raylib::{color::Color, ffi::BlendMode, math::{rrect, Vector2}, prelude::{RaylibBlendModeExt, RaylibDraw}, rgui::RaylibDrawGui};

const SCREEN_WIDTH:i32 = 800;
const SCREEN_HEIGHT:i32 = 400;

#[derive(Default, Debug)]
struct Particle{
    position: Vector2,
    color: Color,
    alpha: f32,
    size: f32,
    rotation: f32,
    active: bool
}
fn main() {
    let max_particles:usize = 200;
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Particle Blending")
        .build();
    let mut mouse_tail: Vec<Particle> = Vec::new();
    for _ in 0..max_particles{
        mouse_tail.push(Particle::default());
    }

    for pa in 0..max_particles{
        mouse_tail[pa] = Particle{
            position: Vector2{
                x: 0.0,
                y: 0.0
            },
            color: Color { 
                r: create_rand(0, 255),
                g: create_rand(0, 255), 
                b: create_rand(0, 255), 
                a: create_rand(0, 255) 
            },
            alpha: 1.0,
            size: (create_rand(1, 30) as f32) / 20.0,
            rotation: (create_rand(0, 360) as f32),
            active: false


        };
    }

    let gravity = 3.0;
    let smoke = &rl.load_texture(&thread, "./assets/spark_flame.png").expect("Could not load the image");

    

    //rl.set_target_fps(60);
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        for mut i in 0..max_particles{
            if !mouse_tail[i].active{
                mouse_tail[i].active = true;
                mouse_tail[i].alpha = 0.1;
                mouse_tail[i].position = d.get_mouse_position().into();
                i = max_particles;
            }
        }

        for i in 0..max_particles{
            if mouse_tail[i].active{
                mouse_tail[i].position.y += gravity/2.0;
                mouse_tail[i].alpha -= 0.005;
                if mouse_tail[i].alpha <= 0.0 {
                    mouse_tail[i].active = false;
                    mouse_tail[i].rotation += 2.0;
                }
            }
        }
        
        let blending = BlendMode::BLEND_ALPHA;

        let _ = d.begin_blend_mode(blending);

        for i in 0..max_particles{
            if mouse_tail[i].active{
                let tint = d.gui_fade(mouse_tail[i].color, mouse_tail[i].alpha);
                d.draw_texture_pro(
                    smoke,
                    rrect(0, 0, smoke.width, smoke.height),
                    rrect(
                        mouse_tail[i].position.x,
                        mouse_tail[i].position.y,
                        smoke.width as f32 * mouse_tail[i].size,
                        smoke.height as f32 * mouse_tail[i].size,

                        ),
                    Vector2{
                        x: smoke.width as f32 * mouse_tail[i].size / 2.0,
                        y: smoke.height as f32 * mouse_tail[i].size / 2.0
                    },
                    mouse_tail[i].rotation,
                    tint
                    );
            }
        }

        //
    }
}


fn create_rand(min:i32, max:i32) -> u8{
    let mut rand_range = rand::thread_rng();
    let rand_val = rand_range.gen_range(min..max);
    rand_val as u8
}
