use macroquad::{
    prelude::*,
    rand::{gen_range, rand, RandomRange},
};

#[macroquad::main("3D")]

async fn main() {
    let mut vert: f32 = 0.0;
    let mut horiz: f32 = 0.0;
    let mut out_scale: f32 = 0.0;
    let xJitter = 0.4;
    let yJitter = 1.0;
    let zJitter = 0.4;

    let mut point_array_jitter = vec![];

    loop {
        clear_background(BLACK);
        if is_key_down(KeyCode::Up) {
            vert += 0.05;
        }
        if is_key_down(KeyCode::Down) {
            vert -= 0.05;
        }
        if is_key_down(KeyCode::Right) {
            horiz += 0.05;
        }
        if is_key_down(KeyCode::Left) {
            horiz -= 0.05;
        }

        if is_key_down(KeyCode::LeftShift) {
            out_scale += 0.05;
        }
        if is_key_down(KeyCode::RightShift) {
            out_scale -= 0.05;
        }

        let mut point_array = vec![];
        for i in 1..20 {
            for j in 1..20 {
                point_array.push(vec3(
                    j as f32 - 10.0,
                    0.0,
                    i as f32 + (j as f32) / 2.0 - 10.0,
                ));
            }
        }
        out_scale += 0.01;
        horiz += 0.001;
        set_camera(&Camera3D {
            position: vec3(horiz.cos() * -10., vert + 3.0, horiz.sin() * 10.0),
            up: vec3(0., 1., 0.),
            target: vec3(0., 0., 0.),
            ..Default::default()
        });

        let sincounter = (out_scale.sin() * 30.0).round() / 30.0; // / (PI)).trunc();

        if (sincounter == 0.0) {
            point_array_jitter.clear();
            for i in 1..400 {
                point_array_jitter.push(vec3(
                    gen_range(0.0, xJitter),
                    gen_range(-yJitter, yJitter),
                    gen_range(-zJitter, 0.0),
                ));
            }
        }

        for k in 1..point_array.len() {
            point_array[k] = point_array[k] - point_array_jitter[k] * 0.1
                + point_array_jitter[k] * out_scale.sin() * 0.5;
        }

        let mut line_grid = Line_Grid {
            pos: point_array.clone(),
            scale: vec3(1.0, 1.0, 1.0),
            col: vec4(1.0, 1.0, 1.0, 1.0),
        };
        let n = 1.3 + 0.001 * (out_scale.sin() * out_scale.sin()).powf(0.5) / 0.05;
        //line_grid.distance_based_line(point_array.clone(), n);
        line_grid.constant_grid(point_array, n);
        set_default_camera();
        let textcen = get_text_center(
            "Test Time :|", //"ONLY THE DEAD KNOW PEACE FROM THIS SUFFERING",
            Option::None,
            40,
            1.0,
            0.0,
        );
        draw_text(
            "Test Time :|", //"ONLY THE DEAD KNOW PEACE FROM THIS SUFFERING",
            screen_width() / 2.0 - textcen.x,
            screen_height() / 5.0 * 4.0 - textcen.y,
            40.0,
            GOLD,
        );

        next_frame().await
    }
}

struct Line_Grid {
    pos: Vec<Vec3>,
    scale: Vec3,
    col: Vec4,
}

impl Line_Grid {
    pub fn distance_based_line(&mut self, vertex_set1: Vec<Vec3>, n: f32) {
        let mut select_point = vec![];
        for i in 1..20 {
            for j in 1..20 {
                let temp = gen_range(2, 8) as usize;
                select_point.push([temp, temp, temp]);
            }
        }
        for v in vertex_set1.clone() {
            let mut temp_collection_points = vec![];
            for i in 1..vertex_set1.len() {
                if v.distance(vertex_set1[i]) < n {
                    temp_collection_points.push(vertex_set1[i]);
                    for k in 1..(temp_collection_points.len()) {
                        let temp = gen_range(1, temp_collection_points.len());
                        draw_line_3d(
                            v + vec3(0.0, 0.05, 0.0),
                            vec3(
                                temp_collection_points[temp].x,
                                temp_collection_points[temp].y,
                                temp_collection_points[temp].z,
                            ) + vec3(0.0, 0.05, 0.0),
                            BLUE,
                        );
                    }
                }
            }
            temp_collection_points.clear();
        }
    }

    pub fn constant_grid(&mut self, vertex_set1: Vec<Vec3>, n: f32) {
        for v in vertex_set1.clone() {
            for i in 1..vertex_set1.len() {
                if v.distance(vertex_set1[i]) < (n * 1.2) {
                    draw_line_3d(
                        v + vec3(0.0, 0.05, 0.0),
                        vec3(vertex_set1[i].x, vertex_set1[i].y, vertex_set1[i].z)
                            + vec3(0.0, 0.05, 0.0),
                        BLUE,
                    );
                }
            }
        }
    }
}
